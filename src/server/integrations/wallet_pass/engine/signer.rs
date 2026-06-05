use std::collections::HashMap;
use sha1::{Sha1, Digest};
use openssl::pkcs7::{Pkcs7, Pkcs7Flags};
use openssl::pkey::PKey;
use openssl::x509::X509;
use openssl::stack::Stack;
use zip::ZipWriter;
use zip::write::FileOptions;
use std::io::Write;

pub struct PassSigner {
    cert_pem: String,
    key_pem: String,
    wwdr_pem: String,
}

impl PassSigner {
    pub fn new(cert_pem: String, key_pem: String, wwdr_pem: String) -> Self {
        Self { cert_pem, key_pem, wwdr_pem }
    }

    pub fn sign_and_zip(&self, files: HashMap<String, Vec<u8>>) -> Result<Vec<u8>, String> {
        // 1. Generate Manifest
        let mut manifest: HashMap<String, String> = HashMap::new();
        for (filename, content) in &files {
            let mut hasher = Sha1::new();
            hasher.update(content);
            let hash = hasher.finalize();
            manifest.insert(filename.clone(), hex::encode(hash));
        }

        let manifest_json = serde_json::to_string(&manifest).map_err(|e| e.to_string())?;

        // 2. Generate PKCS7 Signature
        let cert = X509::from_pem(self.cert_pem.as_bytes()).map_err(|e| e.to_string())?;
        let pkey = PKey::private_key_from_pem(self.key_pem.as_bytes()).map_err(|e| e.to_string())?;
        let wwdr_cert = X509::from_pem(self.wwdr_pem.as_bytes()).map_err(|e| e.to_string())?;

        let mut certs = Stack::new().map_err(|e| e.to_string())?;
        certs.push(wwdr_cert).map_err(|e| e.to_string())?;

        let pkcs7 = Pkcs7::sign(
            &cert,
            &pkey,
            &certs,
            manifest_json.as_bytes(),
            Pkcs7Flags::DETACHED | Pkcs7Flags::BINARY,
        ).map_err(|e| e.to_string())?;

        let signature = pkcs7.to_der().map_err(|e| e.to_string())?;

        // 3. Create Zip
        let mut zip_buf = Vec::new();
        let mut zip = ZipWriter::new(std::io::Cursor::new(&mut zip_buf));
        let options: FileOptions<()> = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        for (filename, content) in &files {
            zip.start_file(filename, options).map_err(|e| e.to_string())?;
            zip.write_all(content).map_err(|e| e.to_string())?;
        }

        zip.start_file("manifest.json", options).map_err(|e| e.to_string())?;
        zip.write_all(manifest_json.as_bytes()).map_err(|e| e.to_string())?;

        zip.start_file("signature", options).map_err(|e| e.to_string())?;
        zip.write_all(&signature).map_err(|e| e.to_string())?;

        zip.finish().map_err(|e| e.to_string())?;

        Ok(zip_buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openssl::rsa::Rsa;

    #[test]
    fn test_signer_manifest_and_zip() {
        // Generate test keys and certs to verify the struct functions
        let rsa = Rsa::generate(2048).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();

        let mut x509_name = openssl::x509::X509NameBuilder::new().unwrap();
        x509_name.append_entry_by_text("CN", "Test Cert").unwrap();
        let x509_name = x509_name.build();

        let mut cert_builder = openssl::x509::X509::builder().unwrap();
        cert_builder.set_version(2).unwrap();
        cert_builder.set_subject_name(&x509_name).unwrap();
        cert_builder.set_issuer_name(&x509_name).unwrap();
        cert_builder.set_pubkey(&pkey).unwrap();
        let not_before = openssl::asn1::Asn1Time::days_from_now(0).unwrap();
        let not_after = openssl::asn1::Asn1Time::days_from_now(365).unwrap();
        cert_builder.set_not_before(&not_before).unwrap();
        cert_builder.set_not_after(&not_after).unwrap();
        cert_builder.sign(&pkey, openssl::hash::MessageDigest::sha256()).unwrap();
        let cert = cert_builder.build();

        let cert_pem = String::from_utf8(cert.to_pem().unwrap()).unwrap();
        let key_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8().unwrap()).unwrap();

        // Use the same cert as WWDR for testing
        let wwdr_pem = cert_pem.clone();

        let signer = PassSigner::new(cert_pem, key_pem, wwdr_pem);

        let mut files = HashMap::new();
        files.insert("pass.json".to_string(), b"{\"test\": 123}".to_vec());
        files.insert("icon.png".to_string(), b"fake png data".to_vec());

        let result = signer.sign_and_zip(files);
        assert!(result.is_ok(), "Failed to sign and zip: {:?}", result.err());

        let zip_data = result.unwrap();
        assert!(zip_data.len() > 0);

        // Verify zip contents
        let reader = std::io::Cursor::new(zip_data);
        let mut zip = zip::ZipArchive::new(reader).unwrap();

        assert!(zip.by_name("pass.json").is_ok());
        assert!(zip.by_name("icon.png").is_ok());
        assert!(zip.by_name("manifest.json").is_ok());
        assert!(zip.by_name("signature").is_ok());
    }
}
