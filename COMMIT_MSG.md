fix: overwrite onboarding state_json instead of string concatenating

The `save_onboarding_state` queries previously used the `||` operator
(`onboarding_state.state_json || EXCLUDED.state_json`), which performs
string concatenation rather than JSON merging on TEXT columns in SQLite/Postgres.
This corrupted the JSON payload. Furthermore, the frontend `save_draft`
action always sends the entire updated state object, making merging on the
backend unnecessary.

This commit updates the queries in `onboarding_agent.rs` and `lib.rs` to
simply overwrite the existing `state_json` with `EXCLUDED.state_json`.