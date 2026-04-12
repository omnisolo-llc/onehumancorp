#include "srcs/server/agents/builtin/tools/tool.h"
#include "srcs/server/agents/builtin/tools/test_hooks.h"

#include <glob.h>

#include <string>
#include <vector>

#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_join.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

namespace {

GlobFnForTesting g_glob_fn = &::glob;

}  // namespace

Tool MakeGlobTool() {
  return Tool{
      .name        = "Glob",
      .description = "List files matching a POSIX glob pattern.",
      .parameters  = nlohmann::json::parse(R"({
        "type": "object",
        "properties": {
          "pattern": {
            "type": "string",
            "description": "The glob pattern (e.g. '/src/**/*.cc')."
          }
        },
        "required": ["pattern"]
      })"),
      .execute = [](const nlohmann::json& args) -> absl::StatusOr<std::string> {
        const auto it = args.find("pattern");
        if (it == args.end() || !it->is_string()) {
          return absl::InvalidArgumentError("Glob: missing 'pattern'");
        }
        const std::string pattern = it->get<std::string>();

        glob_t result{};
        // GLOB_TILDE expands ~ and GLOB_BRACE enables {a,b} patterns.
        const int flags = GLOB_TILDE | GLOB_BRACE;
        const int rc    = g_glob_fn(pattern.c_str(), flags, nullptr, &result);

        struct GlobGuard {
          glob_t* g;
          ~GlobGuard() { globfree(g); }
        } guard{&result};

        if (rc == GLOB_NOMATCH) {
          return "No files found matching pattern.";
        }
        if (rc != 0) {
          return absl::InternalError(
              absl::StrCat("Glob: error expanding pattern '", pattern, "'"));
        }

        std::vector<absl::string_view> matches;
        matches.reserve(result.gl_pathc);
        for (size_t i = 0; i < result.gl_pathc; ++i) {
          matches.emplace_back(result.gl_pathv[i]);
        }
        return absl::StrJoin(matches, "\n");
      },
  };
}

void SetGlobFnForTesting(GlobFnForTesting glob_fn) {
  g_glob_fn = glob_fn;
}

void ResetGlobFnForTesting() {
  g_glob_fn = &::glob;
}

}  // namespace ohc::agent
