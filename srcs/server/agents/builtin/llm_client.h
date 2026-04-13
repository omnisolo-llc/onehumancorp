#pragma once

// Abstract interface for LLM backends (OpenAI, Anthropic, Ollama, …).
// Implementations are in llm_openai.h, llm_anthropic.h, llm_ollama.h.

#include "srcs/server/agents/builtin/types.h"
#include "absl/status/statusor.h"

namespace ohc::agent {

class LLMClient {
 public:
  virtual ~LLMClient() = default;

  // Sends a chat request to the LLM and returns the response.
  // Thread-safe implementations are encouraged but not required by this
  // interface.
  virtual absl::StatusOr<ChatResponse> Chat(const ChatRequest& req) = 0;
};

}  // namespace ohc::agent
