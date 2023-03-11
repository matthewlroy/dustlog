use chrono::prelude::*;
use core::fmt;
use dustcfg::get_env_var;
use std::fmt::Display;
use std::fs::{self, OpenOptions};
use std::io;
use std::io::prelude::*;


pub enum LogLevel {
    INFO,
    ERROR,
}

impl Display for LogLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            LogLevel::INFO => write!(formatter, "INFO"),
            LogLevel::ERROR => write!(formatter, "ERROR"),
        }
    }
}

pub struct HTTPRequestLog {
    pub log_level: LogLevel,
    pub timestamp: DateTime<Utc>,
    pub requester_ip_address: String,
    pub restful_method: String,
    pub api_called: String,
}

impl HTTPRequestLog {
    pub fn as_log_str(&self) -> String {
        format!(
            "[{}] [{}] [{}] [{}] [{}]",
            self.log_level,
            self.timestamp.to_rfc3339(),
            self.requester_ip_address,
            self.restful_method,
            self.api_called
        )
    }

    pub fn write_to_server_log(&self) -> io::Result<()> {
        // Create the path for the desired pile (if not exists)
        match fs::create_dir_all(get_env_var("DUST_LOG_PATH")) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }?;

        let mut log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!(
                "{}/{}.{}",
                get_env_var("DUST_LOG_PATH"),
                "server",
                get_env_var("DUST_LOG_FMT")
            ))
            .unwrap();

        match writeln!(log_file, "{}", &self.as_log_str()) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{HTTPRequestLog, LogLevel};
    use chrono::prelude::*;

    #[test]
    fn test_http_request_log_as_log_str() {
        let log = HTTPRequestLog {
            log_level: LogLevel::INFO,
            timestamp: Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(),
            requester_ip_address: "35.111.95.142".to_owned(),
            restful_method: "GET".to_owned(),
            api_called: "/api/v1/health_check".to_owned(),
        };

        assert_eq!(
            log.as_log_str(),
            "[INFO] [2014-07-08T09:10:11+00:00] [35.111.95.142] [GET] [/api/v1/health_check]"
        );
    }

    #[test]
    fn test_write_to_server_log() {
        let log = HTTPRequestLog {
            log_level: LogLevel::INFO,
            timestamp: Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(),
            requester_ip_address: "35.111.95.142".to_owned(),
            restful_method: "GET".to_owned(),
            api_called: "/api/v1/health_check".to_owned(),
        };

        match log.write_to_server_log() {
            Ok(_) => (),
            Err(_) => (),
        }
    }
}
