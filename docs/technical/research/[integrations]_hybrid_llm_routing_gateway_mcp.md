<div markdown="1" style="backdrop-filter: blur(20px); background: rgba(255,255,255,0.1); border-radius: 12px; font-family: 'Inter', sans-serif; padding: 20px;">

# Title: [integrations] Hybrid LLM Routing Gateway MCP

## Problem Statement
The OHC Hybrid Agentic OS requires agents to be agnostic to the underlying LLM provider. In Cloud-native mode, LLM requests must be routed through a centralized API Gateway (e.g., LiteLLM) to handle load balancing, API key rotation, cost-tracking, and tenant-based rate limits. However, in Standalone Desktop mode, agents must seamlessly fallback to local inference engines (e.g., Ollama, Llama.cpp) to preserve privacy and function in air-gapped environments. Currently, there is no unified MCP tool that dynamically routes these requests based on the deployment footprint.

## Research Report
Market analysis highlights that frameworks like CrewAI and LangChain often require hardcoding LLM clients or relying on environment variables that do not easily support dynamic switching between a multi-tenant cloud gateway and a local LLM daemon.

### Comparative Table: Cloud vs Standalone LLM Routing
| Feature | Cloud-Native Mode (LiteLLM/Gateway) | Standalone Mode (Ollama/Local) |
| :--- | :--- | :--- |
| **Primary Goal** | High-throughput, multi-tenant cost tracking | Zero-latency, privacy-first offline execution |
| **Latency** | Network bounded | Compute bounded (Local GPU/CPU) |
| **Authentication** | SPIFFE/SPIRE & Kubernetes Secrets | Local OS Keychain / None |
| **Rate Limiting** | Strict tenant quotas | Hardware constrained |

**Recommendation:** Develop a Hybrid LLM Routing Gateway MCP Tool that abstracts the provider logic and routes prompts based on `OHC_MULTITENANT` configurations.

### Architecture Flow
```mermaid
graph TD;
    A[Agent Workspace] -->|MCP Tool Request| B(Hybrid LLM Routing MCP);
    B -->{OHC_MULTITENANT == true?};
    {OHC_MULTITENANT == true?} -- Yes --> C[Cloud API Gateway];
    C --> D[OpenAI/Anthropic APIs];
    {OHC_MULTITENANT == true?} -- No --> E[Local Inference Engine];
    E --> F[Ollama / Llama.cpp];
```

## Design Doc
**Architecture:**
- Create a new package `src/server/lib/integrations/llm_router/`.
- Introduce an `LLMRouterManager` implementing the MCP Tool interface.
- Dynamically route based on `os.Getenv("OHC_MULTITENANT") == "true"`.
- **Cloud Mode:** Utilize an HTTP client configured with Keep-Alive pools to connect to the internal OHC LiteLLM gateway. Enforce tenant ID propagation via headers.
- **Standalone Mode:** Connect directly to `localhost:11434` (Ollama default) or similar local socket for offline inference.

**API Contracts:**
- `GenerateCompletion(ctx async context, prompt string, model string, options map[string]interface{}) (string, error)`
- `GenerateEmbeddings(ctx async context, text string, model string) ([]float32, error)`

**Security:**
- Apply `RedactInterfacePII` to all prompts in Cloud Mode before sending payload to external LLM providers.
- Strip external API tokens from Standalone memory logs.

## Implementation Prompt
"Implement the Hybrid LLM Routing Gateway MCP tool in `src/server/lib/integrations/llm_router/`.
1. Create `llm_router.rs` defining the `LLMRouterManager` and its MCP capabilities (`GenerateCompletion`, `GenerateEmbeddings`).
2. Implement dynamic routing logic based on `os.Getenv(\"OHC_MULTITENANT\") == \"true\"`.
3. For Cloud Mode, implement the HTTP client integrating with the LiteLLM gateway, ensuring tenant headers (`X-Tenant-ID`) are injected.
4. For Standalone Mode, implement integration with the Ollama REST API (`http://localhost:11434/api/generate`).
5. Ensure prompts undergo PII Redaction in Cloud Mode.
6. Create comprehensive tests in `llm_router_test.rs` with mocked HTTP responses for both modes. Ensure 100% test coverage.
7. Update or create the adjacent `BUILD.bazel` file, including the new files in the `srcs` array."

## Priority
P1

## Estimated Scope
Medium

</div>
