use chrono::prelude::*;
use core::fmt;
use dustcfg::get_env_var;
use std::fmt::Display;
use std::fs::{self, OpenOptions};
use std::io;
use std::io::prelude::*;

pub enum LogDistinction {
    SERVER,
    DB,
}

impl Display for LogDistinction {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            LogDistinction::SERVER => write!(formatter, "server"),
            LogDistinction::DB => write!(formatter, "db"),
        }
    }
}

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

pub struct DBRequestLog {
    pub timestamp: DateTime<Utc>,
    pub log_level: LogLevel,
    pub log_type: LogType,
    pub socket_addr: String,
    pub method: String,
    pub pile_name: Option<String>,
    pub payload_size_in_bytes: Option<usize>,
}

impl DBRequestLog {
    pub fn as_log_str(&self) -> String {
        format!(
            "[{}] [{}] [{}] [{}] [{}] [{}] [{}B]",
            &self.timestamp.to_rfc3339(),
            &self.log_level,
            &self.log_type,
            &self.socket_addr,
            &self.method,
            match &self.pile_name {
                None => "",
                Some(pile_name) => pile_name,
            },
            match &self.payload_size_in_bytes {
                None => "0".to_owned(),
                Some(payload_size_in_bytes) => payload_size_in_bytes.to_string(),
            },
        )
    }
}

pub struct HTTPRequestLog {
    pub timestamp: DateTime<Utc>,
    pub log_level: LogLevel,
    pub log_type: LogType,
    pub originating_ip_addr: String,
    pub api: String,
    pub restful_method: String,
    pub payload_size_in_bytes: Option<usize>,
    pub body_as_utf8_str: Option<String>,
}

impl HTTPRequestLog {
    pub fn as_log_str(&self) -> String {
        format!(
            "[{}] [{}] [{}] [{}] [{}] [{}] [{}B] [{}]",
            &self.timestamp.to_rfc3339(),
            &self.log_level,
            &self.log_type,
            &self.originating_ip_addr,
            &self.api,
            &self.restful_method,
            match &self.payload_size_in_bytes {
                None => "0".to_owned(),
                Some(payload_size_in_bytes) => payload_size_in_bytes.to_string(),
            },
            match &self.body_as_utf8_str {
                None => "",
                Some(body_as_utf8_str) => body_as_utf8_str,
            }
        )
    }
}

pub struct HTTPResponseLog {
    pub timestamp: DateTime<Utc>,
    pub log_level: LogLevel,
    pub log_type: LogType,
    pub originating_ip_addr: String,
    pub response_status_code: u16,
    pub body_as_utf8_str: Option<String>,
}

impl HTTPResponseLog {
    pub fn as_log_str(&self) -> String {
        format!(
            "[{}] [{}] [{}] [{}] [{}] [{}]",
            &self.timestamp.to_rfc3339(),
            &self.log_level,
            &self.log_type,
            &self.originating_ip_addr,
            &self.response_status_code.to_string(),
            match &self.body_as_utf8_str {
                None => "",
                Some(body_as_utf8_str) => body_as_utf8_str,
            }
        )
    }
}

pub fn write_to_log(log_str: String, log_distinction: LogDistinction) -> io::Result<()> {
    // Create the path for the desired logging area (if not exists)
    fs::create_dir_all(get_env_var("DUST_LOG_PATH"))?;

    let mut log_file = OpenOptions::new().create(true).append(true).open(format!(
        "{}/{}.{}",
        get_env_var("DUST_LOG_PATH"),
        log_distinction,
        get_env_var("DUST_LOG_FMT")
    ))?;

    match writeln!(log_file, "{}", log_str) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        write_to_log, DBRequestLog, HTTPRequestLog, HTTPResponseLog, LogDistinction, LogLevel,
        LogType,
    };
    use chrono::prelude::*;

    #[test]
    fn test_http_request_log_as_log_str() {
        let log = HTTPRequestLog {
            timestamp: Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(),
            log_level: LogLevel::INFO,
            log_type: LogType::REQUEST,
            originating_ip_addr: "35.111.95.142".to_owned(),
            api: "/api/v1/health_check".to_owned(),
            restful_method: "GET".to_owned(),
            payload_size_in_bytes: Some(30),
            body_as_utf8_str: Some("{\"json_key\": \"json_value_str\"}".to_owned()),
        };

        assert_eq!(
            log.as_log_str(),
            "[2014-07-08T09:10:11+00:00] [INFO] [REQUEST] [35.111.95.142] [/api/v1/health_check] [GET] [30B] [{\"json_key\": \"json_value_str\"}]"
        );

        match write_to_log(log.as_log_str(), LogDistinction::SERVER) {
            Ok(_) => assert_eq!(true, true),
            Err(_) => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_http_response_log_as_log_str() {
        let log = HTTPResponseLog {
            timestamp: Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(),
            log_level: LogLevel::INFO,
            log_type: LogType::RESPONSE,
            originating_ip_addr: "127.0.0.1".to_owned(),
            response_status_code: 200,
            body_as_utf8_str: None,
        };

        assert_eq!(
            log.as_log_str(),
            "[2014-07-08T09:10:11+00:00] [INFO] [RESPONSE] [127.0.0.1] [200] []"
        );

        match write_to_log(log.as_log_str(), LogDistinction::SERVER) {
            Ok(_) => assert_eq!(true, true),
            Err(_) => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_write_to_server_log() {
        let log = HTTPRequestLog {
            timestamp: Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(),
            log_level: LogLevel::INFO,
            log_type: LogType::REQUEST,
            originating_ip_addr: "35.111.95.142".to_owned(),
            api: "/api/v1/health_check".to_owned(),
            restful_method: "GET".to_owned(),
            payload_size_in_bytes: Some(30),
            body_as_utf8_str: Some("{\"json_key\": \"json_value_str\"}".to_owned()),
        };

        match write_to_log(log.as_log_str(), LogDistinction::SERVER) {
            Ok(_) => assert_eq!(true, true),
            Err(_) => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_db_request_log_as_log_str() {
        let log = DBRequestLog {
            timestamp: Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(),
            log_level: LogLevel::INFO,
            log_type: LogType::REQUEST,
            socket_addr: "127.0.0.1:44089".to_owned(),
            method: "CREATE".to_owned(),
            pile_name: Some("users".to_owned()),
            payload_size_in_bytes: Some(30),
        };

        assert_eq!(
            log.as_log_str(),
            "[2014-07-08T09:10:11+00:00] [INFO] [REQUEST] [127.0.0.1:44089] [CREATE] [users] [30B]"
        );

        match write_to_log(log.as_log_str(), LogDistinction::DB) {
            Ok(_) => assert_eq!(true, true),
            Err(_) => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_write_to_db_log() {
        let log = DBRequestLog {
            timestamp: Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(),
            log_level: LogLevel::INFO,
            log_type: LogType::REQUEST,
            socket_addr: "127.0.0.1:44089".to_owned(),
            method: "CREATE".to_owned(),
            pile_name: Some("users".to_owned()),
            payload_size_in_bytes: Some(30),
        };

        match write_to_log(log.as_log_str(), LogDistinction::DB) {
            Ok(_) => assert_eq!(true, true),
            Err(_) => assert_eq!(false, true),
        }
    }
}
