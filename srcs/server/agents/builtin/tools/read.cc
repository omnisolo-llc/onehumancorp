#include "srcs/server/agents/builtin/tools/tool.h"

#include <cerrno>
#include <cstring>
#include <fstream>
#include <sstream>
#include <string>

#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

Tool MakeReadTool() {
  return Tool{
      .name        = "Read",
      .description = "Read a file from the local filesystem.",
      .parameters  = nlohmann::json::parse(R"({
        "type": "object",
        "properties": {
          "file_path": {
            "type": "string",
            "description": "The absolute path to the file to read."
          }
        },
        "required": ["file_path"]
      })"),
      .execute = [](const nlohmann::json& args) -> absl::StatusOr<std::string> {
        const auto it = args.find("file_path");
        if (it == args.end() || !it->is_string()) {
          return absl::InvalidArgumentError("Read: missing 'file_path'");
        }
        const std::string path = it->get<std::string>();

        std::ifstream file(path, std::ios::binary);
        if (!file.is_open()) {
          return absl::NotFoundError(
              absl::StrCat("Read: cannot open '", path, "': ",
                           std::strerror(errno)));
        }
        std::ostringstream ss;
        ss << file.rdbuf();
        return ss.str();
      },
  };
}

}  // namespace ohc::agent
