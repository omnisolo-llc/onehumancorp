// C++ unit tests for the BuiltinAgent, mirroring agent_test.go.
// Uses GoogleTest for assertions and GoogleMock for fine-grained mocking of
// the LLMClient interface.

#include "srcs/server/agents/builtin/agent.h"

#include <filesystem>
#include <fstream>
#include <memory>
#include <string>
#include <vector>

#include "srcs/server/agents/builtin/tools/all_tools.h"
#include "srcs/server/agents/builtin/tools/tool.h"
#include "srcs/server/agents/builtin/types.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/types/span.h"
#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace ohc::agent {
namespace {

using ::testing::_;
using ::testing::Return;
using ::testing::StrictMock;

// ---------------------------------------------------------------------------
// MockLLMClient
// ---------------------------------------------------------------------------

class MockLLMClient : public LLMClient {
 public:
  MOCK_METHOD(absl::StatusOr<ChatResponse>, Chat, (const ChatRequest& req),
              (override));
};

// ---------------------------------------------------------------------------
// Helper: build a simple assistant ChatResponse.
// ---------------------------------------------------------------------------

ChatResponse MakeTextResponse(absl::string_view content) {
  ChatResponse resp;
  resp.message.role    = Role::kAssistant;
  resp.message.content = std::string(content);
  return resp;
}

ChatResponse MakeToolCallResponse(absl::string_view tool_name,
                                  const nlohmann::json& args) {
  ChatResponse resp;
  resp.message.role = Role::kAssistant;
  ToolCall tc;
  tc.id        = "tc_001";
  tc.name      = std::string(tool_name);
  tc.arguments = args;
  resp.message.tool_calls.push_back(std::move(tc));
  return resp;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

TEST(BuiltinAgentTest, TerminatesOnNoToolCalls) {
  auto mock = std::make_unique<StrictMock<MockLLMClient>>();
  EXPECT_CALL(*mock, Chat(_))
      .WillOnce(Return(MakeTextResponse("Hello, world!")));

  AgentConfig cfg;
  cfg.model  = "mock-model";
  cfg.system = "You are a helpful assistant.";

  Agent agent(std::move(mock), cfg, {});

  const Message user_msg{.role = Role::kUser, .content = "Say hi"};
  const absl::Span<const Message> initial({user_msg});

  auto result = agent.Run(initial);
  ASSERT_TRUE(result.ok()) << result.status();

  ASSERT_GE(result->size(), 2u);
  EXPECT_EQ((*result)[1].content, "Hello, world!");
}

TEST(BuiltinAgentTest, ExecutesToolCallThenTerminates) {
  auto mock = std::make_unique<StrictMock<MockLLMClient>>();

  // First call: request a WebSearch tool.
  EXPECT_CALL(*mock, Chat(_))
      .WillOnce(Return(
          MakeToolCallResponse("WebSearch", {{"query", "Bazel C++ tutorial"}})))
      .WillOnce(Return(MakeTextResponse("I found some results.")));

  AgentConfig cfg;
  cfg.model = "mock-model";

  Agent agent(std::move(mock), cfg, {MakeWebSearchTool()});

  const Message user_msg{.role = Role::kUser, .content = "Search for Bazel"};
  const absl::Span<const Message> initial({user_msg});

  auto result = agent.Run(initial);
  ASSERT_TRUE(result.ok()) << result.status();

  // Messages: user → assistant (tool call) → tool result → assistant (final).
  ASSERT_GE(result->size(), 4u);
  // The last assistant message should contain the final response.
  EXPECT_EQ(result->back().content, "I found some results.");
}

TEST(BuiltinAgentTest, ToolErrorSurfacedToLLM) {
  auto mock = std::make_unique<StrictMock<MockLLMClient>>();

  // LLM requests an unknown tool.
  EXPECT_CALL(*mock, Chat(_))
      .WillOnce(Return(
          MakeToolCallResponse("NonExistentTool", {{"x", 1}})))
      .WillOnce(Return(MakeTextResponse("I see the tool failed.")));

  AgentConfig cfg;
  cfg.model = "mock-model";

  Agent agent(std::move(mock), cfg, {});  // No tools registered.

  const Message user_msg{.role = Role::kUser, .content = "Do something"};
  const absl::Span<const Message> initial({user_msg});

  auto result = agent.Run(initial);
  ASSERT_TRUE(result.ok()) << result.status();

  // Find the tool-result message and verify it contains an error.
  bool found_error = false;
  for (const auto& msg : *result) {
    if (msg.role == Role::kTool) {
      for (const auto& tr : msg.tool_results) {
        if (!tr.error.empty()) {
          found_error = true;
        }
      }
    }
  }
  EXPECT_TRUE(found_error) << "Expected a tool error message in history";
}

TEST(BuiltinAgentTest, LLMErrorPropagated) {
  auto mock = std::make_unique<StrictMock<MockLLMClient>>();
  EXPECT_CALL(*mock, Chat(_))
      .WillOnce(Return(absl::InternalError("LLM unavailable")));

  AgentConfig cfg;
  cfg.model = "mock-model";

  Agent agent(std::move(mock), cfg, {});

  const Message user_msg{.role = Role::kUser, .content = "ping"};
  const absl::Span<const Message> initial({user_msg});

  auto result = agent.Run(initial);
  EXPECT_FALSE(result.ok());
  EXPECT_EQ(result.status().code(), absl::StatusCode::kInternal);
}

TEST(BuiltinAgentTest, EventCallbackFiredForTextAndToolCalls) {
  auto mock = std::make_unique<StrictMock<MockLLMClient>>();
  EXPECT_CALL(*mock, Chat(_))
      .WillOnce(Return(
          MakeToolCallResponse("WebSearch", {{"query", "test"}})))
      .WillOnce(Return(MakeTextResponse("Done.")));

  AgentConfig cfg;
  cfg.model = "mock-model";

  Agent agent(std::move(mock), cfg, {MakeWebSearchTool()});

  std::vector<AgentEvent::Type> event_types;
  auto on_event = [&](const AgentEvent& ev) {
    event_types.push_back(ev.type);
  };

  const Message user_msg{.role = Role::kUser, .content = "search"};
  const absl::Span<const Message> initial({user_msg});

  auto result = agent.Run(initial, std::move(on_event));
  ASSERT_TRUE(result.ok()) << result.status();

  EXPECT_THAT(event_types, ::testing::Contains(AgentEvent::Type::kToolCall));
  EXPECT_THAT(event_types,
              ::testing::Contains(AgentEvent::Type::kTaskComplete));
}

// ---------------------------------------------------------------------------
// Tool-level tests
// ---------------------------------------------------------------------------

TEST(WebSearchToolTest, ReturnsNonEmptyResult) {
  const auto tool = MakeWebSearchTool();
  auto result     = tool.execute({{"query", "bazel"}});
  ASSERT_TRUE(result.ok()) << result.status();
  EXPECT_FALSE(result->empty());
}

TEST(WebSearchToolTest, MissingQueryReturnsError) {
  const auto tool = MakeWebSearchTool();
  auto result     = tool.execute({});
  EXPECT_FALSE(result.ok());
  EXPECT_EQ(result.status().code(), absl::StatusCode::kInvalidArgument);
}

TEST(TodoWriteToolTest, WritesAndAppendsItems) {
  const auto tool = MakeTodoWriteTool();

  // Use a temp directory that does not affect the real workspace.
  const std::filesystem::path tmp =
      std::filesystem::temp_directory_path() / "ohc_agent_test_todo";
  std::error_code ec;
  std::filesystem::remove_all(tmp, ec);
  std::filesystem::create_directories(tmp, ec);
  ASSERT_FALSE(ec) << "Failed to create tmp dir: " << ec.message();

  // chdir into the temp directory so the tool writes .agent-task/todo.txt
  // there instead of the workspace root.
  const std::filesystem::path original = std::filesystem::current_path();
  std::filesystem::current_path(tmp, ec);
  ASSERT_FALSE(ec) << "chdir failed: " << ec.message();

  // Restore working directory on scope exit, even if a ASSERT fires.
  struct Restorer {
    std::filesystem::path p;
    ~Restorer() {
      std::error_code e;
      std::filesystem::current_path(p, e);
    }
  } restorer{original};

  auto r1 = tool.execute({{"todo", "first item"}});
  ASSERT_TRUE(r1.ok()) << r1.status();

  auto r2 = tool.execute({{"todo", "second item"}});
  ASSERT_TRUE(r2.ok()) << r2.status();

  // Verify both items are present.
  const std::filesystem::path todo_file = tmp / ".agent-task" / "todo.txt";
  std::ifstream f(todo_file);
  ASSERT_TRUE(f.is_open()) << "Cannot open " << todo_file;
  const std::string content(std::istreambuf_iterator<char>(f), {});

  EXPECT_THAT(content, ::testing::HasSubstr("first item"));
  EXPECT_THAT(content, ::testing::HasSubstr("second item"));

  std::filesystem::remove_all(tmp, ec);
}

TEST(BashToolTest, RunsSimpleCommand) {
  const auto tool = MakeBashTool();
  auto result     = tool.execute({{"command", "echo hello_world"}});
  ASSERT_TRUE(result.ok()) << result.status();
  EXPECT_THAT(*result, ::testing::HasSubstr("hello_world"));
}

TEST(ReadToolTest, MissingFileReturnsNotFound) {
  const auto tool = MakeReadTool();
  auto result     = tool.execute({{"file_path", "/nonexistent/path/file.txt"}});
  EXPECT_FALSE(result.ok());
  EXPECT_EQ(result.status().code(), absl::StatusCode::kNotFound);
}

}  // namespace
}  // namespace ohc::agent
