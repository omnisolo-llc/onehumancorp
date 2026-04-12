#pragma once

// C++ implementation of the BuiltinAgent ReAct loop.
// Mirrors the logic in loop.go, adding:
//   • Streaming events via EventCallback.
//   • Context trimming to bound memory usage.
//   • absl::flat_hash_map for O(1) tool dispatch.

#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "srcs/server/agents/builtin/llm_client.h"
#include "srcs/server/agents/builtin/tools/tool.h"
#include "srcs/server/agents/builtin/types.h"
#include "absl/functional/function_ref.h"
#include "absl/container/flat_hash_map.h"
#include "absl/container/inlined_vector.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"

namespace ohc::agent {

// ---------------------------------------------------------------------------
// AgentEvent  (emitted during execution for streaming consumers)
// ---------------------------------------------------------------------------

struct AgentEvent {
  enum class Type : uint8_t {
    kRunStarted       = 0,  // A task was accepted and runtime initialized.
    kIterationStarted = 1,  // A new ReAct iteration has started.
    kTextChunk        = 2,  // Partial assistant text.
    kToolCall         = 3,  // A tool was invoked.
    kTaskComplete     = 4,  // Final response ready.
    kTaskError        = 5,  // Unrecoverable error.
  };

  Type        type;
  int32_t     iteration = 0;     // RUN_STARTED / ITERATION_STARTED
  int32_t     message_count = 0; // RUN_STARTED / ITERATION_STARTED
  std::string content;        // TEXT_CHUNK / TASK_COMPLETE
  std::string tool_name;      // TOOL_CALL
  std::string tool_args_json; // TOOL_CALL
  std::string tool_result;    // TOOL_CALL
  std::string error;          // TASK_ERROR
};

using EventCallback = absl::FunctionRef<void(const AgentEvent&)>;

// ---------------------------------------------------------------------------
// AgentConfig
// ---------------------------------------------------------------------------

struct AgentConfig {
  std::string model;
  std::string system;
  int32_t     max_tokens           = 2048;
  float       temperature          = 0.7f;
  // Maximum ReAct iterations before giving up.
  int32_t     max_iterations       = 50;
  // Trim oldest messages when history exceeds this size.
  int32_t     max_context_messages = 100;
};

// ---------------------------------------------------------------------------
// Agent
// ---------------------------------------------------------------------------

class Agent {
 public:
  // Takes ownership of the LLM client.  Tools are moved in.
  Agent(std::unique_ptr<LLMClient> client, AgentConfig config,
        std::vector<Tool> tools);

  // Not copyable; move-constructible.
  Agent(const Agent&)            = delete;
  Agent& operator=(const Agent&) = delete;
  Agent(Agent&&)                 = default;
  Agent& operator=(Agent&&)      = default;

  // Runs the ReAct loop, starting with initial_messages.
  // Returns the full conversation history on success.
  // on_event is called synchronously from the loop thread.
  absl::StatusOr<std::vector<Message>> Run(
      absl::Span<const Message> initial_messages);

  absl::StatusOr<std::vector<Message>> Run(
      absl::Span<const Message> initial_messages,
      EventCallback on_event);

 private:
  // Removes old tool-result messages when the history is too long.
  void TrimContext(std::vector<Message>* messages) const;

  // Dispatches a single ToolCall to the registered executor.
  absl::StatusOr<std::string> ExecuteToolCall(const ToolCall& tc);

  std::unique_ptr<LLMClient>                       llm_client_;
  AgentConfig                                      config_;
  // Parallel arrays: definitions (sent to LLM) and executors (called locally).
  std::vector<ToolDefinition>                      tool_definitions_;
  absl::flat_hash_map<std::string, ToolExecuteFn>  tool_executors_;
};

}  // namespace ohc::agent
