//! 日志记录器模块，用于记录 iflow 消息

use crate::Message;
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

/// 日志记录器配置
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// 日志文件路径
    pub log_file: PathBuf,
    /// 是否启用日志记录
    pub enabled: bool,
    /// 日志文件最大大小（字节），超过此大小会轮转
    pub max_file_size: u64,
    /// 保留的日志文件数量
    pub max_files: u32,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_file: PathBuf::from("iflow_messages.log"),
            enabled: true,
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_files: 5,
        }
    }
}

/// 日志记录器
#[derive(Clone)]
pub struct MessageLogger {
    config: LoggerConfig,
    writer: Arc<Mutex<BufWriter<File>>>,
}

impl MessageLogger {
    /// 创建新的日志记录器
    pub fn new(config: LoggerConfig) -> Result<Self, io::Error> {
        if !config.enabled {
            return Ok(Self {
                config,
                writer: Arc::new(Mutex::new(BufWriter::new(File::create("/dev/null")?))),
            });
        }

        // 检查是否需要轮转日志文件
        if let Some(parent) = config.log_file.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // 检查文件大小
        if config.log_file.exists() {
            let metadata = std::fs::metadata(&config.log_file)?;
            if metadata.len() >= config.max_file_size {
                Self::rotate_log_file(&config)?;
            }
        }

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&config.log_file)?;

        Ok(Self {
            config,
            writer: Arc::new(Mutex::new(BufWriter::new(file))),
        })
    }

    /// 轮转日志文件
    fn rotate_log_file(config: &LoggerConfig) -> Result<(), io::Error> {
        if !config.log_file.exists() {
            return Ok(());
        }

        // 删除最旧的日志文件
        for i in (0..config.max_files).rev() {
            let old_path = if i == 0 {
                config.log_file.clone()
            } else {
                config.log_file.with_extension(format!("log.{}", i))
            };

            let new_path = if i + 1 >= config.max_files {
                // 超出保留数量的文件直接删除
                if old_path.exists() {
                    std::fs::remove_file(&old_path)?;
                }
                continue;
            } else {
                config.log_file.with_extension(format!("log.{}", i + 1))
            };

            if old_path.exists() {
                std::fs::rename(&old_path, &new_path)?;
            }
        }

        Ok(())
    }

    /// 记录消息
    pub async fn log_message(&self, message: &Message) -> Result<(), io::Error> {
        if !self.config.enabled {
            return Ok(());
        }

        let log_entry = self.format_message(message);
        let mut writer = self.writer.lock().await;

        writeln!(writer, "{}", log_entry)?;
        writer.flush()?;

        // 检查文件大小
        if writer.get_ref().metadata()?.len() >= self.config.max_file_size {
            Self::rotate_log_file(&self.config)?;

            // 重新打开文件
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.config.log_file)?;
            *writer = BufWriter::new(file);
        }

        Ok(())
    }

    /// 记录原始消息，使用 Debug 格式，不做任何加工
    fn format_message(&self, message: &Message) -> String {
        // 直接使用 Debug 格式输出原始消息结构
        format!("{:?}", message)
    }

    /// 获取当前日志文件路径
    pub fn log_file_path(&self) -> &Path {
        &self.config.log_file
    }

    /// 获取配置
    pub fn config(&self) -> &LoggerConfig {
        &self.config
    }
}
