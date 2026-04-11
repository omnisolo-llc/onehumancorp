#include "srcs/server/agents/builtin/tools/tool.h"

#include <string>

#include "srcs/server/agents/builtin/http_client.h"
#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

Tool MakeWebFetchTool() {
  return Tool{
      .name        = "WebFetch",
      .description = "Fetch the raw content of a URL via HTTP GET.",
      .parameters  = nlohmann::json::parse(R"({
        "type": "object",
        "properties": {
          "url": {
            "type": "string",
            "description": "The URL to fetch."
          }
        },
        "required": ["url"]
      })"),
      .execute = [](const nlohmann::json& args) -> absl::StatusOr<std::string> {
        const auto it = args.find("url");
        if (it == args.end() || !it->is_string()) {
          return absl::InvalidArgumentError("WebFetch: missing 'url'");
        }

        HttpRequest req;
        req.url    = it->get<std::string>();
        req.method = "GET";

        auto resp_or = HttpDo(req);
        if (!resp_or.ok()) return resp_or.status();
        if (resp_or->status_code != 200) {
          return absl::InternalError(absl::StrCat(
              "WebFetch: HTTP ", resp_or->status_code));
        }
        return std::move(resp_or->body);
      },
  };
}

}  // namespace ohc::agent
