use crate::services::onboarding::wizard::InteractiveWizard;
use std::collections::BTreeMap;
use std::io::Write;

pub fn run_cli(mut writer: impl Write, is_cloud: bool) -> Result<(), String> {
    let wizard = InteractiveWizard::new();
    let config = wizard.run_interactive_setup(is_cloud)?;

    let mode = if is_cloud { "Cloud-native" } else { "Standalone" };

    writeln!(writer, "OHC Interactive Setup ({})", mode).map_err(|e| e.to_string())?;
    writeln!(writer, "Configuration Options:").map_err(|e| e.to_string())?;

    let sorted_config: BTreeMap<_, _> = config.iter().collect();

    for (k, v) in sorted_config {
        writeln!(writer, "  {}: {}", k, v).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_cli_cloud() {
        let mut buf = Vec::new();
        let res = run_cli(&mut buf, true);
        assert!(res.is_ok());

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("OHC Interactive Setup (Cloud-native)"));
        assert!(output.contains("mode: cloud"));
    }

    #[test]
    fn test_run_cli_standalone() {
        let mut buf = Vec::new();
        let res = run_cli(&mut buf, false);
        assert!(res.is_ok());

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("OHC Interactive Setup (Standalone)"));
        assert!(output.contains("mode: standalone"));
    }
}
