pub mod models;

#[cfg(test)]
pub mod migrations_test {
    include!("migrations/tenant_isolation_test.rs");
}
