use log;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender, rolling_file::{
        policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy
        }, RollingFileAppender
    }},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder, filter::threshold::ThresholdFilter,
};

fn str_to_level(level: &str, default: log::LevelFilter) -> log::LevelFilter {
    if level == "debug" {
        return log::LevelFilter::Debug;
    } else if level == "info" {
        return log::LevelFilter::Info;
    } else if level == "warn" {
        return log::LevelFilter::Warn;
    } else if level == "error" {
        return log::LevelFilter::Error;
    } else {
        return default;
    }
}

pub struct LogBuilder {
    m_console_level: log::LevelFilter,
    m_file_level: log::LevelFilter,
    m_filename: String,
    m_filesize_limit: u64,
    m_file_count: u32,
}

impl LogBuilder {
    pub fn new() -> Self {
        LogBuilder {
            m_console_level: log::LevelFilter::Info,
            m_file_level: log::LevelFilter::Debug,
            m_filename: String::new(),
            m_filesize_limit: 0,
            m_file_count: 3,
        }
    }

    pub fn with_log_file(mut self, filename: &str) -> Self {
        self.m_filename = filename.to_string();

        self
    }

    pub fn with_console_level(mut self, level: &str) -> Self {
        self.m_console_level = str_to_level(level, self.m_console_level);

        self
    }  

    pub fn with_file_level(mut self, level: &str) -> Self {
        self.m_file_level = str_to_level(level, self.m_file_level);

        self
    }  

    pub fn with_filesize_limit(mut self, sz: u64) -> Self {
        self.m_filesize_limit = sz;
        self
    }

    pub fn with_file_count(mut self, num: u32) -> Self {
        self.m_file_count = num;
        self
    }

    pub fn build(self) -> Result<(), Box<dyn std::error::Error>> {
        let console = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S.%3f)} {l} {m}{n}")))
            .build();

        let console_appender = Appender::builder()
            .filter(Box::new(ThresholdFilter::new(self.m_console_level)))
            .build("console", Box::new(console));

        let mut config_builder = Config::builder()
            .appender(console_appender);

        if !self.m_filename.is_empty() {
            if self.m_filesize_limit > 0 {
                let trigger = SizeTrigger::new(self.m_filesize_limit);
                let roller = FixedWindowRoller::builder()
                    .build("{}.{}.log", self.m_file_count)?;
                
                let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

                let rolling_file = RollingFileAppender::builder()
                    .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S.%3f)} {l} {m}{n}")))
                    .build(&self.m_filename, Box::new(policy))?;

                let rolling_file_appender = Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(self.m_file_level)))
                    .build("rolling_file", Box::new(rolling_file));

                config_builder = config_builder
                    .appender(rolling_file_appender);
            } else {
                let file = FileAppender::builder()
                    .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S.%3f)} {l} {m}{n}")))
                    .build(&self.m_filename)?;

                let file_appender = Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(self.m_file_level)))
                    .build("file", Box::new(file));

                config_builder = config_builder
                    .appender(file_appender);
            }
        }

        // Root logger with console and optional file/rolling loggers
        // Note: root logger must be at Debug level to catch all
        let root_logger = if !self.m_filename.is_empty() {
            if self.m_filesize_limit > 0 {
                Root::builder()
                    .appender("console")
                    .appender("rolling_file")
                    .build(log::LevelFilter::Debug)
            } else {
                Root::builder()
                    .appender("console")
                    .appender("file")
                    .build(log::LevelFilter::Debug)
            }
        } else {
            Root::builder()
                .appender("console")
                .build(self.m_console_level)
        };

        let config = config_builder.build(root_logger)?;

        log4rs::init_config(config)?;

        Ok(())
    }
}