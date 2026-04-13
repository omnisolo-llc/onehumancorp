#include "srcs/server/agents/builtin/tools/tool.h"

#include <array>
#include <string>
#include <vector>

#include "absl/status/statusor.h"
#include "absl/strings/ascii.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_join.h"
#include "absl/strings/string_view.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

// Static list kept in sync with all_tools.cc.  Using a plain array avoids
// any heap allocation for the metadata itself.
struct ToolMeta { absl::string_view name; absl::string_view desc; };

static constexpr std::array<ToolMeta, 10> kToolMeta{{
    {"Bash",        "Execute a bash script."},
    {"Read",        "Read a file from the local filesystem."},
    {"Write",       "Write content to a file on the local filesystem."},
    {"Glob",        "List files matching a POSIX glob pattern."},
    {"Grep",        "Search for a pattern in files in the specified directory."},
    {"WebFetch",    "Fetch the raw content of a URL via HTTP GET."},
    {"WebSearch",   "Search the web for a query."},
    {"SendMessage", "Send a message to the user."},
    {"TodoWrite",   "Append an item to the active TODO list."},
    {"ToolSearch",  "Search for available tools and their descriptions."},
}};

Tool MakeToolSearchTool() {
  return Tool{
      .name        = "ToolSearch",
      .description =
          "Search for available tools and their descriptions. "
          "Pass an empty query to list all tools.",
      .parameters  = nlohmann::json::parse(R"({
        "type": "object",
        "properties": {
          "query": {
            "type": "string",
            "description": "Optional keyword to filter tools."
          }
        }
      })"),
      .execute = [](const nlohmann::json& args) -> absl::StatusOr<std::string> {
        std::string query;
        const auto it = args.find("query");
        if (it != args.end() && it->is_string()) {
          query = absl::AsciiStrToLower(it->get<std::string>());
        }

        std::vector<std::string> matches;
        for (const auto& meta : kToolMeta) {
          if (query.empty() ||
              absl::AsciiStrToLower(meta.name).find(query) !=
                  std::string::npos ||
              absl::AsciiStrToLower(meta.desc).find(query) !=
                  std::string::npos) {
            matches.emplace_back(absl::StrCat(meta.name, ": ", meta.desc));
          }
        }

        if (matches.empty()) return "No tools found matching query.";
        return absl::StrJoin(matches, "\n");
      },
  };
}

}  // namespace ohc::agent
