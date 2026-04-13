#pragma once

#include <string>

#include "srcs/server/agents/builtin/llm_client.h"
#include "absl/status/statusor.h"

namespace ohc::agent {

// LLMClient implementation for the Anthropic Messages API (claude-*).
class AnthropicClient final : public LLMClient {
 public:
  // api_key:        Anthropic API key.
  // api_version:    Anthropic API version header (default: "2023-06-01").
  // endpoint:       Override the messages endpoint for tests or proxies.
  explicit AnthropicClient(
      std::string api_key,
      std::string api_version = "2023-06-01",
      std::string endpoint = "https://api.anthropic.com/v1/messages");

  absl::StatusOr<ChatResponse> Chat(const ChatRequest& req) override;

 private:
  std::string api_key_;
  std::string api_version_;
  std::string endpoint_;
};

}  // namespace ohc::agent
