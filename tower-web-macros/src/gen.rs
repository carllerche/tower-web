use Resource;

/// Generate the `Resource` implementations
pub(crate) fn generate(resources: &[Resource]) -> String {
    resources.iter()
        .map(|resource| resource.gen().to_string())
        .collect()
}
