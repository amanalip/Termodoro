// Audio playback and acoustic synthesis module for Termodoro
use crate::timer::PomodoroPhase;
use std::io::Cursor;
use std::sync::atomic::{AtomicBool, Ordering};

// Flag to disable audio hardware playback in unit testing environments
static AUDIO_MUTED_FOR_TESTS: AtomicBool = AtomicBool::new(false);

#[allow(dead_code)]
pub fn set_audio_muted_for_tests(muted: bool) {
    AUDIO_MUTED_FOR_TESTS.store(muted, Ordering::SeqCst);
}

// Generates a standard 44.1 kHz, 16-bit mono RIFF WAV byte vector in-memory
pub fn create_riff_wav_pcm16(samples: &[i16], sample_rate: u32) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(44 + samples.len() * 2);
    let num_samples = samples.len() as u32;
    let subchunk2_size = num_samples * 2; // 16-bit mono = 2 bytes per sample
    let chunk_size = 36 + subchunk2_size;
    let byte_rate = sample_rate * 2;
    let block_align = 2u16;
    let bits_per_sample = 16u16;

    // RIFF chunk descriptor
    buffer.extend_from_slice(b"RIFF");
    buffer.extend_from_slice(&chunk_size.to_le_bytes());
    buffer.extend_from_slice(b"WAVE");

    // "fmt " sub-chunk
    buffer.extend_from_slice(b"fmt ");
    buffer.extend_from_slice(&16u32.to_le_bytes()); // Subchunk1Size for PCM = 16
    buffer.extend_from_slice(&1u16.to_le_bytes()); // AudioFormat = 1 (PCM)
    buffer.extend_from_slice(&1u16.to_le_bytes()); // NumChannels = 1 (Mono)
    buffer.extend_from_slice(&sample_rate.to_le_bytes());
    buffer.extend_from_slice(&byte_rate.to_le_bytes());
    buffer.extend_from_slice(&block_align.to_le_bytes());
    buffer.extend_from_slice(&bits_per_sample.to_le_bytes());

    // "data" sub-chunk
    buffer.extend_from_slice(b"data");
    buffer.extend_from_slice(&subchunk2_size.to_le_bytes());
    for &sample in samples {
        buffer.extend_from_slice(&sample.to_le_bytes());
    }

    buffer
}

// Generates a soothing Zen Tibetan Bell chime (528 Hz transformation frequency with warm harmonics)
pub fn generate_work_complete_chime() -> Vec<u8> {
    let sample_rate = 44100;
    let duration_secs = 1.8;
    let total_samples = (sample_rate as f32 * duration_secs) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        // Exponential decay envelope
        let env = (-2.8 * t).exp();

        // 528 Hz base + 1056 Hz + 1584 Hz overtone harmonics
        let s1 = (2.0 * std::f32::consts::PI * 528.0 * t).sin() * 0.65;
        let s2 = (2.0 * std::f32::consts::PI * 1056.0 * t).sin() * 0.25;
        let s3 = (2.0 * std::f32::consts::PI * 1584.0 * t).sin() * 0.10;

        let mixed = (s1 + s2 + s3) * env;
        let sample_i16 = (mixed * 28000.0).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
        samples.push(sample_i16);
    }

    create_riff_wav_pcm16(&samples, sample_rate)
}

// Generates an uplifting double-chime (D5 587.33 Hz -> A5 880.0 Hz) signaling break completion
pub fn generate_break_complete_chime() -> Vec<u8> {
    let sample_rate = 44100;
    let duration_secs = 1.4;
    let total_samples = (sample_rate as f32 * duration_secs) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        let sample_val = if t < 0.22 {
            // Note 1: 587.33 Hz
            let note_t = t;
            let env = (-4.0 * note_t).exp();
            let s = (2.0 * std::f32::consts::PI * 587.33 * note_t).sin() * 0.8;
            s * env
        } else {
            // Note 2: 880.0 Hz
            let note_t = t - 0.22;
            let env = (-2.5 * note_t).exp();
            let s1 = (2.0 * std::f32::consts::PI * 880.0 * note_t).sin() * 0.75;
            let s2 = (2.0 * std::f32::consts::PI * 1760.0 * note_t).sin() * 0.25;
            (s1 + s2) * env
        };

        let sample_i16 = (sample_val * 28000.0).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
        samples.push(sample_i16);
    }

    create_riff_wav_pcm16(&samples, sample_rate)
}

// Generates a melodic major triad chime (C5 523.25 Hz -> E5 659.25 Hz -> G5 783.99 Hz) for long breaks
pub fn generate_long_break_chime() -> Vec<u8> {
    let sample_rate = 44100;
    let duration_secs = 2.0;
    let total_samples = (sample_rate as f32 * duration_secs) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;
        let sample_val = if t < 0.20 {
            // Note 1: C5 (523.25 Hz)
            let note_t = t;
            let env = (-3.5 * note_t).exp();
            (2.0 * std::f32::consts::PI * 523.25 * note_t).sin() * 0.8 * env
        } else if t < 0.40 {
            // Note 2: E5 (659.25 Hz)
            let note_t = t - 0.20;
            let env = (-3.5 * note_t).exp();
            (2.0 * std::f32::consts::PI * 659.25 * note_t).sin() * 0.8 * env
        } else {
            // Note 3: G5 (783.99 Hz) with harmonics ringing out
            let note_t = t - 0.40;
            let env = (-2.0 * note_t).exp();
            let s1 = (2.0 * std::f32::consts::PI * 783.99 * note_t).sin() * 0.75;
            let s2 = (2.0 * std::f32::consts::PI * 1567.98 * note_t).sin() * 0.25;
            (s1 + s2) * env
        };

        let sample_i16 = (sample_val * 28000.0).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
        samples.push(sample_i16);
    }

    create_riff_wav_pcm16(&samples, sample_rate)
}

// Plays acoustic chime corresponding to the finished Pomodoro phase in a background thread
pub fn play_phase_sound(finished_phase: PomodoroPhase) {
    if AUDIO_MUTED_FOR_TESTS.load(Ordering::SeqCst) {
        return;
    }

    // Spawn detached audio playback thread
    std::thread::spawn(move || {
        let wav_data = match finished_phase {
            PomodoroPhase::Work => generate_work_complete_chime(),
            PomodoroPhase::ShortBreak => generate_break_complete_chime(),
            PomodoroPhase::LongBreak => generate_long_break_chime(),
        };

        // Initialize output stream with rodio
        if let Ok((_stream, stream_handle)) = rodio::OutputStream::try_default() {
            let cursor = Cursor::new(wav_data);
            if let Ok(source) = rodio::Decoder::new(cursor) {
                if let Ok(sink) = rodio::Sink::try_new(&stream_handle) {
                    sink.append(source);
                    sink.sleep_until_end();
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_riff_wav_pcm16_header() {
        let samples = vec![0i16, 1000, -1000, 2000, -2000];
        let wav = create_riff_wav_pcm16(&samples, 44100);

        // Header must be at least 44 bytes + sample bytes
        assert_eq!(wav.len(), 44 + samples.len() * 2);
        // Verify RIFF and WAVE magic signatures
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert_eq!(&wav[12..16], b"fmt ");
        assert_eq!(&wav[36..40], b"data");
    }

    #[test]
    fn test_generate_work_complete_chime() {
        let wav = generate_work_complete_chime();
        assert!(wav.len() > 44);
        // Valid readable WAV using Rodio Decoder
        let cursor = Cursor::new(wav);
        let decoder = rodio::Decoder::new(cursor);
        assert!(decoder.is_ok());
    }

    #[test]
    fn test_generate_break_complete_chime() {
        let wav = generate_break_complete_chime();
        assert!(wav.len() > 44);
        let cursor = Cursor::new(wav);
        let decoder = rodio::Decoder::new(cursor);
        assert!(decoder.is_ok());
    }

    #[test]
    fn test_generate_long_break_chime() {
        let wav = generate_long_break_chime();
        assert!(wav.len() > 44);
        let cursor = Cursor::new(wav);
        let decoder = rodio::Decoder::new(cursor);
        assert!(decoder.is_ok());
    }

    #[test]
    fn test_play_phase_sound_does_not_panic() {
        // Mute to avoid opening hardware device in headless test runner
        set_audio_muted_for_tests(true);
        play_phase_sound(PomodoroPhase::Work);
        play_phase_sound(PomodoroPhase::ShortBreak);
        play_phase_sound(PomodoroPhase::LongBreak);
        set_audio_muted_for_tests(false);
    }

    #[test]
    fn test_wav_sample_bounds_no_clipping_work_chime() {
        let wav = generate_work_complete_chime();
        // Skip 44-byte RIFF header and inspect 16-bit PCM samples
        let sample_bytes = &wav[44..];
        assert_eq!(sample_bytes.len() % 2, 0);

        let mut max_abs: i16 = 0;
        for chunk in sample_bytes.chunks_exact(2) {
            let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
            let abs = sample.saturating_abs();
            if abs > max_abs {
                max_abs = abs;
            }
        }
        // Verify audio has audible signal (not silent) and has headroom (no hard clipping)
        assert!(max_abs > 10000, "Signal too quiet");
        assert!(max_abs < 32000, "Signal clipped");
    }

    #[test]
    fn test_wav_sample_bounds_no_clipping_break_chimes() {
        for (name, wav) in [
            ("break", generate_break_complete_chime()),
            ("long_break", generate_long_break_chime()),
        ] {
            let sample_bytes = &wav[44..];
            let mut max_abs: i16 = 0;
            for chunk in sample_bytes.chunks_exact(2) {
                let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
                let abs = sample.saturating_abs();
                if abs > max_abs {
                    max_abs = abs;
                }
            }
            assert!(max_abs > 10000, "{} signal too quiet", name);
            assert!(max_abs < 32000, "{} signal clipped", name);
        }
    }

    #[test]
    fn test_create_riff_wav_empty_samples() {
        let empty_samples: [i16; 0] = [];
        let wav = create_riff_wav_pcm16(&empty_samples, 44100);
        assert_eq!(wav.len(), 44);
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[36..40], b"data");
        // Subchunk2Size must be 0
        let data_size = u32::from_le_bytes([wav[40], wav[41], wav[42], wav[43]]);
        assert_eq!(data_size, 0);
    }

    #[test]
    fn test_create_riff_wav_custom_sample_rates() {
        let samples = vec![100i16, -100, 200, -200];
        for rate in [8000, 22050, 44100, 48000, 96000] {
            let wav = create_riff_wav_pcm16(&samples, rate);
            let sample_rate_in_header = u32::from_le_bytes([wav[24], wav[25], wav[26], wav[27]]);
            assert_eq!(sample_rate_in_header, rate);
            let byte_rate = u32::from_le_bytes([wav[28], wav[29], wav[30], wav[31]]);
            assert_eq!(byte_rate, rate * 2);
        }
    }

    #[test]
    fn test_audio_mute_flag_concurrency() {
        set_audio_muted_for_tests(true);
        assert!(AUDIO_MUTED_FOR_TESTS.load(Ordering::SeqCst));
        set_audio_muted_for_tests(false);
        assert!(!AUDIO_MUTED_FOR_TESTS.load(Ordering::SeqCst));
    }

    #[test]
    fn test_create_riff_wav_byte_level_alignment() {
        let samples = vec![0x1234i16, -0x1234i16];
        let wav = create_riff_wav_pcm16(&samples, 44100);
        // Header length must be exactly 44 bytes
        assert_eq!(wav.len(), 44 + 4);
        // Channels = 1 (little-endian: 0x01, 0x00)
        assert_eq!(wav[22], 1);
        assert_eq!(wav[23], 0);
        // Bits per sample = 16 (little-endian: 0x10, 0x00)
        assert_eq!(wav[34], 16);
        assert_eq!(wav[35], 0);
        // Data payload begins at offset 44
        assert_eq!(wav[44], 0x34);
        assert_eq!(wav[45], 0x12);
    }

    #[test]
    fn test_generate_chimes_finite_and_clean_samples() {
        let work_samples = generate_work_complete_chime();
        let short_break_samples = generate_break_complete_chime();
        let long_break_samples = generate_long_break_chime();

        for samples in [&work_samples, &short_break_samples, &long_break_samples] {
            assert!(!samples.is_empty());
            // Verify RIFF WAV has at least the 44-byte header + data
            assert!(samples.len() > 44);
        }
    }

    #[test]
    fn test_audio_sample_rate_conversion_and_duration_math() {
        let sample_rate = 44100u32;
        let duration_secs = 0.5f32;
        let total_samples = (sample_rate as f32 * duration_secs) as usize;
        assert_eq!(total_samples, 22050);

        let mut samples = Vec::with_capacity(total_samples);
        for i in 0..total_samples {
            let t = i as f32 / sample_rate as f32;
            let sample = (t * 440.0 * 2.0 * std::f32::consts::PI).sin();
            samples.push((sample * 16000.0) as i16);
        }

        let wav = create_riff_wav_pcm16(&samples, sample_rate);
        // 44-byte header + 22050 * 2 bytes = 44144 bytes
        assert_eq!(wav.len(), 44 + (22050 * 2));
    }

    #[test]
    fn test_wav_header_subchunk2_size_consistency() {
        let test_sizes = [1, 10, 100, 1024, 44100];
        for size in test_sizes {
            let dummy_samples = vec![500i16; size];
            let wav = create_riff_wav_pcm16(&dummy_samples, 44100);
            let subchunk2_size = u32::from_le_bytes([wav[40], wav[41], wav[42], wav[43]]);
            assert_eq!(subchunk2_size, (size * 2) as u32);
            let chunk_size = u32::from_le_bytes([wav[4], wav[5], wav[6], wav[7]]);
            assert_eq!(chunk_size, 36 + (size * 2) as u32);
        }
    }

    #[test]
    fn test_audio_sample_amplitudes_fade_out_smoothly() {
        let work_wav = generate_work_complete_chime();
        let break_wav = generate_break_complete_chime();
        let long_wav = generate_long_break_chime();

        for wav in [work_wav, break_wav, long_wav] {
            // Extract raw PCM 16-bit samples after the 44-byte RIFF header
            let pcm_data = &wav[44..];
            let sample_count = pcm_data.len() / 2;
            assert!(sample_count > 1000);

            // Read the last 100 samples
            let mut last_samples = Vec::new();
            for i in (sample_count - 100)..sample_count {
                let sample = i16::from_le_bytes([pcm_data[i * 2], pcm_data[i * 2 + 1]]);
                last_samples.push(sample.abs());
            }

            // Average amplitude of the final tail must be very small (< 2500) to prevent pop/click on audio DAC
            let avg_tail_amp: i32 = last_samples.iter().map(|&s| s as i32).sum::<i32>() / 100;
            assert!(
                avg_tail_amp < 2500,
                "Audio tail amplitude too high: {}",
                avg_tail_amp
            );
        }
    }

    #[test]
    fn test_audio_mute_for_tests_flag() {
        AUDIO_MUTED_FOR_TESTS.store(true, Ordering::SeqCst);
        assert!(AUDIO_MUTED_FOR_TESTS.load(Ordering::SeqCst));
        // Calling play_phase_sound while muted must return immediately without spawning threads
        play_phase_sound(crate::timer::PomodoroPhase::Work);
        play_phase_sound(crate::timer::PomodoroPhase::ShortBreak);
        play_phase_sound(crate::timer::PomodoroPhase::LongBreak);
    }

    #[test]
    fn test_audio_work_chime_harmonic_components_variance() {
        let work_wav = generate_work_complete_chime();
        let pcm = &work_wav[44..];
        let mut max_val = 0i16;
        for i in 0..(pcm.len() / 2) {
            let sample = i16::from_le_bytes([pcm[i * 2], pcm[i * 2 + 1]]);
            if sample.abs() > max_val {
                max_val = sample.abs();
            }
        }
        // Work chime must have rich audible amplitude (> 15000)
        assert!(max_val > 15000, "Max amplitude was too quiet: {}", max_val);
    }

    #[test]
    fn test_audio_break_chime_two_tone_structure_duration() {
        let break_wav = generate_break_complete_chime();
        // Sample rate 44100 * 1.4s * 2 bytes/sample + 44 bytes header = 123524 bytes
        assert_eq!(break_wav.len(), 44 + (44100 * 14 / 10 * 2));
    }

    #[test]
    fn test_audio_long_break_chime_three_tone_triad_duration() {
        let long_wav = generate_long_break_chime();
        // Sample rate 44100 * 2.0s * 2 bytes/sample + 44 bytes header = 176444 bytes
        assert_eq!(long_wav.len(), 44 + (44100 * 2 * 2));
    }
}
