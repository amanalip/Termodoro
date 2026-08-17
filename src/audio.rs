// Audio playback and acoustic synthesis module for Termodoro
use std::io::Cursor;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::timer::PomodoroPhase;

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
    buffer.extend_from_slice(&1u16.to_le_bytes());  // AudioFormat = 1 (PCM)
    buffer.extend_from_slice(&1u16.to_le_bytes());  // NumChannels = 1 (Mono)
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
}
