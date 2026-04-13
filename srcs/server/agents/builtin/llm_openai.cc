#include "srcs/server/agents/builtin/llm_openai.h"

#include <utility>

#include "srcs/server/agents/builtin/http_client.h"
#include "srcs/server/agents/builtin/llm_parsing.h"
#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

OpenAIClient::OpenAIClient(std::string api_key, std::string base_url)
    : api_key_(std::move(api_key)), base_url_(std::move(base_url)) {}

absl::StatusOr<ChatResponse> OpenAIClient::Chat(const ChatRequest& req) {
  // ---- Build messages array ------------------------------------------------
  nlohmann::json messages = nlohmann::json::array();

  if (!req.system.empty()) {
    messages.push_back({{"role", "system"}, {"content", req.system}});
  }

  for (const auto& m : req.messages) {
    const std::string role(RoleToStringView(m.role));

    // Tool-result messages: OpenAI expects one message per result.
    if (m.role == Role::kTool) {
      for (const auto& tr : m.tool_results) {
        messages.push_back({
            {"role",         "tool"},
            {"tool_call_id", tr.tool_call_id},
            {"content",      tr.content.empty() ? tr.error : tr.content},
        });
      }
      continue;
    }

    nlohmann::json msg = {{"role", role}, {"content", m.content}};

    if (!m.tool_calls.empty()) {
      nlohmann::json tcs = nlohmann::json::array();
      for (const auto& tc : m.tool_calls) {
        tcs.push_back({
            {"id",   tc.id},
            {"type", "function"},
            {"function", {{"name", tc.name},
                          {"arguments", tc.arguments.dump()}}},
        });
      }
      msg["tool_calls"] = std::move(tcs);
    }
    messages.push_back(std::move(msg));
  }

  // ---- Build payload -------------------------------------------------------
  nlohmann::json payload;
  payload["model"]    = req.model;
  payload["messages"] = std::move(messages);
  if (req.max_tokens > 0) payload["max_tokens"]  = req.max_tokens;
  if (req.temperature >= 0.0f) payload["temperature"] = req.temperature;

  if (!req.tool_definitions.empty()) {
    nlohmann::json tools = nlohmann::json::array();
    for (const auto& td : req.tool_definitions) {
      tools.push_back({
          {"type", "function"},
          {"function", {{"name",        td.name},
                        {"description", td.description},
                        {"parameters",  td.parameters}}},
      });
    }
    payload["tools"] = std::move(tools);
  }

  // ---- HTTP POST -----------------------------------------------------------
  HttpRequest http_req;
  http_req.url     = absl::StrCat(base_url_, "/chat/completions");
  http_req.method  = "POST";
  http_req.body    = payload.dump();
  http_req.headers = {{"Authorization", absl::StrCat("Bearer ", api_key_)},
                      {"Content-Type",  "application/json"}};

  auto resp_or = HttpDo(http_req);
  if (!resp_or.ok()) return resp_or.status();
  if (resp_or->status_code != 200) {
    return absl::InternalError(absl::StrCat(
        "OpenAI API HTTP ", resp_or->status_code, ": ", resp_or->body));
  }

  // ---- Parse response ------------------------------------------------------
  return ParseOpenAIChatResponse(resp_or->body);
}

}  // namespace ohc::agent
