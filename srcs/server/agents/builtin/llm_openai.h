#pragma once

#include <string>

#include "srcs/server/agents/builtin/llm_client.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"

namespace ohc::agent {

// LLMClient implementation for the OpenAI Chat Completions API.
// Also compatible with any OpenAI-compatible endpoint (e.g. LM Studio, vLLM).
class OpenAIClient final : public LLMClient {
 public:
  // api_key:  OpenAI secret key.
  // base_url: Override the endpoint root (default: https://api.openai.com/v1).
  explicit OpenAIClient(std::string api_key,
                        std::string base_url = "https://api.openai.com/v1");

  absl::StatusOr<ChatResponse> Chat(const ChatRequest& req) override;

 private:
  std::string api_key_;
  std::string base_url_;
};

}  // namespace ohc::agent
