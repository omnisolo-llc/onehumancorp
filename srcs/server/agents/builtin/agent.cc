#include "srcs/server/agents/builtin/agent.h"

#include <utility>

#include "absl/log/log.h"
#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "absl/types/span.h"

namespace ohc::agent {

namespace {

void IgnoreEvent(const AgentEvent& /*unused*/) {}

}  // namespace

// ---------------------------------------------------------------------------
// Constructor
// ---------------------------------------------------------------------------

Agent::Agent(std::unique_ptr<LLMClient> client, AgentConfig config,
             std::vector<Tool> tools)
    : llm_client_(std::move(client)), config_(std::move(config)) {
  tool_definitions_.reserve(tools.size());
  tool_executors_.reserve(tools.size());
  for (auto& tool : tools) {
    tool_definitions_.push_back(ToolDefinition{
        .name        = tool.name,
        .description = tool.description,
        .parameters  = std::move(tool.parameters),
    });
    tool_executors_.emplace(tool.name, std::move(tool.execute));
  }
}

// ---------------------------------------------------------------------------
// Run  (ReAct loop)
// ---------------------------------------------------------------------------

absl::StatusOr<std::vector<Message>> Agent::Run(
    absl::Span<const Message> initial_messages) {
  return Run(initial_messages, IgnoreEvent);
}

absl::StatusOr<std::vector<Message>> Agent::Run(
    absl::Span<const Message> initial_messages, EventCallback on_event) {
  std::vector<Message> messages(initial_messages.begin(),
                                initial_messages.end());

  on_event(AgentEvent{
      .type = AgentEvent::Type::kRunStarted,
      .message_count = static_cast<int32_t>(messages.size()),
  });

  for (int32_t iter = 0; iter < config_.max_iterations; ++iter) {
    TrimContext(&messages);

    on_event(AgentEvent{
        .type = AgentEvent::Type::kIterationStarted,
        .iteration = iter + 1,
        .message_count = static_cast<int32_t>(messages.size()),
    });

    ChatRequest req;
    req.model            = config_.model;
    req.system           = config_.system;
    req.messages         = messages;
    req.tool_definitions = absl::MakeSpan(tool_definitions_);
    req.max_tokens       = config_.max_tokens;
    req.temperature      = config_.temperature;

    auto resp_or = llm_client_->Chat(req);
    if (!resp_or.ok()) {
      on_event(AgentEvent{.type  = AgentEvent::Type::kTaskError,
                          .error = std::string(resp_or.status().message())});
      return resp_or.status();
    }

    // Move the assistant message into history.
    messages.push_back(std::move(resp_or->message));
    const Message& assistant_msg = messages.back();

    if (!assistant_msg.content.empty()) {
      on_event(AgentEvent{.type    = AgentEvent::Type::kTextChunk,
                          .content = assistant_msg.content});
    }

    if (assistant_msg.tool_calls.empty()) {
      // No tool calls → the agent is done.
      on_event(AgentEvent{.type    = AgentEvent::Type::kTaskComplete,
                          .content = assistant_msg.content});
      return messages;
    }

    // ----- Execute tool calls -----
    Message tool_result_msg;
    tool_result_msg.role = Role::kTool;

    for (const auto& tc : assistant_msg.tool_calls) {
      auto result = ExecuteToolCall(tc);

      ToolResult tr;
      tr.tool_call_id = tc.id;
      if (result.ok()) {
        tr.content = std::move(*result);
      } else {
        tr.error = std::string(result.status().message());
        LOG(WARNING) << "Tool '" << tc.name << "' error: " << tr.error;
      }

      on_event(AgentEvent{
          .type           = AgentEvent::Type::kToolCall,
          .tool_name      = tc.name,
          .tool_args_json = tc.arguments.dump(),
          .tool_result    = tr.content.empty() ? tr.error : tr.content,
      });

      tool_result_msg.tool_results.push_back(std::move(tr));
    }

    messages.push_back(std::move(tool_result_msg));
  }

  const auto err = absl::ResourceExhaustedError(
      absl::StrCat("Agent exceeded max_iterations=", config_.max_iterations));
  on_event(AgentEvent{.type  = AgentEvent::Type::kTaskError,
                      .error = std::string(err.message())});
  return err;
}

// ---------------------------------------------------------------------------
// TrimContext
// ---------------------------------------------------------------------------

void Agent::TrimContext(std::vector<Message>* messages) const {
  // Strategy: keep the first user message (the original task) and remove the
  // oldest *tool-result* messages first when the history is too long.
  // This preserves the task context while freeing memory from stale data.
  while (static_cast<int32_t>(messages->size()) >
             config_.max_context_messages &&
         messages->size() > 1) {
    // Erase the second element (oldest non-task message).
    messages->erase(messages->begin() + 1);
  }
}

// ---------------------------------------------------------------------------
// ExecuteToolCall
// ---------------------------------------------------------------------------

absl::StatusOr<std::string> Agent::ExecuteToolCall(const ToolCall& tc) {
  const auto it = tool_executors_.find(tc.name);
  if (it == tool_executors_.end()) {
    return absl::NotFoundError(absl::StrCat("Unknown tool: ", tc.name));
  }
  return it->second(tc.arguments);
}

}  // namespace ohc::agent
