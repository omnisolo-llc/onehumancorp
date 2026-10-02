# Provider generation contracts

Run `bash scripts/minimax-provider-contract/run.sh` from a checkout with its
locked Cargo dependencies available. This focused crate imports the complete
production `minimax.rs` module and its `local_generation.rs` child. Regression
fixtures are exclusively test-only loopback HTTP servers; they never call a
live provider or require provider credentials. Compilation uses the repository
lockfile's registry versions and reuses the existing development dependency graph.

The runner checks registry dependency identity and binds the result to an
unchanged Rust-source fingerprint. Use the existing coordinated Cargo target
and one compiler job on a disk-constrained host. This contract check does not
replace `make lint`, `make test`, or live-provider verification.

MiniMax chat uses the [official OpenAI-compatible endpoint](https://platform.minimax.io/docs/api-reference/text-chat-openai).
The legacy embedding transport retains its existing endpoint and `embo-01`
wire contract. Loopback verification establishes request/response handling,
not current availability of that legacy service.

## Transport behavior

A text or embedding call now dispatches at most one HTTP request. The older
implicit three-attempt loops are removed: a timeout, interrupted response or
HTTP 5xx can follow already accepted inference. An error remains an error;
callers must reconcile uncertain outcomes and explicitly request any new
attempt. Existing completed-response caching and request deduplication remain.

Nonstream response bodies are capped at 10 MiB before JSON decoding; completed
text is capped at 8 MiB. Embedding input is nonempty and at most 1 MiB, a vector
contains at most 65,536 finite values, and a single MiniMax input must yield
exactly one vector. Streaming preserves UTF-8 and SSE framing across transport
chunks, accepts one choice sequence, and requires successful completion plus
`[DONE]`; provider errors, late choice data and truncated streams fail closed.

Loopback outages never manufacture reports or usage. Provider-reported local
input/output quantities remain optional, and absent usage/duration stays
unknown. No live MiniMax/Ollama availability, tenant budget enforcement or
caller-level fallback behavior is certified by this focused adapter suite.
