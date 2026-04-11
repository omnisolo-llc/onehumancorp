#include "srcs/server/agents/builtin/tools/all_tools.h"

#include <vector>

namespace ohc::agent {

std::vector<Tool> MakeDefaultTools() {
  std::vector<Tool> tools;
  tools.reserve(10);
  tools.push_back(MakeBashTool());
  tools.push_back(MakeReadTool());
  tools.push_back(MakeWriteTool());
  tools.push_back(MakeGlobTool());
  tools.push_back(MakeGrepTool());
  tools.push_back(MakeWebFetchTool());
  tools.push_back(MakeWebSearchTool());
  tools.push_back(MakeSendMessageTool());
  tools.push_back(MakeTodoWriteTool());
  tools.push_back(MakeToolSearchTool());
  return tools;
}

}  // namespace ohc::agent
