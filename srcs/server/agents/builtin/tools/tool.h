#pragma once

// Tool definition for the C++ builtin agent.
// A Tool pairs the LLM-visible ToolDefinition (name, description, JSON Schema)
// with a native C++ executor function.

#include <string>

#include "srcs/server/agents/builtin/types.h"
#include "absl/functional/any_invocable.h"
#include "absl/status/statusor.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

// Execute function type: receives parsed JSON arguments; returns the tool
// output string or an error status.
using ToolExecuteFn =
  absl::AnyInvocable<absl::StatusOr<std::string>(
    const nlohmann::json& args) const>;

// Tool bundles the LLM-visible definition with the C++ execution logic.
struct Tool {
  std::string    name;
  std::string    description;
  nlohmann::json parameters;  // JSON Schema describing accepted arguments.
  ToolExecuteFn  execute;
};

}  // namespace ohc::agent
