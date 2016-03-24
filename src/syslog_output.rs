use syslog::{Severity, Facility, Logger, unix};
use log::{LogRecord, LogLevel, LogMetadata, Log, LogLevelFilter, set_logger, SetLoggerError};
use std::fmt;

pub struct SyslogLogger(Box<Logger>);

impl Log for SyslogLogger {


    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Trace
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let severity = match record.level() {
                LogLevel::Error => Severity::LOG_ERR,
                LogLevel::Warn => Severity::LOG_WARNING,
                LogLevel::Info => Severity::LOG_INFO,
                LogLevel::Debug => Severity::LOG_DEBUG,
                LogLevel::Trace => Severity::LOG_DEBUG
            };
                
            self.0.send_3164(severity, &fmt::format(*record.args())).unwrap();
        }
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    let logger_result : Result<Box<Log>,()> = match unix(Facility::LOG_DAEMON) {
        Err(e)  => panic!("impossible to connect to syslog: {:?}", e),
        Ok(writer) => {
            Ok(Box::new(SyslogLogger(writer)) as Box<Log>)
        }
    };
    
    set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        logger_result.unwrap()
    })
}
