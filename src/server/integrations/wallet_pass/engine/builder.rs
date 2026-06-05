use super::{AppleWalletPass, PassBarcode, PassField, PassStructure};
use serde_json::Value;

pub struct PassBuilder {
    pass: AppleWalletPass,
    pass_type: String, // "generic", "storeCard", "eventTicket"
}

impl PassBuilder {
    pub fn new(
        pass_type_identifier: String,
        team_identifier: String,
        organization_name: String,
        description: String,
        serial_number: String,
        pass_type: &str, // "generic", "storeCard", "eventTicket"
    ) -> Self {
        Self {
            pass_type: pass_type.to_string(),
            pass: AppleWalletPass {
                format_version: 1,
                pass_type_identifier,
                serial_number,
                team_identifier,
                organization_name,
                description,
                logo_text: None,
                foreground_color: None,
                background_color: None,
                label_color: None,
                generic: if pass_type == "generic" { Some(PassStructure::default()) } else { None },
                store_card: if pass_type == "storeCard" { Some(PassStructure::default()) } else { None },
                event_ticket: if pass_type == "eventTicket" { Some(PassStructure::default()) } else { None },
                barcodes: None,
                barcode: None,
            },
        }
    }

    pub fn with_colors(mut self, background: &str, foreground: &str, label: &str) -> Self {
        self.pass.background_color = Some(background.to_string());
        self.pass.foreground_color = Some(foreground.to_string());
        self.pass.label_color = Some(label.to_string());
        self
    }

    pub fn with_logo_text(mut self, text: &str) -> Self {
        self.pass.logo_text = Some(text.to_string());
        self
    }

    pub fn with_barcode(mut self, message: &str) -> Self {
        let barcode = PassBarcode {
            format: "PKBarcodeFormatQR".to_string(),
            message: message.to_string(),
            message_encoding: "iso-8859-1".to_string(),
        };
        self.pass.barcode = Some(barcode.clone());
        self.pass.barcodes = Some(vec![barcode]);
        self
    }

    fn get_structure_mut(&mut self) -> Option<&mut PassStructure> {
        match self.pass_type.as_str() {
            "generic" => self.pass.generic.as_mut(),
            "storeCard" => self.pass.store_card.as_mut(),
            "eventTicket" => self.pass.event_ticket.as_mut(),
            _ => None,
        }
    }

    pub fn add_primary_field(mut self, key: &str, label: &str, value: &str) -> Self {
        if let Some(structure) = self.get_structure_mut() {
            structure.primary_fields.push(PassField {
                key: key.to_string(),
                label: label.to_string(),
                value: value.to_string(),
                text_alignment: None,
            });
        }
        self
    }

    pub fn add_secondary_field(mut self, key: &str, label: &str, value: &str) -> Self {
        if let Some(structure) = self.get_structure_mut() {
            structure.secondary_fields.push(PassField {
                key: key.to_string(),
                label: label.to_string(),
                value: value.to_string(),
                text_alignment: None,
            });
        }
        self
    }

    pub fn add_auxiliary_field(mut self, key: &str, label: &str, value: &str) -> Self {
        if let Some(structure) = self.get_structure_mut() {
            structure.auxiliary_fields.push(PassField {
                key: key.to_string(),
                label: label.to_string(),
                value: value.to_string(),
                text_alignment: None,
            });
        }
        self
    }

    pub fn add_back_field(mut self, key: &str, label: &str, value: &str) -> Self {
        if let Some(structure) = self.get_structure_mut() {
            structure.back_fields.push(PassField {
                key: key.to_string(),
                label: label.to_string(),
                value: value.to_string(),
                text_alignment: None,
            });
        }
        self
    }

    pub fn build(self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(&self.pass)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_builder_generic() {
        let builder = PassBuilder::new(
            "pass.test.id".to_string(),
            "TEAM123".to_string(),
            "Test Org".to_string(),
            "Test Pass".to_string(),
            "12345".to_string(),
            "generic",
        )
        .with_colors("rgb(255, 255, 255)", "rgb(0, 0, 0)", "rgb(100, 100, 100)")
        .with_logo_text("My Logo")
        .with_barcode("123456789")
        .add_primary_field("points", "Points", "100")
        .add_secondary_field("name", "Name", "John Doe");

        let pass_json = builder.build().unwrap();

        assert_eq!(pass_json["pass_type_identifier"], "pass.test.id");
        assert_eq!(pass_json["team_identifier"], "TEAM123");
        assert_eq!(pass_json["organization_name"], "Test Org");
        assert_eq!(pass_json["description"], "Test Pass");
        assert_eq!(pass_json["serial_number"], "12345");
        assert_eq!(pass_json["background_color"], "rgb(255, 255, 255)");
        assert_eq!(pass_json["foreground_color"], "rgb(0, 0, 0)");
        assert_eq!(pass_json["label_color"], "rgb(100, 100, 100)");
        assert_eq!(pass_json["logo_text"], "My Logo");

        let barcode = &pass_json["barcode"];
        assert_eq!(barcode["format"], "PKBarcodeFormatQR");
        assert_eq!(barcode["message"], "123456789");

        let generic = &pass_json["generic"];
        assert!(generic.is_object());

        let primary_fields = &generic["primary_fields"];
        assert_eq!(primary_fields[0]["key"], "points");
        assert_eq!(primary_fields[0]["label"], "Points");
        assert_eq!(primary_fields[0]["value"], "100");

        let secondary_fields = &generic["secondary_fields"];
        assert_eq!(secondary_fields[0]["key"], "name");
        assert_eq!(secondary_fields[0]["label"], "Name");
        assert_eq!(secondary_fields[0]["value"], "John Doe");
    }

    #[test]
    fn test_pass_builder_event_ticket() {
        let builder = PassBuilder::new(
            "pass.test.id".to_string(),
            "TEAM123".to_string(),
            "Test Org".to_string(),
            "Test Pass".to_string(),
            "12345".to_string(),
            "eventTicket",
        )
        .add_auxiliary_field("gate", "Gate", "A1")
        .add_back_field("terms", "Terms", "No refunds");

        let pass_json = builder.build().unwrap();

        let event_ticket = &pass_json["event_ticket"];
        assert!(event_ticket.is_object());

        let auxiliary_fields = &event_ticket["auxiliary_fields"];
        assert_eq!(auxiliary_fields[0]["key"], "gate");

        let back_fields = &event_ticket["back_fields"];
        assert_eq!(back_fields[0]["key"], "terms");
    }
}
