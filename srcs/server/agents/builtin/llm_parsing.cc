#include "srcs/server/agents/builtin/llm_parsing.h"

#include <string>

#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {
namespace {

absl::StatusOr<nlohmann::json> ParseJson(absl::string_view body,
                                         absl::string_view source) {
  const auto parsed =
      nlohmann::json::parse(body.begin(), body.end(), nullptr, false);
  if (parsed.is_discarded()) {
    return absl::InternalError(
        absl::StrCat("Failed to parse ", source, " JSON response"));
  }
  return parsed;
}

}  // namespace

absl::StatusOr<ChatResponse> ParseOpenAIChatResponse(absl::string_view body) {
  auto result = ParseJson(body, "OpenAI");
  if (!result.ok()) {
    return result.status();
  }

  ChatResponse chat_resp;
  chat_resp.message.role = Role::kAssistant;

  const auto& choices = result->value("choices", nlohmann::json::array());
  if (!choices.empty()) {
    const auto& msg = choices[0]["message"];
    if (msg.contains("content") && msg["content"].is_string()) {
      chat_resp.message.content = msg["content"].get<std::string>();
    }
    if (msg.contains("tool_calls")) {
      for (const auto& tc : msg["tool_calls"]) {
        ToolCall call;
        call.id = tc["id"].get<std::string>();
        call.name = tc["function"]["name"].get<std::string>();
        const auto& args_str = tc["function"]["arguments"].get<std::string>();
        call.arguments = nlohmann::json::parse(args_str, nullptr, false);
        chat_resp.message.tool_calls.push_back(std::move(call));
      }
    }
  }

  return chat_resp;
}

absl::StatusOr<ChatResponse> ParseAnthropicChatResponse(absl::string_view body) {
  auto result = ParseJson(body, "Anthropic");
  if (!result.ok()) {
    return result.status();
  }

  ChatResponse chat_resp;
  chat_resp.message.role = Role::kAssistant;

  const auto& blocks = result->value("content", nlohmann::json::array());
  for (const auto& block : blocks) {
    const auto type = block.value("type", std::string{});
    if (type == "text") {
      chat_resp.message.content += block.value("text", std::string{});
    } else if (type == "tool_use") {
      ToolCall call;
      call.id = block.value("id", std::string{});
      call.name = block.value("name", std::string{});
      call.arguments = block.value("input", nlohmann::json{});
      chat_resp.message.tool_calls.push_back(std::move(call));
    }
  }

  return chat_resp;
}

absl::StatusOr<ChatResponse> ParseOllamaChatResponse(absl::string_view body) {
  auto result = ParseJson(body, "Ollama");
  if (!result.ok()) {
    return result.status();
  }

  ChatResponse chat_resp;
  chat_resp.message.role = Role::kAssistant;

  if (result->contains("message")) {
    const auto& msg = (*result)["message"];
    chat_resp.message.content = msg.value("content", std::string{});

    if (msg.contains("tool_calls")) {
      for (const auto& tc : msg["tool_calls"]) {
        ToolCall call;
        if (tc.contains("function")) {
          call.name = tc["function"].value("name", std::string{});
          call.arguments = tc["function"].value("arguments", nlohmann::json{});
        }
        call.id = absl::StrCat(call.name, "_",
                               chat_resp.message.tool_calls.size());
        chat_resp.message.tool_calls.push_back(std::move(call));
      }
    }
  }

  return chat_resp;
}

}  // namespace ohc::agent