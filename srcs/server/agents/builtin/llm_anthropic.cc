#include "srcs/server/agents/builtin/llm_anthropic.h"

#include <utility>

#include "srcs/server/agents/builtin/http_client.h"
#include "srcs/server/agents/builtin/llm_parsing.h"
#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

AnthropicClient::AnthropicClient(std::string api_key,
                                 std::string api_version,
                                 std::string endpoint)
    : api_key_(std::move(api_key)),
      api_version_(std::move(api_version)),
      endpoint_(std::move(endpoint)) {}

absl::StatusOr<ChatResponse> AnthropicClient::Chat(const ChatRequest& req) {
  // ---- Build messages array (Anthropic format) ----------------------------
  // Anthropic requires alternating user/assistant turns and treats tool calls
  // as content blocks.  For this simplified mapping we flatten everything into
  // text content so it works with the current tool schema.
  nlohmann::json messages = nlohmann::json::array();

  for (const auto& m : req.messages) {
    if (m.role == Role::kSystem) continue;  // Handled via top-level "system".

    std::string role(RoleToStringView(m.role));
    // Tool results travel back as a "user" role in Anthropic's format.
    if (m.role == Role::kTool) role = "user";

    nlohmann::json content = nlohmann::json::array();

    // Textual content block.
    if (!m.content.empty()) {
      content.push_back({{"type", "text"}, {"text", m.content}});
    }

    // Tool-use result blocks.
    for (const auto& tr : m.tool_results) {
      content.push_back({
          {"type",        "tool_result"},
          {"tool_use_id", tr.tool_call_id},
          {"content",     tr.content.empty() ? tr.error : tr.content},
      });
    }

    // Tool-use request blocks (from previous assistant turn).
    for (const auto& tc : m.tool_calls) {
      content.push_back({
          {"type",  "tool_use"},
          {"id",    tc.id},
          {"name",  tc.name},
          {"input", tc.arguments},
      });
    }

    if (!content.empty()) {
      messages.push_back({{"role", role}, {"content", std::move(content)}});
    }
  }

  // ---- Build payload -------------------------------------------------------
  nlohmann::json payload;
  payload["model"]      = req.model;
  payload["max_tokens"] = req.max_tokens > 0 ? req.max_tokens : 2048;
  payload["messages"]   = std::move(messages);
  if (!req.system.empty()) payload["system"] = req.system;

  if (!req.tool_definitions.empty()) {
    nlohmann::json tools = nlohmann::json::array();
    for (const auto& td : req.tool_definitions) {
      tools.push_back({
          {"name",         td.name},
          {"description",  td.description},
          {"input_schema", td.parameters},
      });
    }
    payload["tools"] = std::move(tools);
  }

  // ---- HTTP POST -----------------------------------------------------------
  HttpRequest http_req;
  http_req.url    = endpoint_;
  http_req.method = "POST";
  http_req.body   = payload.dump();
  http_req.headers = {
      {"x-api-key",          api_key_},
      {"anthropic-version",  api_version_},
      {"content-type",       "application/json"},
  };

  auto resp_or = HttpDo(http_req);
  if (!resp_or.ok()) return resp_or.status();
  if (resp_or->status_code != 200) {
    return absl::InternalError(absl::StrCat(
        "Anthropic API HTTP ", resp_or->status_code, ": ", resp_or->body));
  }

  // ---- Parse response ------------------------------------------------------
  return ParseAnthropicChatResponse(resp_or->body);
}

}  // namespace ohc::agent
