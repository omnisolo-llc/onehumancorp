# Native structured-output schemas

`parse_structured_output<T>` and `RetryWithErrorOutputParser<T>` require
`schemars::JsonSchema` alongside the existing deserialization bounds. Concrete
result DTOs derive the matching Schemars 1.2.1 trait. Serde remains responsible
for accepting or rejecting returned data; the schema is a description sent to
the model, not a replacement runtime validator or domain policy.

The parser generates a complete object envelope with a required `data` field.
Its schema describes the actual `T`, including array results, required/defaulted
fields, Serde names and enum representations. It explicitly uses Draft 7 and the
deserialization contract, inlines nonrecursive subschemas and keeps any recursive
references rooted in the complete envelope. A nullable result still requires an
explicit `data` key. Arbitrary `serde_json::Value` remains unconstrained JSON.

`structured_output` is a reserved tool name. One exactly matching generated
definition is retained. A different description/schema or multiple same-name
definitions fails before a model request. Unrelated tools are retained. The
definition is generated once and reused across the existing correction loop.

Only native tool calls with the required data key are accepted by this parser.
Plain text and Markdown do not bypass it. The existing retry cap, semantic/custom
parser checks, judge confidence decisions and downstream tool authority remain
unchanged. The initial agent text-planning path is a separate caller behavior.
The two judge result DTOs intentionally keep their different list defaults.

No provider strict-mode flag is enabled. OpenAI-compatible and Anthropic request
adapters carry the schema through their existing tool transports. Local request
fixtures demonstrate adapter parity; they do not certify every model's supported
JSON Schema subset. Gemini/Ollama native tool support is not introduced here.

Relevant native tests live in `output_parser_schema_test.rs`, the real planner,
agent fallback and judge callers, and the LLM adapters. Full repository acceptance
still uses the existing `make lint` and `make test` gates. No telemetry exporter,
billing ledger or provider credential/configuration change is part of this path.
