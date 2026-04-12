#pragma once

// C++ equivalents of the types defined in types.go.
// Optimised for minimal memory footprint:
//   • absl::InlinedVector avoids heap allocations for small collections.
//   • absl::string_view is used wherever ownership is not required.
//   • Role is stored as a 1-byte enum.

#include <cstdint>
#include <string>
#include <vector>

#include "absl/container/inlined_vector.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

// ---------------------------------------------------------------------------
// Role
// ---------------------------------------------------------------------------

enum class Role : uint8_t {
  kUser      = 0,
  kAssistant = 1,
  kSystem    = 2,
  kTool      = 3,
};

// Returns a string_view for the role (no allocation).
constexpr absl::string_view RoleToStringView(Role role) noexcept {
  switch (role) {
    case Role::kUser:      return "user";
    case Role::kAssistant: return "assistant";
    case Role::kSystem:    return "system";
    case Role::kTool:      return "tool";
  }
  return "unknown";
}

// ---------------------------------------------------------------------------
// ToolCall / ToolResult
// ---------------------------------------------------------------------------

// A single tool-call request emitted by the LLM.
struct ToolCall {
  std::string    id;
  std::string    name;
  nlohmann::json arguments;  // Parsed JSON; nlohmann::json owns the storage.
};

// The result of executing one ToolCall.
struct ToolResult {
  std::string tool_call_id;
  std::string content;  // Populated on success.
  std::string error;    // Populated on failure.
};

// In practice a single LLM turn rarely issues more than 4 tool calls; inline
// up to 4 to avoid a heap allocation in the common case.
using ToolCallList   = absl::InlinedVector<ToolCall,   4>;
using ToolResultList = absl::InlinedVector<ToolResult, 4>;

// ---------------------------------------------------------------------------
// Message
// ---------------------------------------------------------------------------

struct Message {
  Role           role         = Role::kUser;
  std::string    content;
  ToolCallList   tool_calls;
  ToolResultList tool_results;
};

// ---------------------------------------------------------------------------
// ToolDefinition  (sent to the LLM so it knows which tools exist)
// ---------------------------------------------------------------------------

struct ToolDefinition {
  std::string    name;
  std::string    description;
  nlohmann::json parameters;  // JSON Schema object.
};

// ---------------------------------------------------------------------------
// ChatRequest / ChatResponse
// ---------------------------------------------------------------------------

struct ChatRequest {
  std::string                      model;
  std::string                      system;
  // Owned message history for this request.
  std::vector<Message>             messages;
  // Non-owning span into the agent's tool_definitions_ vector.
  // The span must not outlive the ChatRequest.
  absl::Span<const ToolDefinition> tool_definitions;
  int32_t                          max_tokens  = 2048;
  float                            temperature = 0.7f;
};

struct ChatResponse {
  Message message;
};

}  // namespace ohc::agent
