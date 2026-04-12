#include "srcs/server/agents/builtin/tools/tool.h"

#include <filesystem>
#include <fstream>
#include <string>

#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {

// Default TODO file path relative to the working directory.
static constexpr absl::string_view kTodoPath = ".agent-task/todo.txt";

Tool MakeTodoWriteTool() {
  return Tool{
      .name        = "TodoWrite",
      .description = "Append an item to the active TODO list at "
                     ".agent-task/todo.txt.",
      .parameters  = nlohmann::json::parse(R"({
        "type": "object",
        "properties": {
          "todo": {
            "type": "string",
            "description": "The item to add to the TODO list."
          }
        },
        "required": ["todo"]
      })"),
      .execute = [](const nlohmann::json& args) -> absl::StatusOr<std::string> {
        const auto it = args.find("todo");
        if (it == args.end() || !it->is_string()) {
          return absl::InvalidArgumentError("TodoWrite: missing 'todo'");
        }
        const std::string todo = it->get<std::string>();
        const std::filesystem::path p(kTodoPath);
        std::error_code ec;
        std::filesystem::create_directories(p.parent_path(), ec);
        if (ec) {
          return absl::InternalError(absl::StrCat(
              "TodoWrite: could not create directory: ", ec.message()));
        }

        std::ofstream file(p, std::ios::app);
        if (!file.is_open()) {
          return absl::InternalError(absl::StrCat(
              "TodoWrite: cannot open '", std::string(kTodoPath), "'"));
        }
        file << "- " << todo << "\n";
        file.close();
        if (!file) {
          return absl::InternalError("TodoWrite: write failed");
        }
        return "Todo added successfully.";
      },
  };
}

}  // namespace ohc::agent
