use tracing_core::{Event, Subscriber};
use tracing_subscriber::fmt::{format::Writer, FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;
use tracing_core::Field;
use tracing_subscriber::field::Visit;
use std::fmt;

/// PIIRedactionFormatter replaces the standard Tracing formatter.
/// It intercepts formatting and redacts PII fields.
pub struct PIIRedactionFormatter;

impl<S, N> FormatEvent<S, N> for PIIRedactionFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let meta = event.metadata();
        write!(
            writer,
            "{} {}: ",
            meta.level(),
            meta.target()
        )?;

        let mut visitor = PIIRedactionVisitor::new(&mut writer);
        event.record(&mut visitor);

        writeln!(writer)
    }
}

struct PIIRedactionVisitor<'a, 'b> {
    writer: &'a mut Writer<'b>,
    is_first: bool,
}

impl<'a, 'b> PIIRedactionVisitor<'a, 'b> {
    fn new(writer: &'a mut Writer<'b>) -> Self {
        Self {
            writer,
            is_first: true,
        }
    }

    fn write_field_name(&mut self, field: &Field) -> fmt::Result {
        if !self.is_first {
            write!(self.writer, " ")?;
        }
        self.is_first = false;
        write!(self.writer, "{}=", field.name())
    }
}

impl<'a, 'b> Visit for PIIRedactionVisitor<'a, 'b> {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if let Err(_) = self.write_field_name(field) {
            return;
        }
        if super::is_sensitive_key(field.name()) {
            let _ = write!(self.writer, "\"[REDACTED]\"");
        } else {
            let _ = write!(self.writer, "{:?}", value);
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if let Err(_) = self.write_field_name(field) {
            return;
        }
        if super::is_sensitive_key(field.name()) {
            let _ = write!(self.writer, "\"[REDACTED]\"");
        } else if super::is_email(value) {
            let _ = write!(self.writer, "\"[EMAIL_REDACTED]\"");
        } else {
            let _ = write!(self.writer, "\"{}\"", value);
        }
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        if let Err(_) = self.write_field_name(field) { return; }
        if super::is_sensitive_key(field.name()) {
            let _ = write!(self.writer, "\"[REDACTED]\"");
        } else {
            let _ = write!(self.writer, "{}", value);
        }
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        if let Err(_) = self.write_field_name(field) { return; }
        if super::is_sensitive_key(field.name()) {
            let _ = write!(self.writer, "\"[REDACTED]\"");
        } else {
            let _ = write!(self.writer, "{}", value);
        }
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        if let Err(_) = self.write_field_name(field) { return; }
        if super::is_sensitive_key(field.name()) {
            let _ = write!(self.writer, "\"[REDACTED]\"");
        } else {
            let _ = write!(self.writer, "{}", value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_core::dispatcher::DefaultGuard;
    use tracing_core::dispatcher;
    use tracing_subscriber::fmt::MakeWriter;
    use std::sync::{Arc, Mutex};
    use tracing::{info, debug};

    #[derive(Clone)]
    struct MockWriter {
        buf: Arc<Mutex<Vec<u8>>>,
    }

    impl MockWriter {
        fn new() -> Self {
            Self {
                buf: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn get_string(&self) -> String {
            let b = self.buf.lock().unwrap();
            String::from_utf8(b.clone()).unwrap()
        }
    }

    impl std::io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buf.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<'a> MakeWriter<'a> for MockWriter {
        type Writer = MockWriter;
        fn make_writer(&'a self) -> Self::Writer {
            self.clone()
        }
    }

    fn init_test_subscriber() -> (MockWriter, DefaultGuard) {
        let writer = MockWriter::new();
        let subscriber = tracing_subscriber::fmt()
            .event_format(PIIRedactionFormatter)
            .with_writer(writer.clone())
            .finish();
        let guard = dispatcher::set_default(&tracing_core::Dispatch::new(subscriber));
        (writer, guard)
    }

    #[test]
    fn test_pii_redaction_password() {
        let (writer, _guard) = init_test_subscriber();
        let my_password = "super_secret_password_123";
        let non_sensitive = "just_some_value";
        info!(password = %my_password, safe_field = %non_sensitive, "Logging a user attempt");
        let output = writer.get_string();

        assert!(!output.contains("super_secret_password_123"));
        assert!(output.contains("password=\"[REDACTED]\""));
        assert!(output.contains("safe_field=\"just_some_value\""));
    }

    #[test]
    fn test_pii_redaction_email() {
        let (writer, _guard) = init_test_subscriber();
        let user_email = "test.user@openclaw.com";
        info!(contact = %user_email, "Contacting user");
        let output = writer.get_string();

        // We matched on email content string
        assert!(!output.contains("test.user@openclaw.com"));
        assert!(output.contains("contact=\"[EMAIL_REDACTED]\""));
    }

    #[test]
    fn test_pii_redaction_credit_card() {
        let (writer, _guard) = init_test_subscriber();
        let cc = "4111-1111-1111-1111";
        info!(credit_card = %cc, "Processing payment");
        let output = writer.get_string();

        assert!(!output.contains("4111-1111-1111-1111"));
        assert!(output.contains("credit_card=\"[REDACTED]\""));
    }

    #[test]
    fn test_pii_redaction_integer_ssn() {
        let (writer, _guard) = init_test_subscriber();
        let ssn: u64 = 123456789;
        info!(ssn = ssn, "Logging SSN");
        let output = writer.get_string();

        assert!(!output.contains("123456789"));
        assert!(output.contains("ssn=\"[REDACTED]\""));
    }

    #[test]
    fn test_pii_redaction_multitenant_scope() {
        let (writer, _guard) = init_test_subscriber();
        let tenant_id = "tenant_xyz_123";
        let organization_id = "org_abc_456";
        info!(tenant_id = %tenant_id, organization_id = %organization_id, "Tenant access");
        let output = writer.get_string();

        assert!(!output.contains("tenant_xyz_123"));
        assert!(!output.contains("org_abc_456"));
        assert!(output.contains("tenant_id=\"[REDACTED]\""));
        assert!(output.contains("organization_id=\"[REDACTED]\""));
    }

    #[test]
    fn test_pii_redaction_mac_address() {
        let (writer, _guard) = init_test_subscriber();
        let mac = "00:1B:44:11:3A:B7";
        info!(mac_address = %mac, "Network interface connected");
        let output = writer.get_string();

        assert!(!output.contains("00:1B:44:11:3A:B7"));
        assert!(output.contains("mac_address=\"[REDACTED]\""));
    }

    #[test]
    fn test_pii_redaction_various_field_types() {
        let (writer, _guard) = init_test_subscriber();
        let api_key = "sk-1234567890";
        let is_admin = true;
        info!(api_key = %api_key, admin = is_admin, "Admin login");
        let output = writer.get_string();

        assert!(!output.contains("sk-1234567890"));
        assert!(output.contains("api_key=\"[REDACTED]\""));
        assert!(output.contains("admin=true")); // not redacted
    }
}
