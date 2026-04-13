#include "srcs/server/agents/builtin/http_client.h"
#include "srcs/server/agents/builtin/llm_parsing.h"

#include <arpa/inet.h>
#include <netinet/in.h>
#include <sys/socket.h>
#include <unistd.h>

#include <atomic>
#include <cerrno>
#include <cstring>
#include <map>
#include <mutex>
#include <memory>
#include <optional>
#include <sstream>
#include <string>
#include <thread>
#include <utility>

#include "srcs/server/agents/builtin/llm_anthropic.h"
#include "srcs/server/agents/builtin/llm_ollama.h"
#include "srcs/server/agents/builtin/llm_openai.h"
#include "srcs/server/agents/builtin/tools/all_tools.h"
#include "srcs/server/agents/builtin/types.h"
#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "absl/strings/ascii.h"
#include "absl/strings/str_cat.h"
#include "gtest/gtest.h"
#include "nlohmann/json.hpp"

namespace ohc::agent {
namespace {

struct TestHttpRequest {
  std::string method;
  std::string path;
  std::string body;
  std::map<std::string, std::string> headers;
};

struct TestHttpResponse {
  int32_t status_code = 200;
  std::string body;
  std::string content_type = "application/json";
  std::map<std::string, std::string> headers;
};

class TestHttpServer {
 public:
  using Handler =
      absl::AnyInvocable<TestHttpResponse(const TestHttpRequest&) const>;

  explicit TestHttpServer(Handler handler)
      : handler_(std::move(handler)) {
    listen_fd_ = ::socket(AF_INET, SOCK_STREAM, 0);
    EXPECT_NE(listen_fd_, -1) << std::strerror(errno);

    const int enable = 1;
    EXPECT_EQ(::setsockopt(listen_fd_, SOL_SOCKET, SO_REUSEADDR, &enable,
                           sizeof(enable)),
              0)
        << std::strerror(errno);

    sockaddr_in addr{};
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    addr.sin_port = 0;
    EXPECT_EQ(::bind(listen_fd_, reinterpret_cast<sockaddr*>(&addr),
             sizeof(addr)),
          0)
        << std::strerror(errno);
    EXPECT_EQ(::listen(listen_fd_, 8), 0) << std::strerror(errno);

    socklen_t len = sizeof(addr);
    EXPECT_EQ(::getsockname(listen_fd_, reinterpret_cast<sockaddr*>(&addr),
                &len),
          0)
        << std::strerror(errno);
    port_ = ntohs(addr.sin_port);

    thread_ = std::thread([this] { Serve(); });
  }

  ~TestHttpServer() {
    running_.store(false);
    if (listen_fd_ != -1) {
      ::shutdown(listen_fd_, SHUT_RDWR);
      ::close(listen_fd_);
    }
    if (thread_.joinable()) {
      thread_.join();
    }
  }

  std::string Url(const std::string& path) const {
    return absl::StrCat("http://127.0.0.1:", port_, path);
  }

 private:
  static std::string StatusText(int32_t status_code) {
    switch (status_code) {
      case 200:
        return "OK";
      case 404:
        return "Not Found";
      case 500:
        return "Internal Server Error";
      case 503:
        return "Service Unavailable";
      default:
        return "Custom";
    }
  }

  static bool ReadIntoBuffer(int fd, std::string* buffer) {
    char chunk[4096];
    const ssize_t bytes = ::recv(fd, chunk, sizeof(chunk), 0);
    if (bytes <= 0) {
      return false;
    }
    buffer->append(chunk, static_cast<size_t>(bytes));
    return true;
  }

  static std::optional<TestHttpRequest> ParseRequest(int fd) {
    std::string buffer;
    const std::string header_marker = "\r\n\r\n";
    while (buffer.find(header_marker) == std::string::npos) {
      if (!ReadIntoBuffer(fd, &buffer)) {
        return std::nullopt;
      }
    }

    const size_t header_end = buffer.find(header_marker);
    std::istringstream stream(buffer.substr(0, header_end));
    TestHttpRequest request;

    std::string request_line;
    std::getline(stream, request_line);
    if (!request_line.empty() && request_line.back() == '\r') {
      request_line.pop_back();
    }
    std::istringstream request_line_stream(request_line);
    std::string http_version;
    request_line_stream >> request.method >> request.path >> http_version;

    std::string line;
    size_t content_length = 0;
    while (std::getline(stream, line)) {
      if (!line.empty() && line.back() == '\r') {
        line.pop_back();
      }
      if (line.empty()) {
        continue;
      }
      const size_t colon = line.find(':');
      if (colon == std::string::npos) {
        continue;
      }
      std::string key = line.substr(0, colon);
      std::string value = line.substr(colon + 1);
      absl::StripAsciiWhitespace(&key);
      absl::StripAsciiWhitespace(&value);
      key = absl::AsciiStrToLower(key);
      request.headers.emplace(std::move(key), std::move(value));
    }

    if (const auto it = request.headers.find("content-length");
        it != request.headers.end()) {
      content_length = static_cast<size_t>(std::stoul(it->second));
    }

    while (buffer.size() < header_end + header_marker.size() + content_length) {
      if (!ReadIntoBuffer(fd, &buffer)) {
        return std::nullopt;
      }
    }

    request.body = buffer.substr(header_end + header_marker.size(),
                                 content_length);
    return request;
  }

  static void WriteAll(int fd, const std::string& payload) {
    size_t offset = 0;
    while (offset < payload.size()) {
      const ssize_t written =
          ::send(fd, payload.data() + offset, payload.size() - offset, 0);
      ASSERT_GT(written, 0) << std::strerror(errno);
      offset += static_cast<size_t>(written);
    }
  }

  void Serve() {
    while (running_.load()) {
      sockaddr_in client_addr{};
      socklen_t client_len = sizeof(client_addr);
      const int client_fd =
          ::accept(listen_fd_, reinterpret_cast<sockaddr*>(&client_addr),
                   &client_len);
      if (client_fd == -1) {
        if (!running_.load()) {
          return;
        }
        continue;
      }

      auto request = ParseRequest(client_fd);
      if (request.has_value()) {
        const TestHttpResponse response = handler_(*request);
        std::string payload = absl::StrCat(
            "HTTP/1.1 ", response.status_code, " ",
            StatusText(response.status_code), "\r\n",
            "Content-Length: ", response.body.size(), "\r\n",
            "Content-Type: ", response.content_type, "\r\n",
            "Connection: close\r\n");
        for (const auto& [key, value] : response.headers) {
          absl::StrAppend(&payload, key, ": ", value, "\r\n");
        }
        absl::StrAppend(&payload, "\r\n", response.body);
        WriteAll(client_fd, payload);
      }
      ::close(client_fd);
    }
  }

  int listen_fd_ = -1;
  uint16_t port_ = 0;
  std::atomic<bool> running_{true};
  Handler handler_;
  std::thread thread_;
};

int UnusedTcpPort() {
  const int fd = ::socket(AF_INET, SOCK_STREAM, 0);
  EXPECT_NE(fd, -1);
  sockaddr_in addr{};
  addr.sin_family = AF_INET;
  addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
  addr.sin_port = 0;
  EXPECT_EQ(::bind(fd, reinterpret_cast<sockaddr*>(&addr), sizeof(addr)), 0);
  socklen_t len = sizeof(addr);
  EXPECT_EQ(::getsockname(fd, reinterpret_cast<sockaddr*>(&addr), &len), 0);
  const int port = ntohs(addr.sin_port);
  ::close(fd);
  return port;
}

Message MakeUserMessage(absl::string_view content) {
  Message message;
  message.role = Role::kUser;
  message.content = std::string(content);
  return message;
}

TEST(HttpClientTest, SupportsGetAndPostRequests) {
  TestHttpServer server([](const TestHttpRequest& request) {
    if (request.path == "/echo") {
      return TestHttpResponse{.body = request.body};
    }
    return TestHttpResponse{.body = absl::StrCat(request.method, " ", request.path)};
  });

  auto get_response = HttpDo(HttpRequest{.url = server.Url("/hello")});
  ASSERT_TRUE(get_response.ok()) << get_response.status();
  EXPECT_EQ(get_response->status_code, 200);
  EXPECT_EQ(get_response->body, "GET /hello");

  HttpRequest post_request;
  post_request.url = server.Url("/echo");
  post_request.method = "POST";
  post_request.body = R"({"ping":true})";
  post_request.headers = {{"X-Test", "yes"}};
  auto post_response = HttpDo(post_request);
  ASSERT_TRUE(post_response.ok()) << post_response.status();
  EXPECT_EQ(post_response->status_code, 200);
  EXPECT_EQ(post_response->body, post_request.body);
}

TEST(HttpClientTest, ReturnsErrorWhenServerIsUnavailable) {
  const int port = UnusedTcpPort();
  auto response = HttpDo(HttpRequest{
      .url = absl::StrCat("http://127.0.0.1:", port, "/missing"),
      .timeout_seconds = 2,
  });
  EXPECT_FALSE(response.ok());
}

TEST(HttpClientTest, RejectsUnsupportedMethods) {
  auto response = HttpDo(HttpRequest{
      .url = "http://127.0.0.1/unused",
      .method = "TRACE",
  });
  EXPECT_FALSE(response.ok());
  EXPECT_EQ(response.status().code(), absl::StatusCode::kInvalidArgument);
}

TEST(HttpClientTest, RejectsMalformedUrls) {
  auto bad_scheme = HttpDo(HttpRequest{.url = "ftp://example.com/resource"});
  EXPECT_FALSE(bad_scheme.ok());
  EXPECT_EQ(bad_scheme.status().code(), absl::StatusCode::kInvalidArgument);

  auto missing_authority =
      HttpDo(HttpRequest{.url = "http:///missing-authority"});
  EXPECT_FALSE(missing_authority.ok());
  EXPECT_EQ(missing_authority.status().code(),
            absl::StatusCode::kInvalidArgument);
}

TEST(WebFetchToolTest, FetchesBodyAndSurfacesHttpErrors) {
  TestHttpServer ok_server([](const TestHttpRequest& /*request*/) {
    return TestHttpResponse{.body = "fetched-content"};
  });
  const auto fetch_tool = MakeWebFetchTool();
  auto ok_response = fetch_tool.execute({{"url", ok_server.Url("/page")}});
  ASSERT_TRUE(ok_response.ok()) << ok_response.status();
  EXPECT_EQ(*ok_response, "fetched-content");

  TestHttpServer error_server([](const TestHttpRequest& /*request*/) {
    return TestHttpResponse{.status_code = 404, .body = "missing"};
  });
  auto error_response =
      fetch_tool.execute({{"url", error_server.Url("/missing")}});
  EXPECT_FALSE(error_response.ok());
  EXPECT_EQ(error_response.status().code(), absl::StatusCode::kInternal);

  auto invalid_response = fetch_tool.execute({});
  EXPECT_FALSE(invalid_response.ok());
  EXPECT_EQ(invalid_response.status().code(),
            absl::StatusCode::kInvalidArgument);
}

TEST(OpenAIClientTest, SerializesRequestAndParsesResponse) {
  std::mutex mutex;
  std::optional<TestHttpRequest> captured_request;
  TestHttpServer server([&](const TestHttpRequest& request) {
    std::lock_guard<std::mutex> lock(mutex);
    captured_request = request;
    return TestHttpResponse{
        .body = R"({"choices":[{"message":{"content":"done","tool_calls":[{"id":"call_1","function":{"name":"Read","arguments":"{\"file_path\":\"/tmp/demo\"}"}}]}}]})",
    };
  });

  OpenAIClient client("secret", server.Url(""));
  Message assistant;
  assistant.role = Role::kAssistant;
  ToolCall prior_tool_call;
  prior_tool_call.id = "call_prev";
  prior_tool_call.name = "Write";
  prior_tool_call.arguments = nlohmann::json{{"file_path", "/tmp/out"}};
  assistant.tool_calls.push_back(prior_tool_call);
  Message tool_message;
  tool_message.role = Role::kTool;
  tool_message.tool_results.push_back(
      ToolResult{.tool_call_id = "call_prev", .content = "ok"});

  ToolDefinition tool_definition{
      .name = "Read",
      .description = "Read a file.",
      .parameters = nlohmann::json{{"type", "object"}},
  };

  ChatRequest request;
  request.model = "gpt-test";
  request.system = "system";
  request.messages = {MakeUserMessage("hello"), assistant, tool_message};
  request.tool_definitions = absl::MakeSpan(&tool_definition, 1);
  request.max_tokens = 32;
  request.temperature = 0.25f;

  auto response = client.Chat(request);
  ASSERT_TRUE(response.ok()) << response.status();
  EXPECT_EQ(response->message.content, "done");
  ASSERT_EQ(response->message.tool_calls.size(), 1u);
  EXPECT_EQ(response->message.tool_calls[0].name, "Read");
  EXPECT_EQ(response->message.tool_calls[0].arguments["file_path"], "/tmp/demo");

  std::lock_guard<std::mutex> lock(mutex);
  ASSERT_TRUE(captured_request.has_value());
  EXPECT_EQ(captured_request->path, "/chat/completions");
  EXPECT_EQ(captured_request->headers.at("authorization"), "Bearer secret");
  const auto payload = nlohmann::json::parse(captured_request->body);
  EXPECT_EQ(payload["model"], "gpt-test");
  EXPECT_TRUE(payload.contains("tools"));
}

TEST(OpenAIClientTest, SurfacesHttpAndParseErrors) {
  TestHttpServer error_server([](const TestHttpRequest& /*request*/) {
    return TestHttpResponse{.status_code = 503, .body = "temporary"};
  });
  OpenAIClient error_client("secret", error_server.Url(""));
  ChatRequest request;
  request.model = "gpt-test";
  request.messages = {MakeUserMessage("hello")};
  auto error_response = error_client.Chat(request);
  EXPECT_FALSE(error_response.ok());

  TestHttpServer invalid_server([](const TestHttpRequest& /*request*/) {
    return TestHttpResponse{.body = "not-json"};
  });
  OpenAIClient invalid_client("secret", invalid_server.Url(""));
  auto invalid_response = invalid_client.Chat(request);
  EXPECT_FALSE(invalid_response.ok());
}

TEST(AnthropicClientTest, SerializesRequestAndParsesResponse) {
  std::mutex mutex;
  std::optional<TestHttpRequest> captured_request;
  TestHttpServer server([&](const TestHttpRequest& request) {
    std::lock_guard<std::mutex> lock(mutex);
    captured_request = request;
    return TestHttpResponse{
        .body = R"({"content":[{"type":"text","text":"anthropic-done"},{"type":"tool_use","id":"tool_1","name":"Read","input":{"file_path":"/tmp/input"}}]})",
    };
  });

  AnthropicClient client("anthropic-key", "2023-06-01", server.Url("/messages"));
  Message assistant;
  assistant.role = Role::kAssistant;
  assistant.content = "thinking";
  ToolCall call;
  call.id = "tool_prev";
  call.name = "Write";
  call.arguments = nlohmann::json{{"file_path", "/tmp/out"}};
  assistant.tool_calls.push_back(call);
  Message tool_message;
  tool_message.role = Role::kTool;
  tool_message.tool_results.push_back(
      ToolResult{.tool_call_id = "tool_prev", .content = "ok"});

  ToolDefinition tool_definition{
      .name = "Read",
      .description = "Read file",
      .parameters = nlohmann::json{{"type", "object"}},
  };

  ChatRequest request;
  request.model = "claude-test";
  request.system = "system";
  request.messages = {MakeUserMessage("hello"), assistant, tool_message};
  request.tool_definitions = absl::MakeSpan(&tool_definition, 1);

  auto response = client.Chat(request);
  ASSERT_TRUE(response.ok()) << response.status();
  EXPECT_EQ(response->message.content, "anthropic-done");
  ASSERT_EQ(response->message.tool_calls.size(), 1u);
  EXPECT_EQ(response->message.tool_calls[0].name, "Read");

  std::lock_guard<std::mutex> lock(mutex);
  ASSERT_TRUE(captured_request.has_value());
  EXPECT_EQ(captured_request->headers.at("x-api-key"), "anthropic-key");
  const auto payload = nlohmann::json::parse(captured_request->body);
  EXPECT_EQ(payload["system"], "system");
  EXPECT_TRUE(payload.contains("tools"));
}

TEST(AnthropicClientTest, SurfacesHttpAndParseErrors) {
  TestHttpServer error_server([](const TestHttpRequest& /*request*/) {
    return TestHttpResponse{.status_code = 500, .body = "boom"};
  });
  AnthropicClient error_client("key", "2023-06-01", error_server.Url("/messages"));
  ChatRequest request;
  request.model = "claude-test";
  request.messages = {MakeUserMessage("hello")};
  auto error_response = error_client.Chat(request);
  EXPECT_FALSE(error_response.ok());

  TestHttpServer invalid_server([](const TestHttpRequest& /*request*/) {
    return TestHttpResponse{.body = "invalid"};
  });
  AnthropicClient invalid_client("key", "2023-06-01", invalid_server.Url("/messages"));
  auto invalid_response = invalid_client.Chat(request);
  EXPECT_FALSE(invalid_response.ok());
}

TEST(OllamaClientTest, SerializesRequestAndParsesResponse) {
  std::mutex mutex;
  std::optional<TestHttpRequest> captured_request;
  TestHttpServer server([&](const TestHttpRequest& request) {
    std::lock_guard<std::mutex> lock(mutex);
    captured_request = request;
    return TestHttpResponse{
        .body = R"({"message":{"content":"ollama-done","tool_calls":[{"function":{"name":"Glob","arguments":{"pattern":"*.cc"}}}]}})",
    };
  });

  std::unique_ptr<LLMClient> client =
      std::make_unique<OllamaClient>(server.Url("/api/chat"));
  Message tool_message;
  tool_message.role = Role::kTool;
  tool_message.tool_results.push_back(
      ToolResult{.tool_call_id = "call_1", .content = "tool-output"});

  ToolDefinition tool_definition{
      .name = "Glob",
      .description = "List files",
      .parameters = nlohmann::json{{"type", "object"}},
  };

  ChatRequest request;
  request.model = "llama-test";
  request.system = "system";
  request.messages = {MakeUserMessage("hello"), tool_message};
  request.tool_definitions = absl::MakeSpan(&tool_definition, 1);
  request.max_tokens = 20;
  request.temperature = 0.5f;

  auto response = client->Chat(request);
  ASSERT_TRUE(response.ok()) << response.status();
  EXPECT_EQ(response->message.content, "ollama-done");
  ASSERT_EQ(response->message.tool_calls.size(), 1u);
  EXPECT_EQ(response->message.tool_calls[0].name, "Glob");
  EXPECT_EQ(response->message.tool_calls[0].id, "Glob_0");

  std::lock_guard<std::mutex> lock(mutex);
  ASSERT_TRUE(captured_request.has_value());
  EXPECT_EQ(captured_request->path, "/api/chat");
  const auto payload = nlohmann::json::parse(captured_request->body);
  EXPECT_TRUE(payload.contains("tools"));
}

TEST(OllamaClientTest, SurfacesHttpAndParseErrors) {
  TestHttpServer error_server([](const TestHttpRequest& /*request*/) {
    return TestHttpResponse{.status_code = 500, .body = "boom"};
  });
  std::unique_ptr<LLMClient> error_client =
      std::make_unique<OllamaClient>(error_server.Url("/api/chat"));
  ChatRequest request;
  request.model = "llama-test";
  request.messages = {MakeUserMessage("hello")};
  auto error_response = error_client->Chat(request);
  EXPECT_FALSE(error_response.ok());

  TestHttpServer invalid_server([](const TestHttpRequest& /*request*/) {
    return TestHttpResponse{.body = "invalid"};
  });
  std::unique_ptr<LLMClient> invalid_client =
      std::make_unique<OllamaClient>(invalid_server.Url("/api/chat"));
  auto invalid_response = invalid_client->Chat(request);
  EXPECT_FALSE(invalid_response.ok());
}

TEST(LLMParsingTest, RejectsInvalidJsonBodies) {
  EXPECT_FALSE(ParseOpenAIChatResponse("not-json").ok());
  EXPECT_FALSE(ParseAnthropicChatResponse("not-json").ok());
  EXPECT_FALSE(ParseOllamaChatResponse("not-json").ok());
}

}  // namespace
}  // namespace ohc::agent