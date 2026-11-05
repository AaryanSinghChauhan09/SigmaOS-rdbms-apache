#[derive(Debug, Clone)]
pub struct ApacheDbConfig { pub db_type: String, pub host: String, pub port: u16 }
impl ApacheDbConfig {
    pub fn new(t: impl Into<String>, host: impl Into<String>, port: u16) -> Self {
        Self { db_type: t.into(), host: host.into(), port }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_cfg() {
        let c = ApacheDbConfig::new("cassandra", "127.0.0.1", 9042);
        assert_eq!(c.port, 9042);
    }
}
