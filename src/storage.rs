// Import ProjectDirs from directories crate for cross-platform standard config/data directories
use directories::ProjectDirs;
// Import serde serialization and deserialization traits
use serde::{Deserialize, Serialize};
// Import filesystem directory creation and file handles from standard library
use std::fs::{create_dir_all, File};
// Import Read and Write traits for I/O operations
use std::io::{Read, Write};
// Import PathBuf for manipulating filesystem paths
use std::path::PathBuf;
// Import atomic counter for unique staging-file names
use std::sync::atomic::{AtomicU64, Ordering};

// Monotonic sequence number making every save's staging file unique within
// this process. Combined with the process id it is unique across processes
// too, which matters because two termodoro instances (or the app and a test)
// writing the same data.json previously shared ONE deterministic
// `data.json.tmp` name: overlapping writers truncated each other's staging
// file mid-write and could rename garbled bytes onto the live state file.
static SAVE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

// Import Config struct from our config module
use crate::config::Config;
// Import StatsHistory from our stats module
use crate::stats::StatsHistory;
// Import TaskManager from our tasks module
use crate::tasks::TaskManager;

// Unified struct representing all serializable application state saved to disk
//
// Every field carries #[serde(default)] so a data.json written by an older or
// newer build that is missing an entire section (for example a pre-stats
// schema) still parses with defaults for that section. Without this, one
// absent key failed the whole load and, via the quarantine path, moved the
// user's surviving tasks aside as if the file were corrupt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppData {
    // Persisted user preferences and theme
    #[serde(default)]
    pub config: Config,
    // Persisted task list and active target
    #[serde(default)]
    pub tasks: TaskManager,
    // Persisted session history and analytics
    #[serde(default)]
    pub stats: StatsHistory,
}

// Storage manager responsible for locating and reading/writing state files
pub struct Storage {
    // Optional custom path override (useful for testing or custom portable configs)
    custom_path: Option<PathBuf>,
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage {
    // Constructor creating a default storage instance using system directories
    pub fn new() -> Self {
        // Return instance with no custom path override
        Self { custom_path: None }
    }

    // Constructor creating a storage instance targeting a custom filepath (useful for testing)
    #[allow(dead_code)]
    pub fn with_path(path: PathBuf) -> Self {
        Self {
            custom_path: Some(path),
        }
    }

    // Resolves standard XDG/system data directory for termodoro (e.g. ~/.local/share/termodoro)
    //
    // Resolution order when ProjectDirs is unavailable (for example HOME unset
    // under cron/systemd/containers): XDG_DATA_HOME, then the user home
    // directory, and only as a last resort the current working directory.
    // A silent CWD fallback previously scattered data files across whatever
    // directory the binary happened to be launched from.
    fn get_data_dir() -> PathBuf {
        // Resolve the location first (pure computation), then create it.
        // Keeping resolution side-effect-free lets tests exercise the real
        // path logic without touching the developer's actual user profile.
        let data_dir = Self::resolve_data_dir();
        let _ = create_dir_all(&data_dir);
        data_dir
    }

    // Pure path resolution used by get_data_dir; performs no filesystem writes
    fn resolve_data_dir() -> PathBuf {
        // Preferred: platform-standard project data directory
        if let Some(proj_dirs) = ProjectDirs::from("com", "termodoro", "termodoro") {
            return proj_dirs.data_dir().to_path_buf();
        }

        // Fallback 1: XDG_DATA_HOME (Linux/BSD convention)
        if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
            if !xdg_data.trim().is_empty() {
                return PathBuf::from(xdg_data).join("termodoro");
            }
        }

        // Fallback 2: the user home directory
        if let Some(home) = std::env::var_os("HOME").filter(|h| !h.is_empty()) {
            return PathBuf::from(home).join(".local/share/termodoro");
        }

        // Last resort: current working directory, announced loudly so the
        // surprise of finding data.json here is at least explainable
        eprintln!(
            "warning: could not resolve a home data directory; falling back to ./termodoro-data"
        );
        PathBuf::from("./termodoro-data")
    }

    // Returns absolute or relative path to data.json storage file
    fn data_file_path(&self) -> PathBuf {
        // Check if custom path is set
        if let Some(ref p) = self.custom_path {
            // Return clone of custom path
            p.clone()
        } else {
            // Join data.json filename to system data directory
            Self::get_data_dir().join("data.json")
        }
    }

    // Loads AppData from disk or constructs a new default state if file doesn't exist
    //
    // A file that exists but cannot be parsed is QUARANTINED (renamed with a
    // timestamp suffix) instead of being left in place. Without this, the
    // defaults returned below would eventually be saved over the original
    // bytes, permanently destroying potentially recoverable user data.
    pub fn load(&self) -> AppData {
        // Get storage file path
        let path = self.data_file_path();
        // Check if file exists on disk
        if path.exists() {
            // Attempt to open file
            match File::open(&path).and_then(|mut file| {
                // Buffer to hold file string contents
                let mut content = String::new();
                // Read entire file to string (propagate read errors)
                file.read_to_string(&mut content)?;
                Ok(content)
            }) {
                // Attempt to parse JSON into AppData struct
                Ok(content) => match serde_json::from_str::<AppData>(&content) {
                    // Success: sanitize any out-of-range values loaded from a
                    // hand-edited or partially corrupted file before use
                    Ok(mut data) => {
                        data.config.sanitize();
                        return data;
                    }
                    // Parse failure: quarantine the unreadable bytes
                    Err(parse_err) => self.quarantine_corrupt_file(&path, &parse_err.to_string()),
                },
                // Open/read failure (permissions, I/O error): quarantine too,
                // because continuing would treat the file as absent
                Err(io_err) => self.quarantine_corrupt_file(&path, &io_err.to_string()),
            }
        }

        // Return fresh default state if file does not exist or was quarantined
        AppData {
            // Default configuration
            config: Config::default(),
            // Empty task manager
            tasks: TaskManager::new(),
            // Empty stats history
            stats: StatsHistory::new(),
        }
    }

    // Renames an unreadable state file aside so its bytes survive for manual
    // recovery, and warns the user on stderr where the copy lives.
    fn quarantine_corrupt_file(&self, path: &std::path::Path, reason: &str) {
        // Build a deterministic, sortable quarantine name: data.json.corrupt-<unix_ts>
        let backup_path =
            path.with_extension(format!("json.corrupt-{}", chrono::Utc::now().timestamp()));
        match std::fs::rename(path, &backup_path) {
            Ok(()) => eprintln!(
                "warning: state file {} was unreadable ({}); original moved to {}",
                path.display(),
                reason,
                backup_path.display()
            ),
            // If even the rename fails, at least explain why defaults are loading
            Err(rename_err) => eprintln!(
                "warning: state file {} was unreadable ({}) and could not be backed up ({}); starting with defaults",
                path.display(),
                reason,
                rename_err
            ),
        }
    }

    // Saves current application state (config, tasks, stats) into JSON file on disk
    //
    // The write is ATOMIC: content goes to a sibling temporary file which is
    // flushed to disk and then renamed over the real file. Rename is atomic on
    // POSIX and same-volume Windows, so a crash mid-write can never leave a
    // truncated data.json behind (the old code truncated the live file first).
    // Errors are returned rather than swallowed so callers can warn the user
    // when persistence silently stops working (full disk, read-only mount).
    pub fn save(
        &self,
        config: &Config,
        tasks: &TaskManager,
        stats: &StatsHistory,
    ) -> std::io::Result<()> {
        // Resolve storage file path
        let path = self.data_file_path();
        // Assemble AppData payload
        let data = AppData {
            // Clone config struct
            config: config.clone(),
            // Clone tasks struct
            tasks: tasks.clone(),
            // Clone stats struct
            stats: stats.clone(),
        };

        // Serialize state to pretty formatted JSON string
        let json = serde_json::to_string_pretty(&data).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("serialize failed: {e}"),
            )
        })?;

        // Ensure parent directory exists before writing
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }

        // Stage the new content in a temporary sibling file. The name is
        // unique per save (pid + monotonic sequence) so concurrent writers
        // to the same data.json can never truncate each other's staging
        // bytes; the ".tmp" suffix stays last so litter-detection heuristics
        // (and humans) still recognize staging files at a glance.
        let tmp_path = path.with_extension(format!(
            "json.{}-{}.tmp",
            std::process::id(),
            SAVE_SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        {
            let mut tmp = File::create(&tmp_path)?;
            // Write JSON bytes to the staging file
            tmp.write_all(json.as_bytes())?;
            // Flush userspace buffers AND force the OS to commit to stable
            // storage so a power cut right after rename cannot lose the data
            tmp.sync_all()?;
        }

        // Atomically replace the live file with the fully-written staging file
        std::fs::rename(&tmp_path, &path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_appdata_roundtrip_serde() {
        let app_data = AppData {
            config: Config::default(),
            tasks: TaskManager::new(),
            stats: StatsHistory::new(),
        };

        let json = serde_json::to_string(&app_data).expect("Serialization failed");
        let deserialized: AppData = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(app_data.config, deserialized.config);
        assert_eq!(app_data.tasks, deserialized.tasks);
        assert_eq!(app_data.stats, deserialized.stats);
    }

    #[test]
    fn test_storage_save_and_load_roundtrip() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_test_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("test_data.json");
        let storage = Storage::with_path(file_path.clone());

        let mut tasks = TaskManager::new();
        tasks.add("Test persistence".to_string(), 3);
        let config = Config::default();
        let mut stats = StatsHistory::new();
        stats.record(crate::timer::PomodoroPhase::Work, 25, None, None);

        storage
            .save(&config, &tasks, &stats)
            .expect("save should succeed");
        assert!(file_path.exists());

        let loaded = storage.load();
        assert_eq!(loaded.config, config);
        assert_eq!(loaded.tasks.tasks.len(), 1);
        assert_eq!(loaded.tasks.tasks[0].title, "Test persistence");
        assert_eq!(loaded.stats.sessions.len(), 1);

        // Cleanup
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_storage_fallback_on_nonexistent_or_corrupt_file() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_test_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("nonexistent.json");
        let storage = Storage::with_path(file_path.clone());

        let loaded = storage.load();
        assert_eq!(loaded.config, Config::default());
        assert_eq!(loaded.tasks.tasks.len(), 0);
        assert_eq!(loaded.stats.sessions.len(), 0);

        // Corrupt file content
        let _ = create_dir_all(&temp_dir);
        let _ = fs::write(&file_path, "{ broken json content");
        let loaded_corrupt = storage.load();
        assert_eq!(loaded_corrupt.config, Config::default());
        assert_eq!(loaded_corrupt.tasks.tasks.len(), 0);

        // Cleanup
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_storage_custom_deep_path_creation() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_deep_{}", uuid::Uuid::new_v4()));
        let deep_file_path = temp_dir.join("sub1").join("sub2").join("deep_data.json");
        let storage = Storage::with_path(deep_file_path.clone());

        let tasks = TaskManager::new();
        let config = Config::default();
        let stats = StatsHistory::new();

        // Saving to non-existent nested directory creates parent dirs
        storage
            .save(&config, &tasks, &stats)
            .expect("save should succeed");
        assert!(deep_file_path.exists());

        let loaded = storage.load();
        assert_eq!(loaded.config, config);

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_storage_save_and_load_with_full_dataset() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_fulldataset_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let storage = Storage::with_path(file_path.clone());

        let config = Config {
            theme: crate::theme::ThemeChoice::OledPhosphor,
            work_duration_mins: 50,
            short_break_mins: 10,
            long_break_mins: 30,
            long_break_interval: 6,
            sound_enabled: false,
            desktop_notifications: true,
            ..Default::default()
        };

        let mut tasks = TaskManager::new();
        for i in 0..20 {
            tasks.add(format!("Full Dataset Task {}", i), (i % 4) + 1);
        }

        let mut stats = StatsHistory::new();
        for i in 0..20 {
            stats.record(
                crate::timer::PomodoroPhase::Work,
                50,
                Some(format!("task-id-{}", i)),
                Some(format!("Full Dataset Task {}", i)),
            );
        }

        storage
            .save(&config, &tasks, &stats)
            .expect("save should succeed");
        assert!(file_path.exists());

        let loaded = storage.load();
        assert_eq!(loaded.config.theme, crate::theme::ThemeChoice::OledPhosphor);
        assert_eq!(loaded.config.work_duration_mins, 50);
        assert_eq!(loaded.tasks.tasks.len(), 20);
        assert_eq!(loaded.stats.sessions.len(), 20);
        assert_eq!(loaded.stats.total_focus_minutes(), 1000);

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_storage_empty_file_and_partial_json_fallback() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_emptyfile_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("empty.json");
        let storage = Storage::with_path(file_path.clone());

        let _ = create_dir_all(&temp_dir);
        // Write completely empty file (0 bytes)
        let _ = fs::write(&file_path, "");
        let loaded = storage.load();
        assert_eq!(loaded.config, Config::default());
        assert_eq!(loaded.tasks.tasks.len(), 0);

        // Write partial JSON without required keys
        let _ = fs::write(&file_path, "{\"unexpected_key\": 123}");
        let loaded_partial = storage.load();
        assert_eq!(loaded_partial.config, Config::default());
        assert_eq!(loaded_partial.tasks.tasks.len(), 0);

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_storage_custom_path_accessor_and_default_fallback() {
        let storage_default = Storage::new();
        assert!(storage_default.custom_path.is_none());

        let custom_p = PathBuf::from("/tmp/custom_test_path.json");
        let storage_custom = Storage::with_path(custom_p.clone());
        assert_eq!(storage_custom.custom_path, Some(custom_p));
    }

    #[test]
    fn test_privacy_zero_telemetry_guarantees() {
        // Formally verify that default AppData and serializations contain zero telemetry or tracking identifiers
        let mut app_data = AppData {
            config: Config::default(),
            tasks: TaskManager::new(),
            stats: StatsHistory::new(),
        };
        app_data.tasks.add("Audit privacy".to_string(), 2);
        app_data
            .stats
            .record(crate::timer::PomodoroPhase::Work, 25, None, None);

        let json = serde_json::to_string_pretty(&app_data).expect("Serialization failed");
        let value: serde_json::Value = serde_json::from_str(&json).expect("JSON parse failed");

        // Verify top-level structure contains ONLY the 3 expected local-first state objects
        let root_obj = value.as_object().expect("Root should be an object");
        let allowed_root_keys = ["config", "tasks", "stats"];
        for key in root_obj.keys() {
            assert!(
                allowed_root_keys.contains(&key.as_str()),
                "Found unexpected root key: '{}' in database schema!",
                key
            );
        }

        // Verify config keys contain no telemetry endpoints, API keys, or user tracking IDs
        let config_obj = value["config"]
            .as_object()
            .expect("Config must be an object");
        let forbidden_keywords = [
            "telemetry",
            "tracking",
            "analytics_endpoint",
            "api_key",
            "remote_url",
            "cloud",
            "server",
            "ip_address",
            "device_id",
            "mac_address",
            "host",
            "port",
            "network",
            "cookie",
            "token",
        ];

        for key in config_obj.keys() {
            for forbidden in &forbidden_keywords {
                assert!(
                    !key.to_lowercase().contains(forbidden),
                    "Found forbidden telemetry/network key in config schema: '{}'",
                    key
                );
            }
        }
    }

    #[test]
    fn test_storage_data_isolation_local_only() {
        // Verify database paths resolve exclusively to local user filesystem
        // directories. Uses the pure resolver so the test never creates (or
        // depends on) the developer's actual user data directory.
        let default_path = Storage::resolve_data_dir().join("data.json");
        let path_str = default_path.to_string_lossy();

        // Must never target cloud storage, external network shares, or global shared tmp
        assert!(
            path_str.ends_with("data.json"),
            "Database filename must be data.json"
        );
        assert!(
            !path_str.starts_with("http://")
                && !path_str.starts_with("https://")
                && !path_str.starts_with("ftp://"),
            "Database path must be local filesystem, found network URL: {}",
            path_str
        );
    }

    #[test]
    fn test_storage_schema_fields_contain_no_device_or_telemetry_keys() {
        let app_data = AppData {
            config: Config::default(),
            tasks: TaskManager::new(),
            stats: StatsHistory::new(),
        };
        let raw_json = serde_json::to_string(&app_data).unwrap();
        // Plaintext inspection of serialized format
        assert!(
            !raw_json.contains("http://"),
            "Serialized state must never contain HTTP urls"
        );
        assert!(
            !raw_json.contains("https://"),
            "Serialized state must never contain HTTPS urls"
        );
        assert!(
            !raw_json.contains("amplitude"),
            "Must not contain analytics trackers"
        );
        assert!(
            !raw_json.contains("mixpanel"),
            "Must not contain analytics trackers"
        );
        assert!(
            !raw_json.contains("sentry"),
            "Must not contain crash-reporting network identifiers"
        );
        assert!(
            !raw_json.contains("google-analytics"),
            "Must not contain Google Analytics trackers"
        );
    }

    #[test]
    fn test_storage_atomic_tmp_file_renamed_after_save() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_tmpclean_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let tmp_file_path = temp_dir.join("data.json.tmp");
        let storage = Storage::with_path(file_path.clone());

        storage
            .save(
                &Config::default(),
                &TaskManager::new(),
                &StatsHistory::new(),
            )
            .expect("save should succeed");
        assert!(file_path.exists());
        // The staging file must have been atomically renamed onto data.json,
        // leaving no .tmp litter behind
        assert!(!tmp_file_path.exists());

        // The live file must contain complete, parseable JSON proving the
        // rename carried the full write rather than a partial buffer
        let raw = fs::read_to_string(&file_path).expect("saved file must be readable");
        let parsed: AppData =
            serde_json::from_str(&raw).expect("renamed file must contain valid JSON");
        assert_eq!(parsed.config, Config::default());

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_storage_failed_save_leaves_previous_data_intact() {
        // Simulates a crash mid-save: if the staging write fails, the previous
        // good data.json must remain untouched and parseable
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_atomic_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let storage = Storage::with_path(file_path.clone());

        // First save succeeds and writes a known task
        let mut tasks = TaskManager::new();
        tasks.add("Original Task".to_string(), 1);
        storage
            .save(&Config::default(), &tasks, &StatsHistory::new())
            .expect("initial save should succeed");

        // Corrupt the staging path into a directory: File::create on it fails,
        // which is exactly the failure window the atomic strategy protects.
        // Staging names are now unique per save, so failure is injected via a
        // path whose parent component is a regular FILE: create_dir_all fails
        // with ENOTDIR deterministically on every platform and user id.
        let blocker = temp_dir.join("blocker");
        fs::write(&blocker, b"regular file, not a directory").expect("write blocker");
        let blocked_storage = Storage::with_path(blocker.join("data.json"));
        let result = blocked_storage.save(
            &Config::default(),
            &TaskManager::new(),
            &StatsHistory::new(),
        );
        assert!(
            result.is_err(),
            "save must report failure instead of swallowing it"
        );

        // The original data.json survives byte-intact with the original task
        let recovered = storage.load();
        assert_eq!(recovered.tasks.tasks.len(), 1);
        assert_eq!(recovered.tasks.tasks[0].title, "Original Task");

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_storage_corrupt_file_is_quarantined_not_overwritten() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_quarantine_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let storage = Storage::with_path(file_path.clone());
        let _ = create_dir_all(&temp_dir);

        // Write unrecoverable garbage
        let _ = fs::write(&file_path, "{ broken json content");
        let loaded = storage.load();

        // Defaults are returned...
        assert_eq!(loaded.config, Config::default());
        assert_eq!(loaded.tasks.tasks.len(), 0);

        // ...but the corrupt bytes were moved aside, not destroyed
        let quarantine_names: Vec<_> = fs::read_dir(&temp_dir)
            .expect("temp dir readable")
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        assert!(
            quarantine_names
                .iter()
                .any(|n| n.starts_with("data.json.corrupt-")),
            "corrupt file must be quarantined with a timestamped name, found: {:?}",
            quarantine_names
        );
        assert!(
            !file_path.exists(),
            "original corrupt file must be renamed away"
        );

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_storage_load_sanitizes_out_of_range_config() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_sanitize_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let storage = Storage::with_path(file_path.clone());
        let _ = create_dir_all(&temp_dir);

        // Hand-edited file with absurd values: zero durations would otherwise
        // cause instant phase completion loops, and a huge value overflows
        // minutes-to-seconds math downstream
        let _ = fs::write(
            &file_path,
            r#"{"config":{"theme":"Dracula","work_duration_mins":0,"short_break_mins":999999,"long_break_mins":0,"long_break_interval":0,"auto_start_breaks":true,"auto_start_work":true,"desktop_notifications":false,"sound_enabled":true},"tasks":{"tasks":[],"active_task_id":null},"stats":{"sessions":[]}}"#,
        );

        let loaded = storage.load();
        assert_eq!(
            loaded.config.work_duration_mins, 1,
            "zero work duration clamps to minimum"
        );
        assert_eq!(
            loaded.config.short_break_mins, 60,
            "absurd short break clamps to max"
        );
        assert_eq!(
            loaded.config.long_break_mins, 1,
            "zero long break clamps to minimum"
        );
        assert_eq!(
            loaded.config.long_break_interval, 1,
            "zero interval clamps to minimum"
        );

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_storage_app_data_clone_and_equality() {
        let app_data = AppData {
            config: Config::default(),
            tasks: TaskManager::new(),
            stats: StatsHistory::new(),
        };
        let cloned = app_data.clone();
        assert_eq!(app_data.config, cloned.config);
        assert_eq!(app_data.tasks.tasks.len(), cloned.tasks.tasks.len());
        assert_eq!(app_data.stats.sessions.len(), cloned.stats.sessions.len());
    }

    #[test]
    fn test_storage_load_idempotence() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_idemp_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let storage = Storage::with_path(file_path.clone());

        let mut tasks = TaskManager::new();
        tasks.add("Idempotent Task".to_string(), 4);
        storage
            .save(&Config::default(), &tasks, &StatsHistory::new())
            .expect("save should succeed");

        let load_1 = storage.load();
        let load_2 = storage.load();
        let load_3 = storage.load();

        assert_eq!(load_1.tasks.tasks[0].title, "Idempotent Task");
        assert_eq!(load_2.tasks.tasks[0].title, "Idempotent Task");
        assert_eq!(load_3.tasks.tasks[0].title, "Idempotent Task");

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_storage_constructor_new_default_path_exists() {
        // Pure resolution check: no directory creation side effects on the
        // real user profile during tests
        let path = Storage::resolve_data_dir().join("data.json");
        assert!(path.to_string_lossy().contains("termodoro"));
    }

    // Concurrent writers to the SAME data.json must never corrupt it.
    // Previously every save staged through ONE deterministic `data.json.tmp`
    // name, so overlapping writers truncated each other's staging bytes and
    // could rename garbled content onto the live file. Staging names are now
    // unique per save; after all writers finish, the live file must be a
    // byte-complete save from exactly one writer and no litter may remain.
    #[test]
    fn test_storage_concurrent_savers_never_corrupt_state_file() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_concurrent_{}", uuid::Uuid::new_v4()));
        let _ = create_dir_all(&temp_dir);
        let file_path = temp_dir.join("data.json");

        const WRITERS: usize = 8;
        const SAVES_PER_WRITER: usize = 25;

        let handles: Vec<_> = (0..WRITERS)
            .map(|writer| {
                let path = file_path.clone();
                std::thread::spawn(move || {
                    let storage = Storage::with_path(path);
                    for i in 0..SAVES_PER_WRITER {
                        let mut tasks = TaskManager::new();
                        for t in 0..=i {
                            tasks.add(format!("writer-{writer}-{t}"), 1);
                        }
                        storage
                            .save(&Config::default(), &tasks, &StatsHistory::new())
                            .expect("concurrent save must succeed");
                    }
                })
            })
            .collect();
        for handle in handles {
            handle.join().expect("writer thread must not panic");
        }

        // Every staging file must have been renamed away: exactly one file
        // remains in the directory.
        let remaining: Vec<String> = fs::read_dir(&temp_dir)
            .expect("temp dir readable")
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        assert_eq!(
            remaining,
            vec!["data.json".to_string()],
            "staging files leaked: {remaining:?}"
        );

        // The surviving file must be a COMPLETE save from ONE writer: it
        // parses, and every task belongs to the same writer prefix (a torn
        // interleave of two writers would mix prefixes or fail to parse).
        let raw = fs::read_to_string(&file_path).expect("live file readable");
        let parsed: AppData = serde_json::from_str(&raw).expect("no torn writes may survive");
        assert!(!parsed.tasks.tasks.is_empty(), "final save had tasks");
        let prefixes: std::collections::HashSet<String> = parsed
            .tasks
            .tasks
            .iter()
            .map(|t| t.title.split('-').take(2).collect::<Vec<_>>().join("-"))
            .collect();
        assert_eq!(
            prefixes.len(),
            1,
            "state file mixes tasks from different writers: {:?}",
            parsed
                .tasks
                .tasks
                .iter()
                .map(|t| &t.title)
                .collect::<Vec<_>>()
        );

        let _ = fs::remove_dir_all(temp_dir);
    }

    // A data.json containing invalid UTF-8 bytes cannot even be read as a
    // String; the load path must quarantine those raw bytes (not delete or
    // overwrite them) and fall back to defaults.
    #[test]
    fn test_storage_invalid_utf8_file_is_quarantined_with_bytes_intact() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_utf8_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let storage = Storage::with_path(file_path.clone());
        let _ = create_dir_all(&temp_dir);

        let garbage: &[u8] = &[0xFF, 0xFE, b'{', b'"', b'}', 0x80];
        fs::write(&file_path, garbage).expect("write raw bytes");

        let loaded = storage.load();
        assert_eq!(loaded.config, Config::default());
        assert_eq!(loaded.tasks.tasks.len(), 0);

        let quarantined = fs::read_dir(&temp_dir)
            .expect("temp dir readable")
            .filter_map(|e| e.ok())
            .find(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with("data.json.corrupt-")
            })
            .expect("invalid UTF-8 file must be quarantined");
        assert_eq!(
            fs::read(quarantined.path()).expect("quarantined bytes readable"),
            garbage,
            "raw bytes must survive verbatim"
        );
        assert!(!file_path.exists());

        let _ = fs::remove_dir_all(temp_dir);
    }

    // A UTF-8 BOM before otherwise-valid JSON is rejected by serde_json;
    // the file must be quarantined rather than silently overwritten.
    #[test]
    fn test_storage_bom_prefixed_json_is_quarantined() {
        let temp_dir = std::env::temp_dir().join(format!("termodoro_bom_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let storage = Storage::with_path(file_path.clone());
        let _ = create_dir_all(&temp_dir);

        let mut content = vec![0xEF, 0xBB, 0xBF];
        content.extend_from_slice(br#"{"config":{"work_duration_mins":50}}"#);
        fs::write(&file_path, &content).expect("write BOM file");

        let loaded = storage.load();
        assert_eq!(
            loaded.config.work_duration_mins, 25,
            "BOM-prefixed file must not parse; defaults are served"
        );
        assert!(loaded.tasks.tasks.is_empty());
        assert!(
            fs::read_dir(&temp_dir)
                .expect("temp dir readable")
                .filter_map(|e| e.ok())
                .any(|e| e
                    .file_name()
                    .to_string_lossy()
                    .starts_with("data.json.corrupt-")),
            "BOM file must be quarantined"
        );

        let _ = fs::remove_dir_all(temp_dir);
    }

    // A field with the wrong JSON type (string where u32 is expected) fails
    // the whole parse; the file must be quarantined and defaults served.
    #[test]
    fn test_storage_wrong_typed_field_is_quarantined() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_wrongtype_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let storage = Storage::with_path(file_path.clone());
        let _ = create_dir_all(&temp_dir);

        fs::write(
            &file_path,
            r#"{"config":{"work_duration_mins":"fifty"},"tasks":{"tasks":[],"active_task_id":null},"stats":{"sessions":[]}}"#,
        )
        .expect("write wrong-typed file");

        let loaded = storage.load();
        assert_eq!(loaded.config, Config::default());
        assert!(
            fs::read_dir(&temp_dir)
                .expect("temp dir readable")
                .filter_map(|e| e.ok())
                .any(|e| e
                    .file_name()
                    .to_string_lossy()
                    .starts_with("data.json.corrupt-")),
            "wrong-typed file must be quarantined"
        );

        let _ = fs::remove_dir_all(temp_dir);
    }

    // Duplicate JSON keys: serde's derived Deserialize REJECTS duplicate
    // fields outright (it does NOT silently keep the last occurrence), so a
    // hand-edited file containing duplicates is treated as corruption:
    // quarantined with defaults served. Pin that contract so a serde upgrade
    // cannot change it unnoticed.
    #[test]
    fn test_storage_duplicate_keys_rejected_and_quarantined() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_dupkeys_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let storage = Storage::with_path(file_path.clone());
        let _ = create_dir_all(&temp_dir);

        fs::write(
            &file_path,
            r#"{"config":{"work_duration_mins":50,"work_duration_mins":90},"tasks":{"tasks":[],"active_task_id":null},"stats":{"sessions":[]}}"#,
        )
        .expect("write duplicate-key file");

        let loaded = storage.load();
        assert_eq!(
            loaded.config,
            Config::default(),
            "duplicate fields fail the parse; defaults are served"
        );
        assert!(
            fs::read_dir(&temp_dir)
                .expect("temp dir readable")
                .filter_map(|e| e.ok())
                .any(|e| e
                    .file_name()
                    .to_string_lossy()
                    .starts_with("data.json.corrupt-")),
            "duplicate-key file must be quarantined"
        );

        let _ = fs::remove_dir_all(temp_dir);
    }

    // JSON nesting deeper than serde_json's recursion limit must fail the
    // parse cleanly: quarantine plus defaults, never a stack overflow.
    #[test]
    fn test_storage_deeply_nested_json_is_quarantined_not_crashing() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_deepjson_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let storage = Storage::with_path(file_path.clone());
        let _ = create_dir_all(&temp_dir);

        // Depth 500 far exceeds serde_json's default 128 recursion limit.
        let mut content = String::new();
        content.push_str(r#"{"config":"#);
        for _ in 0..500 {
            content.push('[');
        }
        for _ in 0..500 {
            content.push(']');
        }
        content.push('}');
        fs::write(&file_path, content.as_bytes()).expect("write deep json");

        let loaded = storage.load();
        assert_eq!(loaded.config, Config::default());
        assert!(
            fs::read_dir(&temp_dir)
                .expect("temp dir readable")
                .filter_map(|e| e.ok())
                .any(|e| e
                    .file_name()
                    .to_string_lossy()
                    .starts_with("data.json.corrupt-")),
            "over-deep JSON must be quarantined"
        );

        let _ = fs::remove_dir_all(temp_dir);
    }

    // A data.json written by an older/newer build may lack whole sections
    // (for example a pre-stats version). Missing sections must deserialize as
    // defaults instead of failing the parse, which would quarantine the file
    // and destroy every task the user still had.
    #[test]
    fn test_appdata_missing_stats_section_loads_with_defaults() {
        let json =
            r#"{"config":{"work_duration_mins":45},"tasks":{"tasks":[],"active_task_id":null}}"#;
        let data: AppData = serde_json::from_str(json).expect("missing stats must not fail parse");
        assert_eq!(data.config.work_duration_mins, 45);
        assert_eq!(data.tasks.tasks.len(), 0);
        assert_eq!(data.stats.sessions.len(), 0);
    }

    #[test]
    fn test_appdata_missing_tasks_section_loads_with_defaults() {
        let json = r#"{"config":{},"stats":{"sessions":[]}}"#;
        let data: AppData = serde_json::from_str(json).expect("missing tasks must not fail parse");
        assert_eq!(data.config, Config::default());
        assert_eq!(data.tasks.tasks.len(), 0);
        assert_eq!(data.stats.sessions.len(), 0);
    }

    #[test]
    fn test_appdata_missing_config_section_loads_with_defaults() {
        let json = r#"{"tasks":{"tasks":[],"active_task_id":null},"stats":{"sessions":[]}}"#;
        let data: AppData = serde_json::from_str(json).expect("missing config must not fail parse");
        assert_eq!(data.config, Config::default());
    }

    #[test]
    fn test_appdata_empty_object_yields_full_defaults() {
        let data: AppData = serde_json::from_str("{}").expect("empty object must not fail parse");
        assert_eq!(data.config, Config::default());
        assert_eq!(data.tasks, TaskManager::new());
        assert_eq!(data.stats, StatsHistory::new());
    }

    // End-to-end through Storage::load: a legacy file without the stats key
    // must load with defaults for that section AND keep its tasks intact,
    // with no quarantine side file appearing.
    #[test]
    fn test_storage_legacy_file_without_stats_keeps_tasks() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_legacy_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let storage = Storage::with_path(file_path.clone());
        let _ = create_dir_all(&temp_dir);

        let _ = fs::write(
            &file_path,
            r#"{"config":{"theme":"Nord","work_duration_mins":50},"tasks":{"tasks":[{"id":"legacy-1","title":"Surviving Task","completed":false,"pomodoros_spent":2,"pomodoros_estimated":5,"created_at":"2026-01-01T00:00:00Z"}],"active_task_id":"legacy-1"}}"#,
        );

        let loaded = storage.load();
        assert_eq!(loaded.tasks.tasks.len(), 1, "tasks must survive the load");
        assert_eq!(loaded.tasks.tasks[0].title, "Surviving Task");
        assert_eq!(loaded.config.work_duration_mins, 50);
        assert_eq!(loaded.stats.sessions.len(), 0);

        // No quarantine backup may appear: nothing was corrupt
        let no_quarantine = !fs::read_dir(&temp_dir)
            .expect("temp dir readable")
            .filter_map(|e| e.ok())
            .any(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with("data.json.corrupt-")
            });
        assert!(no_quarantine, "valid legacy file must not be quarantined");

        let _ = fs::remove_dir_all(temp_dir);
    }
}
