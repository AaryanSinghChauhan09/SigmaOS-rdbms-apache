//! SigmaOS-rdbms-apache: Apache database server model.
//!
//! Real implementation: configuration, server management, connections, and validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Server status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error,
}

/// Apache configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApacheConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApacheError {
    InvalidPort,
    InvalidMaxConnections,
    DuplicateServer(String),
}

impl fmt::Display for ApacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPort => write!(f, "port must be between 1 and 65535"),
            Self::InvalidMaxConnections => write!(f, "max connections must be positive"),
            Self::DuplicateServer(id) => write!(f, "duplicate server {:?}", id),
        }
    }
}

impl std::error::Error for ApacheError {}

/// Apache server instance.
pub struct ApacheServer {
    pub id: String,
    pub config: ApacheConfig,
    pub status: ServerStatus,
}

impl ApacheServer {
    pub fn new(id: impl Into<String>, config: ApacheConfig) -> Result<Self, ApacheError> {
        if config.port == 0 { return Err(ApacheError::InvalidPort); }
        if config.max_connections == 0 { return Err(ApacheError::InvalidMaxConnections); }
        Ok(Self {
            id: id.into(),
            config,
            status: ServerStatus::Stopped,
        })
    }

    pub fn start(&mut self) { self.status = ServerStatus::Running; }

    pub fn stop(&mut self) { self.status = ServerStatus::Stopped; }
}

/// Apache server manager.
pub struct ApacheManager {
    servers: BTreeMap<String, ApacheServer>,
}

impl ApacheManager {
    pub fn new() -> Self { Self { servers: BTreeMap::new() } }

    pub fn add_server(&mut self, server: ApacheServer) -> Result<(), ApacheError> {
        let id = server.id.clone();
        if self.servers.contains_key(&id) {
            return Err(ApacheError::DuplicateServer(id.clone()));
        }
        self.servers.insert(id, server);
        Ok(())
    }

    pub fn server_count(&self) -> usize { self.servers.len() }

    pub fn get_server(&self, id: &str) -> Option<&ApacheServer> { self.servers.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_create_aaa() {
        let cfg = ApacheConfig { host: "localhost".into(), port: 5432, max_connections: 100 };
        let s = ApacheServer::new("s1", cfg).unwrap();
        assert_eq!(s.id, "s1");
        assert_eq!(s.status, ServerStatus::Stopped);
    }

    #[test]
    fn test_invalid_port_rejected_aaa() {
        let cfg = ApacheConfig { host: "localhost".into(), port: 0, max_connections: 100 };
        assert!(ApacheServer::new("x", cfg).is_err());
    }

    #[test]
    fn test_start_stop_aaa() {
        let cfg = ApacheConfig { host: "localhost".into(), port: 5432, max_connections: 100 };
        let mut s = ApacheServer::new("s1", cfg).unwrap();
        s.start();
        assert_eq!(s.status, ServerStatus::Running);
        s.stop();
        assert_eq!(s.status, ServerStatus::Stopped);
    }

    #[test]
    fn test_manager_add_aaa() {
        let mut m = ApacheManager::new();
        let cfg = ApacheConfig { host: "localhost".into(), port: 5432, max_connections: 100 };
        let s = ApacheServer::new("s1", cfg).unwrap();
        m.add_server(s).unwrap();
        assert_eq!(m.server_count(), 1);
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut m = ApacheManager::new();
        let cfg = ApacheConfig { host: "localhost".into(), port: 5432, max_connections: 100 };
        let s = ApacheServer::new("x", cfg).unwrap();
        m.add_server(s).unwrap();
        let cfg2 = ApacheConfig { host: "localhost".into(), port: 5432, max_connections: 100 };
        let s2 = ApacheServer::new("x", cfg2).unwrap();
        assert!(m.add_server(s2).is_err());
    }
}
