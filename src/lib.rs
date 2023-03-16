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

pub enum LogType {
    REQUEST,
    RESPONSE,
}

impl Display for LogType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            LogType::REQUEST => write!(formatter, "REQUEST"),
            LogType::RESPONSE => write!(formatter, "RESPONSE"),
        }
    }
}

pub struct HTTPLog {
    pub timestamp: DateTime<Utc>,
    pub log_level: LogLevel,
    pub log_type: LogType,
    pub originating_ip_addr: String,
    pub api: String,
    pub restful_method: String,
    pub response_status_code: Option<i16>,
    pub body_as_utf8_str: Option<String>,
}

impl HTTPLog {
    pub fn as_log_str(&self) -> String {
        format!(
            "[{}] [{}] [{}] [{}] [{}] [{}] [{}] [{}]",
            &self.timestamp.to_rfc3339(),
            &self.log_level,
            &self.log_type,
            &self.originating_ip_addr,
            &self.api,
            &self.restful_method,
            match &self.response_status_code {
                None => "".to_owned(),
                Some(response_status_code) => response_status_code.to_string(),
            },
            match &self.body_as_utf8_str {
                None => "",
                Some(body_as_utf8_str) => body_as_utf8_str,
            }
        )
    }
}

pub fn write_to_server_log(log_str: String) -> io::Result<()> {
    // Create the path for the desired logging area (if not exists)
    fs::create_dir_all(get_env_var("DUST_LOG_PATH"))?;

    let mut log_file = OpenOptions::new().create(true).append(true).open(format!(
        "{}/{}.{}",
        get_env_var("DUST_LOG_PATH"),
        "server",
        get_env_var("DUST_LOG_FMT")
    ))?;

    match writeln!(log_file, "{}", log_str) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::{write_to_server_log, HTTPLog, LogLevel, LogType};
    use chrono::prelude::*;

    #[test]
    fn test_http_request_log_as_log_str() {
        let log = HTTPLog {
            timestamp: Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(),
            log_level: LogLevel::INFO,
            log_type: LogType::REQUEST,
            originating_ip_addr: "35.111.95.142".to_owned(),
            api: "/api/v1/health_check".to_owned(),
            restful_method: "GET".to_owned(),
            response_status_code: None,
            body_as_utf8_str: Some("{\"json_key\": \"json_value_str\"}".to_owned()),
        };

        assert_eq!(
            log.as_log_str(),
            "[2014-07-08T09:10:11+00:00] [INFO] [REQUEST] [35.111.95.142] [/api/v1/health_check] [GET] [] [{\"json_key\": \"json_value_str\"}]"
        );

        match write_to_server_log(log.as_log_str()) {
            Ok(_) => assert_eq!(true, true),
            Err(_) => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_http_response_log_as_log_str() {
        let log = HTTPLog {
            timestamp: Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(),
            log_level: LogLevel::INFO,
            log_type: LogType::RESPONSE,
            originating_ip_addr: "127.0.0.1".to_owned(),
            api: "/api/v1/health_check".to_owned(),
            restful_method: "GET".to_owned(),
            response_status_code: Some(200),
            body_as_utf8_str: None,
        };

        assert_eq!(
            log.as_log_str(),
            "[2014-07-08T09:10:11+00:00] [INFO] [RESPONSE] [127.0.0.1] [/api/v1/health_check] [GET] [200] []"
        );

        match write_to_server_log(log.as_log_str()) {
            Ok(_) => assert_eq!(true, true),
            Err(_) => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_write_to_server_log() {
        let log = HTTPLog {
            timestamp: Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(),
            log_level: LogLevel::INFO,
            log_type: LogType::REQUEST,
            originating_ip_addr: "35.111.95.142".to_owned(),
            api: "/api/v1/health_check".to_owned(),
            restful_method: "GET".to_owned(),
            response_status_code: None,
            body_as_utf8_str: Some("{\"json_key\": \"json_value_str\"}".to_owned()),
        };

        match write_to_server_log(log.as_log_str()) {
            Ok(_) => assert_eq!(true, true),
            Err(_) => assert_eq!(false, true),
        }
    }
}
