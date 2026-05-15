pub mod hybridfsmcp;
pub mod ingest_arch;
pub mod local_proxy;
pub mod fsmcp;

// Strict Input Validation for Agent Tools
// When an LLM decides to use a tool (e.g., `send_email`), we cannot blindly trust
// the parameters it provides. Hallucinations or prompt injections could lead to
// disastrous side effects (like emailing the entire customer database).
//
// Every tool in this module implements a `validate_input` method.
// - It ensures the requested action is logically scoped to the current `tenant_id`.
// - It checks against hardcoded blacklists (e.g., cannot refund more than $500
//   without human approval).
// - It sanitizes text inputs to prevent XSS or SQL injection if the output is
//   later rendered in the UI or saved to the DB.
