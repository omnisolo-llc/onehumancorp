#include <cstddef>
#include <cstdint>
#include <string>

#include "srcs/server/agents/builtin/llm_parsing.h"

extern "C" int LLVMFuzzerTestOneInput(const uint8_t* data, size_t size) {
  const std::string body(reinterpret_cast<const char*>(data), size);
  static_cast<void>(ohc::agent::ParseOpenAIChatResponse(body));
  static_cast<void>(ohc::agent::ParseAnthropicChatResponse(body));
  static_cast<void>(ohc::agent::ParseOllamaChatResponse(body));
  return 0;
}