#pragma once

#include "srcs/server/agents/builtin/types.h"

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"

namespace ohc::agent {

absl::StatusOr<ChatResponse> ParseOpenAIChatResponse(absl::string_view body);
absl::StatusOr<ChatResponse> ParseAnthropicChatResponse(absl::string_view body);
absl::StatusOr<ChatResponse> ParseOllamaChatResponse(absl::string_view body);

}  // namespace ohc::agent