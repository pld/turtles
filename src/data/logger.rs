use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::config::Config;

/// Logger configuration
#[derive(Debug, Clone)]
pub struct Logger {
    /// Log level
    level: LevelFilter,
    /// Whether to log to file
    log_to_file: bool,
    /// Log file directory
    log_dir: PathBuf,
    /// Maximum log file size in MB
    max_file_size: u32,
    /// Number of log files to keep
    max_files: u32,
    /// Current log file path
    current_log_file: Option<PathBuf>,
}

impl Logger {
    /// Create a new logger from configuration
    pub fn new(config: &Config) -> Self {
        // Convert string log level to LevelFilter
        let level = match config.logging.level.to_lowercase().as_str() {
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => LevelFilter::Info, // Default to info if invalid
        };

        // Determine log directory
        let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
        let log_dir = current_dir.join(".local").join("share").join("screensage").join("logs");

        Self {
            level,
            log_to_file: config.logging.log_to_file,
            log_dir,
            max_file_size: config.logging.max_file_size,
            max_files: config.logging.max_files,
            current_log_file: None,
        }
    }

    /// Initialize the logger
    pub fn init(&mut self) -> Result<()> {
        // Create log directory if it doesn't exist
        if self.log_to_file {
            fs::create_dir_all(&self.log_dir)
                .with_context(|| format!("Failed to create log directory: {}", self.log_dir.display()))?;
        }

        // Set up colors for console output
        let colors = ColoredLevelConfig::new()
            .error(Color::Red)
            .warn(Color::Yellow)
            .info(Color::Green)
            .debug(Color::Blue)
            .trace(Color::Magenta);

        // Create a dispatch for console logging
        let mut dispatch = fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    colors.color(record.level()),
                    record.target(),
                    message
                ))
            })
            .level(self.level)
            .chain(io::stdout());

        // Add file logging if enabled
        if self.log_to_file {
            // Generate log file path with current date
            let log_file_path = self.get_log_file_path()?;
            self.current_log_file = Some(log_file_path.clone());

            // Create file logger with detailed format
            let file_dispatch = fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{},{},{},{},{}",
                        Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                        record.level(),
                        record.target(),
                        record.line().unwrap_or(0),
                        message
                    ))
                })
                .chain(fern::log_file(log_file_path)?);

            dispatch = dispatch.chain(file_dispatch);
        }

        // Apply the logger configuration
        dispatch.apply();

        // Perform log rotation if needed
        if self.log_to_file {
            self.rotate_logs()?;
        }

        Ok(())
    }

    /// Get the path for the current log file
    fn get_log_file_path(&self) -> Result<PathBuf> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        let log_file_path = self.log_dir.join(format!("screensage-{}.log", today));
        Ok(log_file_path)
    }

    /// Rotate log files, removing old ones
    fn rotate_logs(&self) -> Result<()> {
        // Get all log files in the directory
        let entries = fs::read_dir(&self.log_dir)
            .with_context(|| format!("Failed to read log directory: {}", self.log_dir.display()))?;

        // Collect log files with their modification times
        let mut log_files: Vec<(PathBuf, DateTime<Local>)> = Vec::new();
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            // Only process .log files
            if path.extension().map_or(false, |ext| ext == "log") {
                if let Ok(metadata) = fs::metadata(&path) {
                    if let Ok(modified) = metadata.modified() {
                        // Convert SystemTime to DateTime<Local>
                        let duration_since_epoch = modified.duration_since(std::time::UNIX_EPOCH).ok();
                        if let Some(duration) = duration_since_epoch {
                            let secs = duration.as_secs() as i64;
                            let nsecs = duration.subsec_nanos();
                            if let Some(utc_datetime) = chrono::DateTime::<chrono::Utc>::from_timestamp(secs, nsecs) {
                                let datetime = utc_datetime.with_timezone(&chrono::Local);
                                log_files.push((path, datetime));
                            }
                        }
                    }
                }
            }
        }
        
        // Sort by modification time (oldest first)
        log_files.sort_by(|a, b| a.1.cmp(&b.1));
        
        // Remove oldest files if we have more than max_files
        if log_files.len() > self.max_files as usize {
            let files_to_remove = log_files.len() - self.max_files as usize;
            for (path, _) in log_files.iter().take(files_to_remove) {
                fs::remove_file(path)
                    .with_context(|| format!("Failed to remove old log file: {}", path.display()))?;
            }
        }
        
        Ok(())
    }

    /// Check if the current log file exceeds the maximum size
    pub fn check_log_file_size(&self) -> Result<bool> {
        if let Some(log_file_path) = &self.current_log_file {
            if log_file_path.exists() {
                let metadata = fs::metadata(log_file_path)
                    .with_context(|| format!("Failed to get metadata for log file: {}", log_file_path.display()))?;
                
                // Convert max_file_size from MB to bytes
                let max_size_bytes = self.max_file_size as u64 * 1024 * 1024;
                
                return Ok(metadata.len() > max_size_bytes);
            }
        }
        
        Ok(false)
    }
}

/// Initialize the logger with the given configuration
pub fn init_logger(config: &Config) -> Result<Logger> {
    let mut logger = Logger::new(config);
    logger.init()?;
    Ok(logger)
}

/// Create a test logger for unit tests
#[cfg(test)]
pub fn init_test_logger() -> Result<()> {
    // Simple logger for tests that only logs to console
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(io::stdout())
        .apply()?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use tempfile::tempdir;
    use log::{debug, error, info, trace, warn};

    #[test]
    fn test_logger_initialization() {
        // Create a test configuration
        let mut config = Config::default();
        config.logging.log_to_file = true;
        
        // Create a logger
        let mut logger = Logger::new(&config);
        
        // Initialize the logger
        assert!(logger.init().is_ok());
        
        // Log some test messages
        error!("Test error message");
        warn!("Test warning message");
        info!("Test info message");
        debug!("Test debug message");
        trace!("Test trace message");
        
        // Check that the log file was created
        if let Some(log_file_path) = &logger.get_log_file_path().ok() {
            assert!(log_file_path.exists());
            
            // Read the log file content
            let content = fs::read_to_string(log_file_path).unwrap();
            
            // Verify that the log messages were written
            assert!(content.contains("Test error message"));
            assert!(content.contains("Test warning message"));
            assert!(content.contains("Test info message"));
            
            // Debug and trace might not be in the file depending on the default level
            if config.logging.level == "debug" || config.logging.level == "trace" {
                assert!(content.contains("Test debug message"));
            }
            
            if config.logging.level == "trace" {
                assert!(content.contains("Test trace message"));
            }
        } else {
            panic!("Log file path not set");
        }
    }

    #[test]
    fn test_log_rotation() {
        // Create a temporary directory for logs
        let temp_dir = tempdir().unwrap();
        
        // Create a test configuration with small max file size
        let mut config = Config::default();
        config.logging.log_to_file = true;
        config.logging.max_file_size = 1; // 1MB
        config.logging.max_files = 3;     // Keep 3 files
        
        // Create a logger
        let mut logger = Logger::new(&config);
        
        // Override the log directory to use the temporary directory
        logger.log_dir = temp_dir.path().to_path_buf();
        
        // Initialize the logger
        assert!(logger.init().is_ok());
        
        // Create some test log files with different dates
        for i in 1..6 {
            let log_file = logger.log_dir.join(format!("screensage-2023-01-0{}.log", i));
            let mut file = std::fs::File::create(&log_file).unwrap();
            // Write some content to the file
            use std::io::Write;
            writeln!(file, "Test log file {}", i).unwrap();
        }
        
        // Rotate logs
        assert!(logger.rotate_logs().is_ok());

        // Check that only the 3 newest files remain (based on our max_files setting)
        let entries = fs::read_dir(&logger.log_dir).unwrap();
        let log_files: Vec<_> = entries
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.path().extension().map_or(false, |ext| ext == "log")
            })
            .collect();
        
        assert_eq!(log_files.len(), 3);
    }
}
