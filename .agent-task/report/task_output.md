issue_title: "Implement Built-in Provider using OpenRouter"
issue_description: |
  **Problem Statement**:
  The application currently lacks a robust and flexible built-in AI provider that can route requests to various models dynamically. Owners and operators need a reliable AI assistant that works out of the box without requiring complex configuration of individual API keys for every possible model they might want to use. The current setup is either tightly coupled to specific providers or requires manual key management for each, creating friction for non-technical users.

  **Research Report**:
  - The existing codebase has configurations for specific models like Gemini, MiniMax, Anthropic, and OpenAI.
  - The `OHC_LLM_PROVIDER` environment variable indicates an intent to support an `openai-compatible` provider.
  - OpenRouter is an excellent solution for this as it provides a single unified API to access numerous models (OpenAI, Anthropic, Google, Meta, etc.) using the OpenAI compatible endpoint format.
  - Using OpenRouter as the primary `openai-compatible` provider or adding explicit support for it will give owners access to the best models without managing multiple accounts and keys.
  - The repository has a `src/server/agents` and `src/server/services` directory which handle LLM integrations. We need a flexible built-in provider that acts as a router.

  **Design Doc**:
  - **Architecture diagram (Mermaid.js)**:
    ```mermaid
    graph TD
        Client[OHC App / Client] --> API[OHC API Server (Go)]
        API --> AgentRunner[Agent Orchestrator]
        AgentRunner --> ProviderManager[Provider Manager]
        ProviderManager -- "If OpenRouter/OpenAI-compatible" --> OpenRouter[OpenRouter API]
        OpenRouter --> Models[GPT-4, Claude 3, Llama 3, etc.]
    ```
  - **Mobile UX flow**:
    - The owner opens the app (375px viewport).
    - The app automatically connects to the built-in AI assistant powered by the unified provider.
    - No setup screen is required for the AI to start functioning. Advanced settings are hidden under an "Advanced Settings" switch.
  - **AI agent integration points**:
    - The `AgentRunner` needs to be able to instantiate a client that uses the generic OpenRouter/OpenAI compatible endpoint.
    - Context and memory must be passed cleanly through this unified interface.
  - **Key design decisions and why**:
    - Use the existing `openai-compatible` configuration path if possible, or add explicit OpenRouter support to ensure maximum compatibility.
    - OpenRouter uses the OpenAI SDK/API format, making it easy to integrate with existing OpenAI clients in the Go codebase.
    - Hide API key configuration behind an advanced settings toggle to maintain the "Grandmother test".

  **Implementation Prompt**:
  Implement a unified built-in AI provider that leverages OpenRouter (or a generic OpenAI-compatible interface configured for OpenRouter). This provider should be the default when starting the application without specific provider configurations, ensuring the AI assistant is immediately available.

  Acceptance Criteria:
  - The system can communicate with multiple different models through a single generic provider interface.
  - The implementation must include 100% unit test coverage for the new or modified provider logic.
  - At least one E2E test using Playwright must verify that the AI assistant can successfully answer a query using the newly configured provider.
  - Ensure the configuration for this provider is clearly documented in the README.

  **Estimated Scope**: Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
