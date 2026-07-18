use server_ohc::orchestration::{
    Agent, RegisterAgentRequest, hub_service_client::HubServiceClient,
};
use std::{env, fs, time::Duration};
use tonic::transport::{Certificate, ClientTlsConfig, Endpoint, Identity};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let endpoint = args.next().ok_or("missing endpoint")?;
    let ca_path = args.next().ok_or("missing CA path")?;
    let cert_path = args.next().ok_or("missing client certificate path")?;
    let key_path = args.next().ok_or("missing client key path")?;
    let expected = args.next().unwrap_or_else(|| "success".to_string());
    let organization_id = args.next().unwrap_or_else(|| "e2e-org".to_string());
    if args.next().is_some()
        || !matches!(
            expected.as_str(),
            "success" | "unauthenticated" | "tls-rejected"
        )
    {
        return Err("usage: grpc_mtls_probe ENDPOINT CA CERT KEY [success|unauthenticated|tls-rejected] [organization-id] (use '-' for CERT and KEY when expecting tls-rejected)".into());
    }

    let mut tls = ClientTlsConfig::new()
        .domain_name("localhost")
        .ca_certificate(Certificate::from_pem(fs::read(ca_path)?));
    if cert_path == "-" || key_path == "-" {
        if expected != "tls-rejected" || cert_path != "-" || key_path != "-" {
            return Err("CERT and KEY must both be '-' only when expecting tls-rejected".into());
        }
    } else {
        tls = tls.identity(Identity::from_pem(
            fs::read(cert_path)?,
            fs::read(key_path)?,
        ));
    }
    let channel = Endpoint::from_shared(endpoint)?
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .tls_config(tls)?
        .connect()
        .await;
    if expected == "tls-rejected" {
        return match channel {
            Err(_) => Ok(()),
            Ok(channel) => {
                let mut client = HubServiceClient::new(channel);
                match client
                    .register_agent(RegisterAgentRequest {
                        agent: Some(Agent {
                            id: "e2e-client".to_string(),
                            name: "gRPC mTLS E2E probe".to_string(),
                            role: "E2E".to_string(),
                            organization_id,
                            status: "ONLINE".to_string(),
                            provider_type: "test".to_string(),
                        }),
                    })
                    .await
                {
                    Err(_) => Ok(()),
                    Ok(_) => Err("gRPC service accepted a client without a certificate".into()),
                }
            }
        };
    }
    let channel = channel?;
    let mut client = HubServiceClient::new(channel);
    let result = client
        .register_agent(RegisterAgentRequest {
            agent: Some(Agent {
                id: "e2e-client".to_string(),
                name: "gRPC mTLS E2E probe".to_string(),
                role: "E2E".to_string(),
                organization_id,
                status: "ONLINE".to_string(),
                provider_type: "test".to_string(),
            }),
        })
        .await;

    match (expected.as_str(), result) {
        ("success", Ok(response)) => {
            if response.into_inner().success {
                Ok(())
            } else {
                Err("gRPC probe returned an unsuccessful response".into())
            }
        }
        ("unauthenticated", Err(status)) if status.code() == tonic::Code::Unauthenticated => Ok(()),
        ("success", Err(status)) => Err(format!("gRPC probe failed: {status}").into()),
        ("unauthenticated", Ok(_)) => {
            Err("gRPC probe accepted a client without a valid SPIFFE identity".into())
        }
        ("unauthenticated", Err(status)) => Err(format!(
            "gRPC probe expected UNAUTHENTICATED, got {}: {}",
            status.code(),
            status.message()
        )
        .into()),
        _ => unreachable!(),
    }
}
