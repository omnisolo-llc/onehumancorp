use server_integrations_pkpass::generator::{PkpassGenerator, PkpassData};

#[test]
fn test_generator() {
    let generator = PkpassGenerator::new();
    let data = PkpassData {
        pass_type_identifier: "pass.store.ohc".to_string(),
        team_identifier: "ABC".to_string(),
        serial_number: "123".to_string(),
        organization_name: "OHC".to_string(),
        description: "A pass".to_string(),
        foreground_color: None,
        background_color: None,
    };
    let result = generator.generate(data);
    assert!(result.is_ok());
}
