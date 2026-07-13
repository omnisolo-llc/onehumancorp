use tonic::Status;
use x509_parser::extensions::GeneralName;
use x509_parser::parse_x509_certificate;

pub fn spiffe_id_from_certificate_der(der: &[u8]) -> Result<String, Status> {
    let (_, certificate) = parse_x509_certificate(der)
        .map_err(|_| Status::unauthenticated("invalid peer certificate"))?;
    let alternative_names = certificate
        .subject_alternative_name()
        .map_err(|_| Status::unauthenticated("invalid peer certificate extensions"))?
        .ok_or_else(|| {
            Status::unauthenticated("peer certificate has no subject alternative name")
        })?;

    let identities: Vec<&str> = alternative_names
        .value
        .general_names
        .iter()
        .filter_map(|name| match name {
            GeneralName::URI(uri) if uri.starts_with("spiffe://") => Some(*uri),
            _ => None,
        })
        .collect();
    let [identity] = identities.as_slice() else {
        return Err(Status::unauthenticated(
            "peer certificate must contain exactly one SPIFFE URI",
        ));
    };
    crate::parse_spiffe_id(identity)?;
    Ok((*identity).to_string())
}

#[cfg(test)]
mod tests {
    use super::spiffe_id_from_certificate_der;
    use rcgen::string::Ia5String;
    use rcgen::{CertificateParams, KeyPair, SanType};

    fn certificate_with_uris(uris: &[&str]) -> Vec<u8> {
        let mut params = CertificateParams::new(Vec::<String>::new()).unwrap();
        for uri in uris {
            params
                .subject_alt_names
                .push(SanType::URI(Ia5String::try_from(*uri).unwrap()));
        }
        let key = KeyPair::generate().unwrap();
        params.self_signed(&key).unwrap().der().to_vec()
    }

    #[test]
    fn extracts_one_strict_spiffe_uri_san() {
        let expected = "spiffe://onehumancorp.io/org/acme/agent/worker-1";
        let der = certificate_with_uris(&[expected]);
        assert_eq!(spiffe_id_from_certificate_der(&der).unwrap(), expected);
    }

    #[test]
    fn rejects_certificates_without_spiffe_uri_sans() {
        let der = certificate_with_uris(&[]);
        assert!(spiffe_id_from_certificate_der(&der).is_err());
        assert!(spiffe_id_from_certificate_der(b"not a certificate").is_err());
    }

    #[test]
    fn rejects_ambiguous_or_untrusted_spiffe_uri_sans() {
        let ambiguous = certificate_with_uris(&[
            "spiffe://onehumancorp.io/org/acme/agent/worker-1",
            "spiffe://onehumancorp.io/org/acme/agent/worker-2",
        ]);
        assert!(spiffe_id_from_certificate_der(&ambiguous).is_err());

        let untrusted = certificate_with_uris(&["spiffe://evil.example/org/acme/agent/worker-1"]);
        assert!(spiffe_id_from_certificate_der(&untrusted).is_err());
    }
}
