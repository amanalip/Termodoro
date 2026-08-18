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

// Import Config struct from our config module
use crate::config::Config;
// Import StatsHistory from our stats module
use crate::stats::StatsHistory;
// Import TaskManager from our tasks module
use crate::tasks::TaskManager;

// Unified struct representing all serializable application state saved to disk
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppData {
    // Persisted user preferences and theme
    pub config: Config,
    // Persisted task list and active target
    pub tasks: TaskManager,
    // Persisted session history and analytics
    pub stats: StatsHistory,
}

// Storage manager responsible for locating and reading/writing state files
pub struct Storage {
    // Optional custom path override (useful for testing or custom portable configs)
    custom_path: Option<PathBuf>,
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
    fn get_data_dir() -> PathBuf {
        // Attempt to resolve ProjectDirs for termodoro
        if let Some(proj_dirs) = ProjectDirs::from("com", "termodoro", "termodoro") {
            // Get data directory path
            let data_dir = proj_dirs.data_dir();
            // Create data directory if it doesn't already exist
            let _ = create_dir_all(data_dir);
            // Return owned PathBuf
            data_dir.to_path_buf()
        } else {
            // Fallback to current working directory if system dirs unavailable
            PathBuf::from(".")
        }
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
    pub fn load(&self) -> AppData {
        // Get storage file path
        let path = self.data_file_path();
        // Check if file exists on disk
        if path.exists() {
            // Attempt to open file
            if let Ok(mut file) = File::open(&path) {
                // Buffer to hold file string contents
                let mut content = String::new();
                // Read entire file to string
                if file.read_to_string(&mut content).is_ok() {
                    // Attempt to parse JSON into AppData struct
                    if let Ok(data) = serde_json::from_str::<AppData>(&content) {
                        // Return loaded data
                        return data;
                    }
                }
            }
        }

        // Return fresh default state if file does not exist or parsing fails
        AppData {
            // Default configuration
            config: Config::default(),
            // Empty task manager
            tasks: TaskManager::new(),
            // Empty stats history
            stats: StatsHistory::new(),
        }
    }

    // Saves current application state (config, tasks, stats) into JSON file on disk
    pub fn save(&self, config: &Config, tasks: &TaskManager, stats: &StatsHistory) {
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
        if let Ok(json) = serde_json::to_string_pretty(&data) {
            // Ensure parent directory exists before writing
            if let Some(parent) = path.parent() {
                // Create parent directories if missing
                let _ = create_dir_all(parent);
            }
            // Create or truncate file
            if let Ok(mut file) = File::create(&path) {
                // Write JSON bytes to file
                let _ = file.write_all(json.as_bytes());
            }
        }
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

        storage.save(&config, &tasks, &stats);
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
        storage.save(&config, &tasks, &stats);
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

        let mut config = Config::default();
        config.theme = crate::theme::ThemeChoice::OledPhosphor;
        config.work_duration_mins = 50;
        config.short_break_mins = 10;
        config.long_break_mins = 30;
        config.long_break_interval = 6;
        config.sound_enabled = false;
        config.desktop_notifications = true;

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

        storage.save(&config, &tasks, &stats);
        assert!(file_path.exists());

        let loaded = storage.load();
        assert_eq!(loaded.config.theme, crate::theme::ThemeChoice::OledPhosphor);
        assert_eq!(loaded.config.work_duration_mins, 50);
        assert_eq!(loaded.tasks.tasks.len(), 20);
        assert_eq!(loaded.stats.sessions.len(), 20);
        assert_eq!(loaded.stats.total_focus_minutes(), 1000);

        let _ = fs::remove_dir_all(temp_dir);
    }
}
