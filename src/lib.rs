use chrono::prelude::*;

pub struct HTTPRequestLog {
    timestamp: DateTime<Utc>,
    requester_ip_address: &'static str,
    restful_method: &'static str,
    api_called: &'static str,
}

impl HTTPRequestLog {
    pub fn as_log_str(&self) -> String {
        format!(
            "[{}] [{}] [{}] [{}]",
            self.timestamp.to_rfc3339(),
            self.requester_ip_address,
            self.restful_method,
            self.api_called
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::HTTPRequestLog;
    use chrono::prelude::*;

    #[test]
    fn test_http_request_log_as_log_str() {
        let log = HTTPRequestLog {
            timestamp: Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(),
            requester_ip_address: "35.111.95.142",
            restful_method: "GET",
            api_called: "/api/v1/health_check",
        };

        assert_eq!(
            log.as_log_str(),
            "[2014-07-08T09:10:11+00:00] [35.111.95.142] [GET] [/api/v1/health_check]"
        );
    }
}
