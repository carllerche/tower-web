use Service;

/// Generate the service implementations
pub(crate) fn generate(services: &[Service]) -> String {
    services.iter()
        .map(|service| service.gen().to_string())
        .collect()
}
