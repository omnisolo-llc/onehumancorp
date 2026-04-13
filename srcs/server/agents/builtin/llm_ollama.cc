#include "srcs/server/agents/builtin/llm_ollama.h"

#include <utility>

#include "srcs/server/agents/builtin/http_client.h"
#include "srcs/server/agents/builtin/llm_parsing.h"
#include "absl/base/attributes.h"
#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

OllamaClient::OllamaClient(std::string endpoint)
    : endpoint_(std::move(endpoint)) {}

ABSL_ATTRIBUTE_NOINLINE absl::StatusOr<ChatResponse> OllamaClient::Chat(
  const ChatRequest& req) {
  // ---- Build messages array (OpenAI-compatible) ----------------------------
  nlohmann::json messages = nlohmann::json::array();

  if (!req.system.empty()) {
    messages.push_back({{"role", "system"}, {"content", req.system}});
  }

  for (const auto& m : req.messages) {
    const std::string role(RoleToStringView(m.role));
    nlohmann::json content_val = m.content;

    // Tool results: flatten to text for Ollama which may not support native
    // tool-call formats.
    if (m.role == Role::kTool && !m.tool_results.empty()) {
      std::string combined;
      for (const auto& tr : m.tool_results) {
        if (!combined.empty()) combined += "\n";
        combined += tr.content.empty() ? tr.error : tr.content;
      }
      content_val = std::move(combined);
    }

    messages.push_back({{"role", role}, {"content", content_val}});
  }

  // ---- Build payload -------------------------------------------------------
  nlohmann::json payload;
  payload["model"]    = req.model;
  payload["messages"] = std::move(messages);
  payload["stream"]   = false;

  nlohmann::json options;
  if (req.max_tokens > 0)   options["num_predict"] = req.max_tokens;
  if (req.temperature >= 0) options["temperature"] = req.temperature;
  if (!options.empty())     payload["options"]     = std::move(options);

  // Expose tools if present (Ollama supports tool calls from v0.3+).
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
  http_req.url     = endpoint_;
  http_req.method  = "POST";
  http_req.body    = payload.dump();
  http_req.headers = {{"Content-Type", "application/json"}};
  // Ollama can be slow on a Pi; give it generous time.
  http_req.timeout_seconds = 600;

  auto resp_or = HttpDo(http_req);
  if (!resp_or.ok()) return resp_or.status();
  if (resp_or->status_code != 200) {
    return absl::InternalError(absl::StrCat(
        "Ollama API HTTP ", resp_or->status_code, ": ", resp_or->body));
  }

  // ---- Parse response ------------------------------------------------------
  return ParseOllamaChatResponse(resp_or->body);
}

}  // namespace ohc::agent
