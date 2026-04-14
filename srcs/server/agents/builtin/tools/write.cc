#include "srcs/server/agents/builtin/tools/tool.h"

#include <cerrno>
#include <system_error>
#include <filesystem>
#include <fstream>
#include <string>

#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

Tool MakeWriteTool() {
  return Tool{
      .name        = "Write",
      .description = "Write content to a file on the local filesystem. "
                     "Creates parent directories as needed.",
      .parameters  = nlohmann::json::parse(R"({
        "type": "object",
        "properties": {
          "file_path": {
            "type": "string",
            "description": "The absolute path to the file to write."
          },
          "content": {
            "type": "string",
            "description": "The content to write to the file."
          }
        },
        "required": ["file_path", "content"]
      })"),
      .execute = [](const nlohmann::json& args) -> absl::StatusOr<std::string> {
        const auto path_it    = args.find("file_path");
        const auto content_it = args.find("content");
        if (path_it == args.end() || !path_it->is_string()) {
          return absl::InvalidArgumentError("Write: missing 'file_path'");
        }
        if (content_it == args.end() || !content_it->is_string()) {
          return absl::InvalidArgumentError("Write: missing 'content'");
        }

        const std::filesystem::path p = path_it->get<std::string>();
        std::error_code ec;
        std::filesystem::create_directories(p.parent_path(), ec);
        if (ec) {
          return absl::InternalError(absl::StrCat(
              "Write: failed to create directories for '",
              p.string(), "': ", ec.message()));
        }

        std::ofstream file(p, std::ios::binary | std::ios::trunc);
        if (!file.is_open()) {
          // Use std::error_code for thread-safe error description.
          const std::error_code ec(errno, std::generic_category());
          return absl::InternalError(absl::StrCat(
              "Write: cannot open '", p.string(), "': ", ec.message()));
        }
        const std::string content = content_it->get<std::string>();
        file.write(content.data(), static_cast<std::streamsize>(content.size()));
        if (!file) {
          return absl::InternalError(
            absl::StrCat("Write: error writing to '", p.string(), "'"));
        }
        return "File written successfully.";
      },
  };
}

}  // namespace ohc::agent
