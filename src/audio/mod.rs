//! Audio system for emulator

use crate::{EmuliteResult, EmuliteError, config::AudioConfig as ConfigAudioConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, SampleFormat, SampleRate, Stream, StreamConfig};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

/// Audio configuration
#[derive(Debug, Clone)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub buffer_size: usize,
    pub volume: f32,
    pub enabled: bool,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            buffer_size: 1024,
            volume: 0.8,
            enabled: true,
        }
    }
}

/// Audio sample types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SampleType {
    U8,
    I16,
    I32,
    F32,
}

/// Audio channel information
#[derive(Debug, Clone)]
pub struct AudioChannel {
    pub frequency: f32,
    pub volume: f32,
    pub waveform: Waveform,
    pub enabled: bool,
    pub duty_cycle: f32, // For square waves
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Waveform {
    Square,
    Triangle,
    Sawtooth,
    Sine,
    Noise,
}

/// Audio system for handling sound output
pub struct AudioSystem {
    config: AudioConfig,
    host: Host,
    device: Option<Device>,
    stream: Option<Stream>,
    audio_buffer: Arc<Mutex<VecDeque<f32>>>,
    channels: Vec<AudioChannel>,
    sample_clock: f32,
}

impl AudioSystem {
    pub fn new(config: &ConfigAudioConfig) -> EmuliteResult<Self> {
        let audio_config = AudioConfig {
            sample_rate: config.sample_rate,
            channels: config.channels as u16,
            buffer_size: config.buffer_size,
            volume: config.volume,
            enabled: config.enabled,
        };
        
        Self::new_with_audio_config(&audio_config)
    }
    
    pub fn new_with_audio_config(config: &AudioConfig) -> EmuliteResult<Self> {
        let host = cpal::default_host();
        let device = host.default_output_device();
        
        let mut audio_system = Self {
            config: config.clone(),
            host,
            device,
            stream: None,
            audio_buffer: Arc::new(Mutex::new(VecDeque::new())),
            channels: Vec::new(),
            sample_clock: 0.0,
        };
        
        // Initialize audio channels (typical for retro consoles)
        audio_system.initialize_channels()?;
        
        // Try to start audio stream, but don't fail if it doesn't work
        if let Err(e) = audio_system.start_stream() {
            log::warn!("Failed to start audio stream: {}. Continuing without audio.", e);
            // Disable audio but continue
            audio_system.config.enabled = false;
        }
        
        Ok(audio_system)
    }
    
    fn initialize_channels(&mut self) -> EmuliteResult<()> {
        // Initialize channels based on platform
        // This is a generic setup - specific platforms will override
        self.channels = vec![
            AudioChannel {
                frequency: 440.0,
                volume: 0.0,
                waveform: Waveform::Square,
                enabled: false,
                duty_cycle: 0.5,
            },
            AudioChannel {
                frequency: 440.0,
                volume: 0.0,
                waveform: Waveform::Triangle,
                enabled: false,
                duty_cycle: 0.5,
            },
            AudioChannel {
                frequency: 440.0,
                volume: 0.0,
                waveform: Waveform::Noise,
                enabled: false,
                duty_cycle: 0.5,
            },
            AudioChannel {
                frequency: 440.0,
                volume: 0.0,
                waveform: Waveform::Square,
                enabled: false,
                duty_cycle: 0.5,
            },
        ];
        
        Ok(())
    }
    
    fn start_stream(&mut self) -> EmuliteResult<()> {
        let device = self.device.as_ref()
            .ok_or_else(|| EmuliteError::AudioError("No audio device available".to_string()))?;
        
        let config = device.default_output_config()
            .map_err(|e| EmuliteError::AudioError(format!("Failed to get default config: {}", e)))?;
        
        // Use default configuration instead of custom one for better compatibility
        let stream_config = StreamConfig {
            channels: config.channels(),
            sample_rate: config.sample_rate(),
            buffer_size: cpal::BufferSize::Default,
        };
        
        let audio_buffer = self.audio_buffer.clone();
        let channels = self.channels.clone();
        let sample_rate = self.config.sample_rate as f32;
        
        let stream = match config.sample_format() {
            SampleFormat::F32 => {
                device.build_output_stream(
                    &stream_config,
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        Self::audio_callback_f32(data, &audio_buffer, &channels, sample_rate);
                    },
                    move |err| {
                        eprintln!("Audio stream error: {}", err);
                    },
                    None,
                )?
            },
            SampleFormat::I16 => {
                device.build_output_stream(
                    &stream_config,
                    move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                        Self::audio_callback_i16(data, &audio_buffer, &channels, sample_rate);
                    },
                    move |err| {
                        eprintln!("Audio stream error: {}", err);
                    },
                    None,
                )?
            },
            SampleFormat::U8 => {
                device.build_output_stream(
                    &stream_config,
                    move |data: &mut [u8], _: &cpal::OutputCallbackInfo| {
                        Self::audio_callback_u8(data, &audio_buffer, &channels, sample_rate);
                    },
                    move |err| {
                        eprintln!("Audio stream error: {}", err);
                    },
                    None,
                )?
            },
            _ => return Err(EmuliteError::AudioError("Unsupported sample format".to_string())),
        };
        
        stream.play()
            .map_err(|e| EmuliteError::AudioError(format!("Failed to start stream: {}", e)))?;
        
        self.stream = Some(stream);
        Ok(())
    }
    
    fn audio_callback_f32(
        data: &mut [f32],
        audio_buffer: &Arc<Mutex<VecDeque<f32>>>,
        channels: &[AudioChannel],
        sample_rate: f32,
    ) {
        let mut buffer = audio_buffer.lock().unwrap();
        
        for sample in data.iter_mut() {
            let mut output = 0.0;
            
            // Mix all channels
            for channel in channels {
                if channel.enabled {
                    let channel_sample = Self::generate_sample(channel, sample_rate);
                    output += channel_sample * channel.volume;
                }
            }
            
            // Apply master volume and clipping
            output = (output * 0.8).clamp(-1.0, 1.0);
            
            *sample = output;
        }
    }
    
    fn audio_callback_i16(
        data: &mut [i16],
        audio_buffer: &Arc<Mutex<VecDeque<f32>>>,
        channels: &[AudioChannel],
        sample_rate: f32,
    ) {
        let mut buffer = audio_buffer.lock().unwrap();
        
        for sample in data.iter_mut() {
            let mut output = 0.0;
            
            for channel in channels {
                if channel.enabled {
                    let channel_sample = Self::generate_sample(channel, sample_rate);
                    output += channel_sample * channel.volume;
                }
            }
            
            output = (output * 0.8).clamp(-1.0, 1.0);
            *sample = (output * i16::MAX as f32) as i16;
        }
    }
    
    fn audio_callback_u8(
        data: &mut [u8],
        audio_buffer: &Arc<Mutex<VecDeque<f32>>>,
        channels: &[AudioChannel],
        sample_rate: f32,
    ) {
        let mut buffer = audio_buffer.lock().unwrap();
        
        for sample in data.iter_mut() {
            let mut output = 0.0;
            
            for channel in channels {
                if channel.enabled {
                    let channel_sample = Self::generate_sample(channel, sample_rate);
                    output += channel_sample * channel.volume;
                }
            }
            
            output = (output * 0.8).clamp(-1.0, 1.0);
            *sample = ((output + 1.0) * 127.5) as u8;
        }
    }
    
    fn generate_sample(channel: &AudioChannel, sample_rate: f32) -> f32 {
        let phase = (channel.frequency / sample_rate) * 2.0 * std::f32::consts::PI;
        
        match channel.waveform {
            Waveform::Square => {
                if (phase * 1000.0) as i32 % 1000 < (channel.duty_cycle * 1000.0) as i32 {
                    1.0
                } else {
                    -1.0
                }
            },
            Waveform::Triangle => {
                let t = (phase * 1000.0) as i32 % 2000;
                if t < 1000 {
                    (t as f32 / 500.0) - 1.0
                } else {
                    3.0 - (t as f32 / 500.0)
                }
            },
            Waveform::Sawtooth => {
                ((phase * 1000.0) as i32 % 1000) as f32 / 500.0 - 1.0
            },
            Waveform::Sine => {
                (phase * 1000.0).sin()
            },
            Waveform::Noise => {
                // Simple pseudo-random noise
                ((phase * 1000.0) as i32 * 1103515245 + 12345) as f32 / i32::MAX as f32
            },
        }
    }
    
    /// Update audio system (called each frame)
    pub fn update(&mut self) -> EmuliteResult<()> {
        // Skip audio processing if disabled
        if !self.config.enabled {
            return Ok(());
        }
        
        // Update sample clock
        self.sample_clock += 1.0;
        
        // Generate samples for this frame
        let samples_per_frame = self.config.sample_rate / 60; // Assuming 60 FPS
        
        for _ in 0..samples_per_frame {
            let mut sample = 0.0;
            
            for channel in &self.channels {
                if channel.enabled {
                    let channel_sample = Self::generate_sample(channel, self.config.sample_rate as f32);
                    sample += channel_sample * channel.volume;
                }
            }
            
            // Add to buffer
            let mut buffer = self.audio_buffer.lock().unwrap();
            buffer.push_back(sample);
            
            // Keep buffer size manageable
            if buffer.len() > self.config.buffer_size * 4 {
                buffer.pop_front();
            }
        }
        
        Ok(())
    }
    
    /// Set channel frequency
    pub fn set_channel_frequency(&mut self, channel: usize, frequency: f32) -> EmuliteResult<()> {
        if channel >= self.channels.len() {
            return Err(EmuliteError::AudioError(format!("Invalid channel: {}", channel)));
        }
        self.channels[channel].frequency = frequency;
        Ok(())
    }
    
    /// Set channel volume
    pub fn set_channel_volume(&mut self, channel: usize, volume: f32) -> EmuliteResult<()> {
        if channel >= self.channels.len() {
            return Err(EmuliteError::AudioError(format!("Invalid channel: {}", channel)));
        }
        self.channels[channel].volume = volume.clamp(0.0, 1.0);
        Ok(())
    }
    
    /// Enable/disable channel
    pub fn set_channel_enabled(&mut self, channel: usize, enabled: bool) -> EmuliteResult<()> {
        if channel >= self.channels.len() {
            return Err(EmuliteError::AudioError(format!("Invalid channel: {}", channel)));
        }
        self.channels[channel].enabled = enabled;
        Ok(())
    }
    
    /// Set channel waveform
    pub fn set_channel_waveform(&mut self, channel: usize, waveform: Waveform) -> EmuliteResult<()> {
        if channel >= self.channels.len() {
            return Err(EmuliteError::AudioError(format!("Invalid channel: {}", channel)));
        }
        self.channels[channel].waveform = waveform;
        Ok(())
    }
    
    /// Set master volume
    pub fn set_volume(&mut self, volume: f32) {
        self.config.volume = volume.clamp(0.0, 1.0);
    }
    
    /// Get current configuration
    pub fn config(&self) -> &AudioConfig {
        &self.config
    }
    
    /// Stop audio system
    pub fn stop(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
    }
}

impl Drop for AudioSystem {
    fn drop(&mut self) {
        self.stop();
    }
}
