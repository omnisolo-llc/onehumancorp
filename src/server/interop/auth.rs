
use std::sync::Arc;
use crate::interop::protocol::proto;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub struct InteropAuthenticator {
    secret: Vec<u8>,
}

impl InteropAuthenticator {
    pub fn new(secret: &str) -> Self {
        Self { secret: secret.as_bytes().to_vec() }
    }

    pub fn sign_job(&self, job: &mut proto::JobDispatch) {
        let mut mac = HmacSha256::new_from_slice(&self.secret).unwrap();
        mac.update(job.job_id.as_bytes());
        mac.update(job.tenant_id.as_bytes());

        let result = mac.finalize().into_bytes();
        let mut signed_payload = result.to_vec();
        signed_payload.extend_from_slice(&job.payload);
        job.payload = signed_payload;
    }

    pub fn verify_job(&self, job: &proto::JobDispatch) -> bool {
        if job.payload.len() < 32 {
            return false;
        }

        let (signature, actual_payload) = job.payload.split_at(32);
        let mut mac = HmacSha256::new_from_slice(&self.secret).unwrap();
        mac.update(job.job_id.as_bytes());
        mac.update(job.tenant_id.as_bytes());

        mac.verify_slice(signature).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_validator() {
        let auth = InteropAuthenticator::new("super_secret_mesh_key");

        let mut job = proto::JobDispatch {
            job_id: "job_xyz".to_string(),
            tenant_id: "tenant_abc".to_string(),
            action_name: "do_work".to_string(),
            payload: vec![1, 2, 3, 4],
            timestamp_ms: 1000,
        };

        auth.sign_job(&mut job);
        assert!(auth.verify_job(&job));

        let mut tampered_job = job.clone();
        tampered_job.tenant_id = "tenant_hacker".to_string();
        assert!(!auth.verify_job(&tampered_job));

        let mut tampered_sig = job.clone();
        tampered_sig.payload[0] ^= 0xFF;
        assert!(!auth.verify_job(&tampered_sig));
    }
}
