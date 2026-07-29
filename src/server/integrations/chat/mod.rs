pub mod pb {
    tonic::include_proto!("ohc.chat");
}

pub mod service;
#[cfg(test)]
pub mod service_test;

pub mod webhook;
#[cfg(test)]
pub mod webhook_test;
