use sigmaos_rdbms_apache::{ApacheConfig, ApacheManager, ApacheServer};

#[test]
fn apache_manager_integration_aaa() {
    let mut m = ApacheManager::new();
    m.add_server(
        ApacheServer::new(
            "s1",
            ApacheConfig {
                host: "localhost".into(),
                port: 5432,
                max_connections: 100,
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(m.server_count(), 1);
}
