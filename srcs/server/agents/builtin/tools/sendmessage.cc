#include "srcs/server/agents/builtin/tools/tool.h"

#include <cstdio>
#include <string>

#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

Tool MakeSendMessageTool() {
  return Tool{
      .name        = "SendMessage",
      .description = "Send a message to the user (printed to stdout).",
      .parameters  = nlohmann::json::parse(R"({
        "type": "object",
        "properties": {
          "message": {
            "type": "string",
            "description": "The message to send."
          }
        },
        "required": ["message"]
      })"),
      .execute = [](const nlohmann::json& args) -> absl::StatusOr<std::string> {
        const auto it = args.find("message");
        if (it == args.end() || !it->is_string()) {
          return absl::InvalidArgumentError("SendMessage: missing 'message'");
        }
        const std::string msg = it->get<std::string>();
        std::printf("\n=== MESSAGE TO USER ===\n%s\n=======================\n",
                    msg.c_str());
        std::fflush(stdout);
        return "Message sent.";
      },
  };
}

}  // namespace ohc::agent
