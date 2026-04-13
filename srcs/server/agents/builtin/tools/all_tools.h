#pragma once

// Convenience header that exposes factory functions for every built-in tool.
// Each function returns a fully initialised Tool object.

#include <vector>

#include "srcs/server/agents/builtin/tools/tool.h"

namespace ohc::agent {

// Individual tool factories (defined in their respective .cc files).
Tool MakeBashTool();
Tool MakeReadTool();
Tool MakeWriteTool();
Tool MakeGlobTool();
Tool MakeGrepTool();
Tool MakeWebFetchTool();
Tool MakeWebSearchTool();
Tool MakeSendMessageTool();
Tool MakeTodoWriteTool();
Tool MakeToolSearchTool();

// Returns the complete set of default tools.
// The result is move-constructed into the Agent, so no extra copies occur.
std::vector<Tool> MakeDefaultTools();

}  // namespace ohc::agent
