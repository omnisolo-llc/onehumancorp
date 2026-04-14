package agentgrpc

import (
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
)

// newLLMClient selects and constructs the appropriate LLM client based on cfg.
// Priority: Anthropic → OpenAI → Ollama.
func newLLMClient(cfg AgentConfig) (builtin.LLMClient, error) {
	provider := cfg.LLMProvider
	if provider == "" {
		// Auto-detect from environment variables.
		if os.Getenv("ANTHROPIC_API_KEY") != "" {
			provider = "anthropic"
		} else if os.Getenv("OPENAI_API_KEY") != "" {
			provider = "openai"
		} else {
			provider = "ollama"
		}
	}

	switch provider {
	case "anthropic":
		key := os.Getenv("ANTHROPIC_API_KEY")
		if key == "" {
			return nil, fmt.Errorf("ANTHROPIC_API_KEY is required for provider %q", provider)
		}
		return builtin.NewAnthropicClient(key), nil
	case "openai":
		key := os.Getenv("OPENAI_API_KEY")
		if key == "" {
			return nil, fmt.Errorf("OPENAI_API_KEY is required for provider %q", provider)
		}
		return builtin.NewOpenAIClient(key), nil
	case "ollama":
		endpoint := cfg.LLMEndpoint
		if endpoint == "" {
			endpoint = os.Getenv("OHC_LOCAL_LLM_ENDPOINT")
		}
		return builtin.NewOllamaClient(endpoint), nil
	default:
		return nil, fmt.Errorf("unknown LLM provider %q", provider)
	}
}
