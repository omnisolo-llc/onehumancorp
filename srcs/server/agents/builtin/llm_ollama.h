#pragma once

#include <string>

#include "srcs/server/agents/builtin/llm_client.h"
#include "absl/status/statusor.h"

namespace ohc::agent {

// LLMClient implementation for a locally-running Ollama server.
// This is the preferred backend for Raspberry Pi / embedded deployments
// because it incurs zero cloud-API cost and zero egress latency.
class OllamaClient final : public LLMClient {
 public:
  // endpoint: full URL to the Ollama chat endpoint.
  //           Default: http://localhost:11434/api/chat
  explicit OllamaClient(
      std::string endpoint = "http://localhost:11434/api/chat");

  absl::StatusOr<ChatResponse> Chat(const ChatRequest& req) override;

 private:
  std::string endpoint_;
};

}  // namespace ohc::agent
