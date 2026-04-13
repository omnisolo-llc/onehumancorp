#include "srcs/server/agents/builtin/tools/tool.h"

#include <array>
#include <cstdio>
#include <filesystem>
#include <string>

#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

Tool MakeGrepTool() {
  return Tool{
      .name        = "Grep",
      .description =
          "Search for a pattern in files under the specified directory "
          "(uses the system grep with -rn flags).",
      .parameters  = nlohmann::json::parse(R"({
        "type": "object",
        "properties": {
          "pattern": {
            "type": "string",
            "description": "The regex pattern to search for."
          },
          "directory": {
            "type": "string",
            "description": "The directory to search in."
          }
        },
        "required": ["pattern", "directory"]
      })"),
      .execute = [](const nlohmann::json& args) -> absl::StatusOr<std::string> {
        const auto pat_it = args.find("pattern");
        const auto dir_it = args.find("directory");
        if (pat_it == args.end() || !pat_it->is_string()) {
          return absl::InvalidArgumentError("Grep: missing 'pattern'");
        }
        if (dir_it == args.end() || !dir_it->is_string()) {
          return absl::InvalidArgumentError("Grep: missing 'directory'");
        }

        const std::string pattern   = pat_it->get<std::string>();
        const std::string directory = dir_it->get<std::string>();

        if (!std::filesystem::is_directory(directory)) {
          return absl::NotFoundError(
              absl::StrCat("Grep: directory not found: ", directory));
        }

        // Shell-quote arguments to prevent injection.
        // Using single quotes; internal single quotes are escaped.
        auto shell_quote = [](const std::string& s) {
          std::string out = "'";
          for (char c : s) {
            if (c == '\'') out += "'\\''";
            else           out += c;
          }
          return out + "'";
        };

        const std::string cmd = absl::StrCat(
            "grep -rn ", shell_quote(pattern), " ", shell_quote(directory));

        FILE* fp = ::popen(cmd.c_str(), "r");
        if (!fp) {
          return absl::InternalError("Grep: popen failed");
        }
        std::string output;
        std::array<char, 4096> buf{};
        while (std::fgets(buf.data(), buf.size(), fp) != nullptr) {
          output.append(buf.data());
        }
        ::pclose(fp);
        return output.empty() ? "No matches found." : output;
      },
  };
}

}  // namespace ohc::agent
