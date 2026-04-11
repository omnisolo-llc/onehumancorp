#include "srcs/server/agents/builtin/llm_ollama.h"

#include <utility>

#include "srcs/server/agents/builtin/http_client.h"
#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

OllamaClient::OllamaClient(std::string endpoint)
    : endpoint_(std::move(endpoint)) {}

absl::StatusOr<ChatResponse> OllamaClient::Chat(const ChatRequest& req) {
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
  const auto result =
      nlohmann::json::parse(resp_or->body, nullptr, false);
  if (result.is_discarded()) {
    return absl::InternalError("Failed to parse Ollama JSON response");
  }

  ChatResponse chat_resp;
  chat_resp.message.role = Role::kAssistant;

  if (result.contains("message")) {
    const auto& msg = result["message"];
    chat_resp.message.content = msg.value("content", std::string{});

    // Ollama tool-call format mirrors OpenAI.
    if (msg.contains("tool_calls")) {
      for (const auto& tc : msg["tool_calls"]) {
        ToolCall call;
        if (tc.contains("function")) {
          call.name      = tc["function"].value("name",      std::string{});
          call.arguments = tc["function"].value("arguments", nlohmann::json{});
        }
        // Ollama does not always supply an id; synthesise one.
        call.id = call.name + "_" + std::to_string(chat_resp.message.tool_calls.size());
        chat_resp.message.tool_calls.push_back(std::move(call));
      }
    }
  }

  return chat_resp;
}

}  // namespace ohc::agent
