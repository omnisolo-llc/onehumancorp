// C++ unit tests for the BuiltinAgent, mirroring agent_test.go.
// Uses GoogleTest for assertions and GoogleMock for fine-grained mocking of
// the LLMClient interface.

#include "srcs/server/agents/builtin/agent.h"

#include <array>
#include <filesystem>
#include <fstream>
#include <chrono>
#include <memory>
#include <string>
#include <sys/resource.h>
#include <vector>

#include "srcs/server/agents/builtin/tools/all_tools.h"
#include "srcs/server/agents/builtin/tools/test_hooks.h"
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
using ::testing::Invoke;
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

std::filesystem::path MakeTempDir(absl::string_view prefix) {
  const auto suffix = std::to_string(
      std::chrono::steady_clock::now().time_since_epoch().count());
  const auto path = std::filesystem::temp_directory_path() /
                    (std::string(prefix) + "_" + suffix);
  std::error_code ec;
  std::filesystem::create_directories(path, ec);
  EXPECT_FALSE(ec) << ec.message();
  return path;
}

class ScopedCurrentDirectory {
 public:
  explicit ScopedCurrentDirectory(const std::filesystem::path& path)
      : original_(std::filesystem::current_path()) {
    std::error_code ec;
    std::filesystem::current_path(path, ec);
    EXPECT_FALSE(ec) << ec.message();
  }

  ~ScopedCurrentDirectory() {
    std::error_code ec;
    std::filesystem::current_path(original_, ec);
  }

 private:
  std::filesystem::path original_;
};

class ScopedFileDescriptorLimit {
 public:
  explicit ScopedFileDescriptorLimit(rlim_t soft_limit) {
    EXPECT_EQ(::getrlimit(RLIMIT_NOFILE, &original_), 0);
    rlimit updated = original_;
    updated.rlim_cur = soft_limit;
    EXPECT_EQ(::setrlimit(RLIMIT_NOFILE, &updated), 0);
  }

  ~ScopedFileDescriptorLimit() {
    EXPECT_EQ(::setrlimit(RLIMIT_NOFILE, &original_), 0);
  }

 private:
  rlimit original_{};
};

std::vector<Tool> MakeToolVector(Tool tool) {
  std::vector<Tool> tools;
  tools.push_back(std::move(tool));
  return tools;
}

std::array<Message, 1> MakeInitialMessages(const Message& message) {
  return {message};
}

int FailingGlob(const char* /*pattern*/, int /*flags*/,
                int (*/*errfunc*/)(const char*, int), glob_t* /*result*/) {
  return GLOB_ABORTED;
}

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
  const auto initial = MakeInitialMessages(user_msg);

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

  Agent agent(std::move(mock), cfg, MakeToolVector(MakeWebSearchTool()));

  const Message user_msg{.role = Role::kUser, .content = "Search for Bazel"};
  const auto initial = MakeInitialMessages(user_msg);

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
  const auto initial = MakeInitialMessages(user_msg);

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
  const auto initial = MakeInitialMessages(user_msg);

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

  Agent agent(std::move(mock), cfg, MakeToolVector(MakeWebSearchTool()));

  std::vector<AgentEvent::Type> event_types;
  auto on_event = [&](const AgentEvent& ev) {
    event_types.push_back(ev.type);
  };

  const Message user_msg{.role = Role::kUser, .content = "search"};
  const auto initial = MakeInitialMessages(user_msg);

  auto result = agent.Run(initial, std::move(on_event));
  ASSERT_TRUE(result.ok()) << result.status();

  EXPECT_THAT(event_types, ::testing::Contains(AgentEvent::Type::kToolCall));
  EXPECT_THAT(event_types,
              ::testing::Contains(AgentEvent::Type::kTaskComplete));
}

TEST(BuiltinAgentTest, TrimsContextWhenHistoryGrowsTooLarge) {
  auto mock = std::make_unique<StrictMock<MockLLMClient>>();
  EXPECT_CALL(*mock, Chat(_))
      .WillOnce(Return(MakeToolCallResponse("WebSearch", {{"query", "trim"}})))
      .WillOnce(Invoke([](const ChatRequest& req)
                           -> absl::StatusOr<ChatResponse> {
        EXPECT_EQ(req.messages.size(), 2u);
        EXPECT_EQ(req.messages.front().content, "trim the context");
        return MakeTextResponse("trimmed");
      }));

  AgentConfig cfg;
  cfg.model = "mock-model";
  cfg.max_context_messages = 2;

  Agent agent(std::move(mock), cfg, MakeToolVector(MakeWebSearchTool()));
  const Message user_msg{.role = Role::kUser, .content = "trim the context"};
  const auto initial = MakeInitialMessages(user_msg);

  auto result = agent.Run(initial);
  ASSERT_TRUE(result.ok()) << result.status();
  EXPECT_EQ(result->back().content, "trimmed");
}

TEST(BuiltinAgentTest, ReturnsResourceExhaustedWhenMaxIterationsExceeded) {
  auto mock = std::make_unique<StrictMock<MockLLMClient>>();
  EXPECT_CALL(*mock, Chat(_))
      .WillRepeatedly(Return(MakeToolCallResponse("WebSearch", {{"query", "loop"}})));

  AgentConfig cfg;
  cfg.model = "mock-model";
  cfg.max_iterations = 2;

  Agent agent(std::move(mock), cfg, MakeToolVector(MakeWebSearchTool()));
  const Message user_msg{.role = Role::kUser, .content = "loop forever"};
  const auto initial = MakeInitialMessages(user_msg);

  auto result = agent.Run(initial);
  EXPECT_FALSE(result.ok());
  EXPECT_EQ(result.status().code(), absl::StatusCode::kResourceExhausted);
}

TEST(RoleToStringViewTest, CoversSystemAndUnknownRoles) {
  EXPECT_EQ(RoleToStringView(Role::kSystem), "system");
  EXPECT_EQ(RoleToStringView(static_cast<Role>(255)), "unknown");
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

TEST(BashToolTest, MissingCommandReturnsInvalidArgument) {
  const auto tool = MakeBashTool();
  auto result = tool.execute({});
  EXPECT_FALSE(result.ok());
  EXPECT_EQ(result.status().code(), absl::StatusCode::kInvalidArgument);
}

TEST(BashToolTest, PopenFailureReturnsInternalError) {
  const auto tool = MakeBashTool();
  ScopedFileDescriptorLimit fd_limit(0);
  auto result = tool.execute({{"command", "echo should_fail"}});
  EXPECT_FALSE(result.ok());
  EXPECT_EQ(result.status().code(), absl::StatusCode::kInternal);
}

TEST(ReadToolTest, MissingFileReturnsNotFound) {
  const auto tool = MakeReadTool();
  auto result     = tool.execute({{"file_path", "/nonexistent/path/file.txt"}});
  EXPECT_FALSE(result.ok());
  EXPECT_EQ(result.status().code(), absl::StatusCode::kNotFound);
}

TEST(ReadToolTest, ReadsExistingFile) {
  const auto tool = MakeReadTool();
  const auto dir = MakeTempDir("ohc_read");
  const auto path = dir / "input.txt";
  {
    std::ofstream file(path);
    file << "hello";
  }

  auto result = tool.execute({{"file_path", path.string()}});
  ASSERT_TRUE(result.ok()) << result.status();
  EXPECT_EQ(*result, "hello");

  std::error_code ec;
  std::filesystem::remove_all(dir, ec);
}

TEST(ReadToolTest, MissingFilePathReturnsInvalidArgument) {
  const auto tool = MakeReadTool();
  auto result = tool.execute({});
  EXPECT_FALSE(result.ok());
  EXPECT_EQ(result.status().code(), absl::StatusCode::kInvalidArgument);
}

TEST(WriteToolTest, WritesFileAndCreatesDirectories) {
  const auto tool = MakeWriteTool();
  const auto dir = MakeTempDir("ohc_write");
  const auto path = dir / "nested" / "output.txt";

  auto result = tool.execute(
      {{"file_path", path.string()}, {"content", "payload"}});
  ASSERT_TRUE(result.ok()) << result.status();

  std::ifstream file(path);
  ASSERT_TRUE(file.is_open());
  std::string content((std::istreambuf_iterator<char>(file)), {});
  EXPECT_EQ(content, "payload");

  std::error_code ec;
  std::filesystem::remove_all(dir, ec);
}

TEST(WriteToolTest, ValidatesArgumentsAndFilesystemFailures) {
  const auto tool = MakeWriteTool();

  auto missing_path = tool.execute({{"content", "payload"}});
  EXPECT_FALSE(missing_path.ok());
  EXPECT_EQ(missing_path.status().code(), absl::StatusCode::kInvalidArgument);

  auto missing_content = tool.execute({{"file_path", "/tmp/output.txt"}});
  EXPECT_FALSE(missing_content.ok());
  EXPECT_EQ(missing_content.status().code(),
            absl::StatusCode::kInvalidArgument);

  const auto dir = MakeTempDir("ohc_write_fail");
  const auto parent_file = dir / "parent";
  {
    std::ofstream file(parent_file);
    file << "not a directory";
  }
  auto create_dirs_failure = tool.execute(
      {{"file_path", (parent_file / "child.txt").string()},
       {"content", "payload"}});
  EXPECT_FALSE(create_dirs_failure.ok());

  const auto open_dir = dir / "open_dir";
  std::error_code ec;
  std::filesystem::create_directories(open_dir, ec);
  ASSERT_FALSE(ec) << ec.message();
  auto open_failure = tool.execute(
      {{"file_path", open_dir.string()}, {"content", "payload"}});
  EXPECT_FALSE(open_failure.ok());

  const auto write_failure_dir = dir / "write_failure_dir";
  std::filesystem::create_directories(write_failure_dir, ec);
  ASSERT_FALSE(ec) << ec.message();
  auto write_failure = tool.execute(
      {{"file_path", write_failure_dir.string()}, {"content", "payload"}});
  EXPECT_FALSE(write_failure.ok());

  std::filesystem::remove_all(dir, ec);
}

TEST(GlobToolTest, MatchesFilesAndHandlesValidation) {
  const auto tool = MakeGlobTool();
  const auto dir = MakeTempDir("ohc_glob");
  {
    std::ofstream(dir / "a.txt") << "a";
    std::ofstream(dir / "b.txt") << "b";
  }

  auto result = tool.execute({{"pattern", (dir / "*.txt").string()}});
  ASSERT_TRUE(result.ok()) << result.status();
  EXPECT_THAT(*result, ::testing::HasSubstr("a.txt"));
  EXPECT_THAT(*result, ::testing::HasSubstr("b.txt"));

  auto no_matches = tool.execute({{"pattern", (dir / "*.md").string()}});
  ASSERT_TRUE(no_matches.ok()) << no_matches.status();
  EXPECT_EQ(*no_matches, "No files found matching pattern.");

  auto missing_pattern = tool.execute({});
  EXPECT_FALSE(missing_pattern.ok());
  EXPECT_EQ(missing_pattern.status().code(),
            absl::StatusCode::kInvalidArgument);

  std::error_code ec;
  std::filesystem::remove_all(dir, ec);
}

TEST(GlobToolTest, ReturnsInternalErrorWhenGlobFails) {
  SetGlobFnForTesting(&FailingGlob);
  const auto tool = MakeGlobTool();
  auto result = tool.execute({{"pattern", "*.txt"}});
  ResetGlobFnForTesting();

  EXPECT_FALSE(result.ok());
  EXPECT_EQ(result.status().code(), absl::StatusCode::kInternal);
}

TEST(GrepToolTest, FindsMatchesAndValidatesArguments) {
  const auto tool = MakeGrepTool();
  const auto dir = MakeTempDir("ohc_grep");
  {
    std::ofstream(dir / "notes.txt") << "alpha\nbeta\n";
  }

  auto match = tool.execute(
      {{"pattern", "alpha"}, {"directory", dir.string()}});
  ASSERT_TRUE(match.ok()) << match.status();
  EXPECT_THAT(*match, ::testing::HasSubstr("notes.txt"));

  auto no_match = tool.execute(
      {{"pattern", "gamma"}, {"directory", dir.string()}});
  ASSERT_TRUE(no_match.ok()) << no_match.status();
  EXPECT_EQ(*no_match, "No matches found.");

  auto missing_pattern = tool.execute({{"directory", dir.string()}});
  EXPECT_FALSE(missing_pattern.ok());
  EXPECT_EQ(missing_pattern.status().code(),
            absl::StatusCode::kInvalidArgument);

  auto missing_directory = tool.execute({{"pattern", "alpha"}});
  EXPECT_FALSE(missing_directory.ok());
  EXPECT_EQ(missing_directory.status().code(),
            absl::StatusCode::kInvalidArgument);

  auto not_found = tool.execute(
      {{"pattern", "alpha"}, {"directory", "/definitely/not/here"}});
  EXPECT_FALSE(not_found.ok());
  EXPECT_EQ(not_found.status().code(), absl::StatusCode::kNotFound);

  std::error_code ec;
  std::filesystem::remove_all(dir, ec);
}

TEST(GrepToolTest, PopenFailureReturnsInternalError) {
  const auto tool = MakeGrepTool();
  const auto dir = MakeTempDir("ohc_grep_popen_fail");
  ScopedFileDescriptorLimit fd_limit(0);
  auto result = tool.execute(
      {{"pattern", "alpha"}, {"directory", dir.string()}});
  EXPECT_FALSE(result.ok());
  EXPECT_EQ(result.status().code(), absl::StatusCode::kInternal);

  std::error_code ec;
  std::filesystem::remove_all(dir, ec);
}

TEST(SendMessageToolTest, ValidatesArgumentsAndSucceeds) {
  const auto tool = MakeSendMessageTool();
  auto missing = tool.execute({});
  EXPECT_FALSE(missing.ok());
  EXPECT_EQ(missing.status().code(), absl::StatusCode::kInvalidArgument);

  auto sent = tool.execute({{"message", "hello user"}});
  ASSERT_TRUE(sent.ok()) << sent.status();
  EXPECT_EQ(*sent, "Message sent.");
}

TEST(TodoWriteToolTest, ValidatesArgumentsAndFailureModes) {
  const auto tool = MakeTodoWriteTool();

  auto missing = tool.execute({});
  EXPECT_FALSE(missing.ok());
  EXPECT_EQ(missing.status().code(), absl::StatusCode::kInvalidArgument);

  const auto dir = MakeTempDir("ohc_todo_fail");
  {
    ScopedCurrentDirectory cwd(dir);

    std::ofstream(".agent-task") << "file blocks directory creation";
    auto create_dir_failure = tool.execute({{"todo", "blocked"}});
    EXPECT_FALSE(create_dir_failure.ok());
    std::filesystem::remove(".agent-task");

    std::error_code ec;
    std::filesystem::create_directories(".agent-task/todo.txt", ec);
    ASSERT_FALSE(ec) << ec.message();
    auto open_failure = tool.execute({{"todo", "cannot open"}});
    EXPECT_FALSE(open_failure.ok());
    std::filesystem::remove_all(".agent-task", ec);

    std::filesystem::create_directories(".agent-task", ec);
    ASSERT_FALSE(ec) << ec.message();
    std::filesystem::create_symlink("/dev/full", ".agent-task/todo.txt", ec);
    ASSERT_FALSE(ec) << ec.message();
    auto write_failure = tool.execute({{"todo", "cannot write"}});
    EXPECT_FALSE(write_failure.ok());
  }

  std::error_code ec;
  std::filesystem::remove_all(dir, ec);
}

TEST(ToolSearchToolTest, ListsAndFiltersTools) {
  const auto tool = MakeToolSearchTool();

  auto list_all = tool.execute({});
  ASSERT_TRUE(list_all.ok()) << list_all.status();
  EXPECT_THAT(*list_all, ::testing::HasSubstr("Bash: Execute a bash script."));

  auto filtered = tool.execute({{"query", "fetch"}});
  ASSERT_TRUE(filtered.ok()) << filtered.status();
  EXPECT_THAT(*filtered, ::testing::HasSubstr("WebFetch"));

  auto no_match = tool.execute({{"query", "does-not-exist"}});
  ASSERT_TRUE(no_match.ok()) << no_match.status();
  EXPECT_EQ(*no_match, "No tools found matching query.");
}

TEST(DefaultToolsTest, ReturnsAllBuiltinTools) {
  const auto tools = MakeDefaultTools();
  EXPECT_EQ(tools.size(), 10u);
  EXPECT_EQ(tools.front().name, "Bash");
  EXPECT_EQ(tools.back().name, "ToolSearch");
}

}  // namespace
}  // namespace ohc::agent
