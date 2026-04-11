#pragma once

// Tool definition for the C++ builtin agent.
// A Tool pairs the LLM-visible ToolDefinition (name, description, JSON Schema)
// with a native C++ executor function.

#include <functional>
#include <string>

#include "srcs/server/agents/builtin/types.h"
#include "absl/status/statusor.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

// Execute function type: receives parsed JSON arguments; returns the tool
// output string or an error status.
// Using std::function (owning, heap-based) since tools are stored in a
// vector and need to outlive the call site.
using ToolExecuteFn =
    std::function<absl::StatusOr<std::string>(const nlohmann::json& args)>;

// Tool bundles the LLM-visible definition with the C++ execution logic.
struct Tool {
  std::string    name;
  std::string    description;
  nlohmann::json parameters;  // JSON Schema describing accepted arguments.
  ToolExecuteFn  execute;
};

}  // namespace ohc::agent
