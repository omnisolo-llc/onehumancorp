#include "srcs/server/agents/builtin/tools/tool.h"

#include <string>

#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_replace.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

Tool MakeWebSearchTool() {
  return Tool{
      .name        = "WebSearch",
      .description =
          "Search the web for a query. Returns a DuckDuckGo search URL and "
          "a simulated result summary. Plug in a real search API key via the "
          "--search_api_key flag for production use.",
      .parameters  = nlohmann::json::parse(R"({
        "type": "object",
        "properties": {
          "query": {
            "type": "string",
            "description": "The search query."
          }
        },
        "required": ["query"]
      })"),
      .execute = [](const nlohmann::json& args) -> absl::StatusOr<std::string> {
        const auto it = args.find("query");
        if (it == args.end() || !it->is_string()) {
          return absl::InvalidArgumentError("WebSearch: missing 'query'");
        }
        const std::string query = it->get<std::string>();

        // Percent-encode the query (minimal: replace spaces with +).
        const std::string encoded =
            absl::StrReplaceAll(query, {{" ", "+"}});

        return absl::StrCat(
            "Simulated search results for: ", query, "\n",
            "https://duckduckgo.com/html/?q=", encoded);
      },
  };
}

}  // namespace ohc::agent
