use sha1::{Sha1, Digest};
use std::io::Write;
use zip::write::FileOptions;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PkpassData {
    pub pass_type_identifier: String,
    pub team_identifier: String,
    pub serial_number: String,
    pub organization_name: String,
    pub description: String,
    pub foreground_color: Option<String>,
    pub background_color: Option<String>,
}

pub struct PkpassGenerator {}

impl PkpassGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, data: PkpassData) -> Result<Vec<u8>, String> {
        let mut zip_buffer = Vec::new();
        {
            let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buffer));
            let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

            // pass.json
            let pass_json = serde_json::to_string(&data).map_err(|e| e.to_string())?;
            zip.start_file("pass.json", options).map_err(|e| e.to_string())?;
            zip.write_all(pass_json.as_bytes()).map_err(|e| e.to_string())?;

            // Manifest
            let mut hasher = Sha1::new();
            hasher.update(pass_json.as_bytes());
            let hash = hasher.finalize();
            let manifest = format!("{{\"pass.json\":\"{:x}\"}}", hash);
            zip.start_file("manifest.json", options).map_err(|e| e.to_string())?;
            zip.write_all(manifest.as_bytes()).map_err(|e| e.to_string())?;

            // Signature (Dummy)
            zip.start_file("signature", options).map_err(|e| e.to_string())?;
            // NOTE: PKCS #7 detached signature is required for actual Apple Wallet validation.
            // Here we provide a mock signature for the engine skeleton to be replaced by the KMS integration.
            zip.write_all(b"dummy_signature").map_err(|e| e.to_string())?;

            zip.finish().map_err(|e| e.to_string())?;
        }
        Ok(zip_buffer)
    }
}
