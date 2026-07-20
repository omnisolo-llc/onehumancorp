use tonic::{Request, Status};
use x509_parser::extensions::GeneralName;
use x509_parser::parse_x509_certificate;

pub fn authenticated_spiffe_id(
    standalone: bool,
    claimed_identity: Option<&str>,
    peer_certificate_der: Option<&[u8]>,
) -> Result<String, Status> {
    if standalone {
        let identity = claimed_identity
            .ok_or_else(|| Status::unauthenticated("missing x-spiffe-id header"))?;
        crate::parse_spiffe_id(identity)
            .map_err(|_| Status::unauthenticated("invalid SPIFFE identity"))?;
        return Ok(identity.to_string());
    }

    let certificate = peer_certificate_der
        .ok_or_else(|| Status::unauthenticated("verified client certificate is required"))?;
    spiffe_id_from_certificate_der(certificate)
}

pub fn authenticate_spiffe_request<T>(
    request: &mut Request<T>,
    standalone: bool,
) -> Result<(), Status> {
    request.extensions_mut().remove::<crate::AuthInfo>();
    request
        .extensions_mut()
        .remove::<crate::orchestration::AuthInfo>();

    let claimed_identity = request
        .metadata()
        .get("x-spiffe-id")
        .map(|value| {
            value
                .to_str()
                .map(str::to_string)
                .map_err(|_| Status::unauthenticated("invalid x-spiffe-id header"))
        })
        .transpose()?;
    let peer_certificates = request.extensions().get::<std::sync::Arc<Vec<tonic::transport::Certificate>>>().cloned();
    let peer_certificate = peer_certificates
        .as_deref()
        .and_then(|certificates: &Vec<tonic::transport::Certificate>| certificates.first())
        .map(|c| c.get_ref());
    let identity =
        authenticated_spiffe_id(standalone, claimed_identity.as_deref(), peer_certificate)?;
    let (org_id, agent_id) = crate::parse_spiffe_id(&identity)?;
    let metadata_identity = identity
        .parse()
        .map_err(|_| Status::internal("verified SPIFFE identity is not valid metadata"))?;

    request
        .metadata_mut()
        .insert("x-spiffe-id", metadata_identity);
    request.extensions_mut().insert(crate::AuthInfo {
        spiffe_id: identity.clone(),
        org_id: org_id.clone(),
        agent_id: agent_id.clone(),
    });
    request
        .extensions_mut()
        .insert(crate::orchestration::AuthInfo {
            spiffe_id: identity,
            org_id,
            agent_id,
        });
    Ok(())
}

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
    crate::parse_spiffe_id(identity)
        .map_err(|_| Status::unauthenticated("invalid SPIFFE identity"))?;
    Ok((*identity).to_string())
}

#[cfg(test)]
mod tests {
    use super::{authenticated_spiffe_id, spiffe_id_from_certificate_der};
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

    #[test]
    fn cloud_authentication_uses_only_the_verified_certificate_identity() {
        let verified = "spiffe://onehumancorp.io/org/acme/agent/worker-1";
        let claimed = "spiffe://onehumancorp.io/org/other/agent/forged";
        let certificate = certificate_with_uris(&[verified]);

        assert_eq!(
            authenticated_spiffe_id(false, Some(claimed), Some(&certificate)).unwrap(),
            verified,
        );
        assert!(authenticated_spiffe_id(false, Some(verified), None).is_err());

        let no_spiffe = certificate_with_uris(&[]);
        assert!(authenticated_spiffe_id(false, None, Some(&no_spiffe)).is_err());
    }
}
