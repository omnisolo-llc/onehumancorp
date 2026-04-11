#include "srcs/server/agents/builtin/tools/tool.h"

#include <array>
#include <cstdio>
#include <string>

#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

// Execute a bash command and capture its combined stdout+stderr output.
// The command is run via /bin/sh -c so pipelines and redirections work.
// Exit status != 0 is returned as content (not an error) so the LLM can
// observe the failure output and decide the next step.
Tool MakeBashTool() {
  return Tool{
      .name        = "Bash",
      .description = "Execute a bash script and return its combined output.",
      .parameters  = nlohmann::json::parse(R"({
        "type": "object",
        "properties": {
          "command": {
            "type": "string",
            "description": "The bash command or script to execute."
          }
        },
        "required": ["command"]
      })"),
      .execute = [](const nlohmann::json& args) -> absl::StatusOr<std::string> {
        const auto it = args.find("command");
        if (it == args.end() || !it->is_string()) {
          return absl::InvalidArgumentError("Bash: missing 'command'");
        }
        const std::string cmd = it->get<std::string>();

        // popen runs the command in a shell; we read until EOF.
        FILE* fp = ::popen(cmd.c_str(), "r");
        if (!fp) {
          return absl::InternalError(
              absl::StrCat("popen failed for command: ", cmd));
        }
        std::string output;
        std::array<char, 4096> buf{};
        while (std::fgets(buf.data(), buf.size(), fp) != nullptr) {
          output.append(buf.data());
        }
        ::pclose(fp);
        return output;
      },
  };
}

}  // namespace ohc::agent
