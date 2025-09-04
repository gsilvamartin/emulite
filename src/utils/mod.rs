//! Utility functions and helpers

use crate::{EmuliteResult, EmuliteError};
use std::path::Path;
use std::fs;

/// File utilities
pub struct FileUtils;

impl FileUtils {
    /// Check if file exists and is readable
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }
    
    /// Get file size
    pub fn get_file_size(path: &str) -> EmuliteResult<u64> {
        let metadata = fs::metadata(path)?;
        Ok(metadata.len())
    }
    
    /// Read file as bytes
    pub fn read_file(path: &str) -> EmuliteResult<Vec<u8>> {
        Ok(fs::read(path)?)
    }
    
    /// Write bytes to file
    pub fn write_file(path: &str, data: &[u8]) -> EmuliteResult<()> {
        Ok(fs::write(path, data)?)
    }
    
    /// Create directory if it doesn't exist
    pub fn create_dir(path: &str) -> EmuliteResult<()> {
        Ok(fs::create_dir_all(path)?)
    }
    
    /// Get file extension
    pub fn get_extension(path: &str) -> Option<String> {
        Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }
    
    /// Get filename without extension
    pub fn get_stem(path: &str) -> Option<String> {
        Path::new(path)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(|s| s.to_string())
    }
    
    /// Get parent directory
    pub fn get_parent(path: &str) -> Option<String> {
        Path::new(path)
            .parent()
            .and_then(|parent| parent.to_str())
            .map(|s| s.to_string())
    }
}

/// Math utilities
pub struct MathUtils;

impl MathUtils {
    /// Clamp value between min and max
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
    
    /// Linear interpolation
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }
    
    /// Convert degrees to radians
    pub fn deg_to_rad(degrees: f32) -> f32 {
        degrees * std::f32::consts::PI / 180.0
    }
    
    /// Convert radians to degrees
    pub fn rad_to_deg(radians: f32) -> f32 {
        radians * 180.0 / std::f32::consts::PI
    }
    
    /// Check if value is power of 2
    pub fn is_power_of_2(n: u32) -> bool {
        n != 0 && (n & (n - 1)) == 0
    }
    
    /// Get next power of 2
    pub fn next_power_of_2(n: u32) -> u32 {
        if n == 0 {
            1
        } else {
            1 << (32 - n.leading_zeros())
        }
    }
}

/// String utilities
pub struct StringUtils;

impl StringUtils {
    /// Convert string to lowercase
    pub fn to_lowercase(s: &str) -> String {
        s.to_lowercase()
    }
    
    /// Convert string to uppercase
    pub fn to_uppercase(s: &str) -> String {
        s.to_uppercase()
    }
    
    /// Trim whitespace
    pub fn trim(s: &str) -> String {
        s.trim().to_string()
    }
    
    /// Split string by delimiter
    pub fn split(s: &str, delimiter: &str) -> Vec<String> {
        s.split(delimiter).map(|s| s.to_string()).collect()
    }
    
    /// Join strings with delimiter
    pub fn join(strings: &[String], delimiter: &str) -> String {
        strings.join(delimiter)
    }
    
    /// Check if string starts with prefix
    pub fn starts_with(s: &str, prefix: &str) -> bool {
        s.starts_with(prefix)
    }
    
    /// Check if string ends with suffix
    pub fn ends_with(s: &str, suffix: &str) -> bool {
        s.ends_with(suffix)
    }
    
    /// Replace all occurrences
    pub fn replace_all(s: &str, from: &str, to: &str) -> String {
        s.replace(from, to)
    }
}

/// Time utilities
pub struct TimeUtils;

impl TimeUtils {
    /// Get current timestamp
    pub fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    /// Get current timestamp in milliseconds
    pub fn current_timestamp_ms() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
    
    /// Format duration as string
    pub fn format_duration(seconds: u64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        
        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, secs)
        } else {
            format!("{:02}:{:02}", minutes, secs)
        }
    }
    
    /// Sleep for specified milliseconds
    pub fn sleep_ms(ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
    
    /// Sleep for specified seconds
    pub fn sleep_secs(secs: u64) {
        std::thread::sleep(std::time::Duration::from_secs(secs));
    }
}

/// Hash utilities
pub struct HashUtils;

impl HashUtils {
    /// Calculate CRC32 checksum
    pub fn crc32(data: &[u8]) -> u32 {
        let mut crc = 0xFFFFFFFFu32;
        
        for &byte in data {
            crc ^= byte as u32;
            for _ in 0..8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
        }
        
        !crc
    }
    
    /// Calculate simple hash
    pub fn simple_hash(data: &[u8]) -> u32 {
        let mut hash = 0u32;
        for &byte in data {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
        }
        hash
    }
    
    /// Calculate MD5 hash (simplified)
    pub fn md5(_data: &[u8]) -> String {
        // This is a placeholder - in a real implementation you'd use a proper MD5 library
        "00000000000000000000000000000000".to_string()
    }
}

/// Validation utilities
pub struct ValidationUtils;

impl ValidationUtils {
    /// Validate ROM file
    pub fn validate_rom(path: &str) -> EmuliteResult<()> {
        if !FileUtils::file_exists(path) {
            return Err(EmuliteError::InvalidRom("File does not exist".to_string()));
        }
        
        let size = FileUtils::get_file_size(path)?;
        if size == 0 {
            return Err(EmuliteError::InvalidRom("File is empty".to_string()));
        }
        
        if size > 100 * 1024 * 1024 { // 100MB limit
            return Err(EmuliteError::InvalidRom("File too large".to_string()));
        }
        
        Ok(())
    }
    
    /// Validate memory address
    pub fn validate_address(address: u32, max_address: u32) -> EmuliteResult<()> {
        if address > max_address {
            return Err(EmuliteError::MemoryAccessViolation(address));
        }
        Ok(())
    }
    
    /// Validate configuration
    pub fn validate_config(config: &crate::config::Config) -> EmuliteResult<()> {
        config.validate()
    }
}

/// Performance utilities
pub struct PerformanceUtils;

impl PerformanceUtils {
    /// Measure execution time
    pub fn measure_time<F, R>(f: F) -> (R, std::time::Duration)
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }
    
    /// Benchmark function
    pub fn benchmark<F>(f: F, iterations: usize) -> std::time::Duration
    where
        F: Fn(),
    {
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            f();
        }
        start.elapsed()
    }
    
    /// Get memory usage (simplified)
    pub fn get_memory_usage() -> u64 {
        // This is a placeholder - in a real implementation you'd use system-specific APIs
        0
    }
}

/// Error handling utilities
pub struct ErrorUtils;

impl ErrorUtils {
    /// Convert error to string
    pub fn error_to_string(error: &dyn std::error::Error) -> String {
        error.to_string()
    }
    
    /// Get error chain
    pub fn get_error_chain(error: &dyn std::error::Error) -> Vec<String> {
        let mut chain = vec![error.to_string()];
        let mut source = error.source();
        
        while let Some(err) = source {
            chain.push(err.to_string());
            source = err.source();
        }
        
        chain
    }
    
    /// Log error with context
    pub fn log_error(error: &dyn std::error::Error, context: &str) {
        log::error!("{}: {}", context, error);
        
        let chain = Self::get_error_chain(error);
        if chain.len() > 1 {
            log::error!("Error chain:");
            for (i, err) in chain.iter().enumerate() {
                log::error!("  {}: {}", i, err);
            }
        }
    }
}

/// Platform detection utilities
pub struct PlatformUtils;

impl PlatformUtils {
    /// Get current operating system
    pub fn get_os() -> &'static str {
        if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else {
            "unknown"
        }
    }
    
    /// Get current architecture
    pub fn get_arch() -> &'static str {
        if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else if cfg!(target_arch = "x86") {
            "x86"
        } else if cfg!(target_arch = "arm") {
            "arm"
        } else if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else {
            "unknown"
        }
    }
    
    /// Check if running on mobile
    pub fn is_mobile() -> bool {
        cfg!(target_os = "android") || cfg!(target_os = "ios")
    }
    
    /// Get platform-specific path separator
    pub fn get_path_separator() -> char {
        if cfg!(target_os = "windows") {
            '\\'
        } else {
            '/'
        }
    }
}
