#include "srcs/server/agents/builtin/grpc/agent_service.h"

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "srcs/server/agents/builtin/llm_client.h"
#include "srcs/server/agents/builtin/tools/all_tools.h"
#include "srcs/server/agents/builtin/grpc/sub_agent_client.h"
#include "srcs/server/agents/builtin/types.h"
#include "srcs/proto/agent_service.grpc.pb.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "grpcpp/grpcpp.h"
#include "gtest/gtest.h"

namespace ohc::agent {
namespace {

using grpc::Channel;
using grpc::ClientContext;
using grpc::Server;
using grpc::ServerBuilder;
using grpc::Status;
using ohc::agent::service::AgentService;
using ohc::agent::service::AgentRuntimeConfig;
using ohc::agent::service::PingRequest;
using ohc::agent::service::PingResponse;
using ohc::agent::service::RunTaskEvent;
using ohc::agent::service::RunTaskRequest;
using ohc::agent::service::SubAgentRequest;
using ohc::agent::service::SubAgentResponse;

Message MakeAssistantMessage(absl::string_view content) {
  Message message;
  message.role = Role::kAssistant;
  message.content = std::string(content);
  return message;
}

ChatResponse MakeToolCallResponse(absl::string_view tool_name,
                                  const nlohmann::json& args) {
  ChatResponse response;
  response.message.role = Role::kAssistant;
  response.message.tool_calls.push_back(
      ToolCall{.id = "call_1", .name = std::string(tool_name), .arguments = args});
  return response;
}

class SequenceLLMClient final : public LLMClient {
 public:
  explicit SequenceLLMClient(std::vector<absl::StatusOr<ChatResponse>> responses)
      : responses_(std::move(responses)) {}

  absl::StatusOr<ChatResponse> Chat(const ChatRequest& /*req*/) override {
    EXPECT_LT(index_, responses_.size());
    return responses_[index_++];
  }

 private:
  std::vector<absl::StatusOr<ChatResponse>> responses_;
  size_t index_ = 0;
};

std::unique_ptr<Agent> MakeAgentForResponses(
    std::vector<absl::StatusOr<ChatResponse>> responses) {
  AgentConfig config;
  config.model = "test-model";
  config.system = "test-system";
  return std::make_unique<Agent>(
      std::make_unique<SequenceLLMClient>(std::move(responses)), config,
      MakeDefaultTools());
}

class TestGrpcServer {
 public:
  explicit TestGrpcServer(AgentService::Service* service) {
    ServerBuilder builder;
    builder.AddListeningPort("127.0.0.1:0", grpc::InsecureServerCredentials(),
                             &port_);
    builder.RegisterService(service);
    server_ = builder.BuildAndStart();
    EXPECT_NE(server_, nullptr);
  }

  ~TestGrpcServer() {
    if (server_ != nullptr) {
      server_->Shutdown();
    }
  }

  std::string address() const {
    return "127.0.0.1:" + std::to_string(port_);
  }

 private:
  std::unique_ptr<Server> server_;
  int port_ = 0;
};

class StaticSubAgentService final : public AgentService::Service {
 public:
  Status RunTask(grpc::ServerContext* /*ctx*/, const RunTaskRequest* /*req*/,
                 grpc::ServerWriter<RunTaskEvent>* /*writer*/) override {
    return Status::OK;
  }

  Status Ping(grpc::ServerContext* /*ctx*/, const PingRequest* /*req*/,
              PingResponse* resp) override {
    resp->set_agent_id("sub-agent");
    resp->set_version("1.0");
    return Status::OK;
  }

  Status DispatchToSubAgent(grpc::ServerContext* /*ctx*/,
                            const SubAgentRequest* /*req*/,
                            SubAgentResponse* resp) override {
    resp->set_result("remote-result");
    return Status::OK;
  }
};

class ErrorSubAgentService final : public AgentService::Service {
 public:
  Status RunTask(grpc::ServerContext* /*ctx*/, const RunTaskRequest* /*req*/,
                 grpc::ServerWriter<RunTaskEvent>* /*writer*/) override {
    return Status::OK;
  }

  Status Ping(grpc::ServerContext* /*ctx*/, const PingRequest* /*req*/,
              PingResponse* resp) override {
    resp->set_agent_id("sub-agent");
    resp->set_version("1.0");
    return Status::OK;
  }

  Status DispatchToSubAgent(grpc::ServerContext* /*ctx*/,
                            const SubAgentRequest* /*req*/,
                            SubAgentResponse* resp) override {
    resp->set_error("remote-error");
    return Status::OK;
  }
};

TEST(AgentServiceTest, PingAndRunTaskStreamWork) {
  AgentServiceImpl service([](const std::string& /*model*/,
                              const std::string& /*provider*/,
                              const std::string& /*endpoint*/,
                              const std::string& /*system*/,
                              int32_t /*max_tokens*/,
                              float /*temperature*/,
                              int32_t /*max_iterations*/,
                              int32_t /*max_context_messages*/) {
    ChatResponse response;
    response.message = MakeAssistantMessage("finished");
    std::vector<absl::StatusOr<ChatResponse>> responses;
    responses.push_back(std::move(response));
    return MakeAgentForResponses(std::move(responses));
  });
  TestGrpcServer server(&service);

  auto channel = grpc::CreateChannel(server.address(), grpc::InsecureChannelCredentials());
  auto stub = AgentService::NewStub(channel);

  ClientContext ping_ctx;
  PingRequest ping_req;
  PingResponse ping_resp;
  auto ping_status = stub->Ping(&ping_ctx, ping_req, &ping_resp);
  ASSERT_TRUE(ping_status.ok()) << ping_status.error_message();
  EXPECT_EQ(ping_resp.agent_id(), "builtin-cpp-agent");

  RunTaskRequest run_req;
  run_req.set_task("do work");
  run_req.mutable_runtime_config()->set_model("model");
  run_req.mutable_runtime_config()->set_llm_provider("openai");
  ClientContext run_ctx;
  auto reader = stub->RunTask(&run_ctx, run_req);
  RunTaskEvent event;
  std::vector<service::EventType> event_types;
  while (reader->Read(&event)) {
    event_types.push_back(event.type());
  }
  auto run_status = reader->Finish();
  ASSERT_TRUE(run_status.ok()) << run_status.error_message();
  ASSERT_EQ(event_types.size(), 4u);
  EXPECT_EQ(event_types[0], service::RUN_STARTED);
  EXPECT_EQ(event_types[1], service::ITERATION_STARTED);
  EXPECT_EQ(event_types[2], service::TEXT_CHUNK);
  EXPECT_EQ(event_types[3], service::TASK_COMPLETE);
}

TEST(AgentServiceTest, RunTaskStreamsToolCallEventsAndUsesCustomSystemPrompt) {
  AgentServiceImpl service([](const std::string& /*model*/,
                              const std::string& /*provider*/,
                              const std::string& /*endpoint*/,
                              const std::string& system,
                              int32_t /*max_tokens*/,
                              float /*temperature*/,
                              int32_t /*max_iterations*/,
                              int32_t /*max_context_messages*/) {
    EXPECT_EQ(system, "custom-system");
    std::vector<absl::StatusOr<ChatResponse>> responses;
    responses.push_back(
        MakeToolCallResponse("SendMessage", {{"message", "hello"}}));
    ChatResponse final_response;
    final_response.message = MakeAssistantMessage("finished");
    responses.push_back(std::move(final_response));
    return MakeAgentForResponses(std::move(responses));
  });
  TestGrpcServer server(&service);

  auto channel =
      grpc::CreateChannel(server.address(), grpc::InsecureChannelCredentials());
  auto stub = AgentService::NewStub(channel);

  RunTaskRequest run_req;
  run_req.set_task("do work");
  run_req.mutable_runtime_config()->set_system_prompt("custom-system");
  ClientContext run_ctx;
  auto reader = stub->RunTask(&run_ctx, run_req);
  RunTaskEvent event;
  std::vector<service::EventType> event_types;
  while (reader->Read(&event)) {
    event_types.push_back(event.type());
  }
  auto status = reader->Finish();
  ASSERT_TRUE(status.ok()) << status.error_message();
  ASSERT_EQ(event_types.size(), 6u);
  EXPECT_EQ(event_types[0], service::RUN_STARTED);
  EXPECT_EQ(event_types[1], service::ITERATION_STARTED);
  EXPECT_EQ(event_types[2], service::TOOL_CALL);
  EXPECT_EQ(event_types[3], service::ITERATION_STARTED);
  EXPECT_EQ(event_types[4], service::TEXT_CHUNK);
  EXPECT_EQ(event_types[5], service::TASK_COMPLETE);
}

TEST(AgentServiceTest, RunTaskUsesProcessDefaultRuntimeConfig) {
  AgentRuntimeConfig default_runtime_config;
  default_runtime_config.set_model("configured-model");
  default_runtime_config.set_llm_provider("anthropic");
  default_runtime_config.set_llm_endpoint("http://127.0.0.1:9992/messages");
  default_runtime_config.set_system_prompt("configured-system");
  default_runtime_config.set_max_tokens(321);
  default_runtime_config.set_temperature(0.15f);
  default_runtime_config.set_max_iterations(7);
  default_runtime_config.set_max_context_messages(11);

  AgentServiceImpl service(
      [](const std::string& model, const std::string& provider,
         const std::string& endpoint, const std::string& system,
         int32_t max_tokens, float temperature, int32_t max_iterations,
         int32_t max_context_messages) {
        EXPECT_EQ(model, "configured-model");
        EXPECT_EQ(provider, "anthropic");
        EXPECT_EQ(endpoint, "http://127.0.0.1:9992/messages");
        EXPECT_EQ(system, "configured-system");
        EXPECT_EQ(max_tokens, 321);
        EXPECT_FLOAT_EQ(temperature, 0.15f);
        EXPECT_EQ(max_iterations, 7);
        EXPECT_EQ(max_context_messages, 11);

        ChatResponse response;
        response.message = MakeAssistantMessage("finished");
        std::vector<absl::StatusOr<ChatResponse>> responses;
        responses.push_back(std::move(response));
        return MakeAgentForResponses(std::move(responses));
      },
      default_runtime_config);
  TestGrpcServer server(&service);

  auto channel =
      grpc::CreateChannel(server.address(), grpc::InsecureChannelCredentials());
  auto stub = AgentService::NewStub(channel);

  RunTaskRequest run_req;
  run_req.set_task("use defaults");
  ClientContext run_ctx;
  auto reader = stub->RunTask(&run_ctx, run_req);
  RunTaskEvent event;
  std::vector<service::EventType> event_types;
  while (reader->Read(&event)) {
    event_types.push_back(event.type());
  }

  auto status = reader->Finish();
  ASSERT_TRUE(status.ok()) << status.error_message();
  ASSERT_EQ(event_types.size(), 4u);
  EXPECT_EQ(event_types[0], service::RUN_STARTED);
  EXPECT_EQ(event_types[1], service::ITERATION_STARTED);
  EXPECT_EQ(event_types[2], service::TEXT_CHUNK);
  EXPECT_EQ(event_types[3], service::TASK_COMPLETE);
}

TEST(AgentServiceTest, RunTaskReturnsGrpcErrorWhenAgentFails) {
  AgentServiceImpl service([](const std::string& /*model*/,
                              const std::string& /*provider*/,
                              const std::string& /*endpoint*/,
                              const std::string& /*system*/,
                              int32_t /*max_tokens*/,
                              float /*temperature*/,
                              int32_t /*max_iterations*/,
                              int32_t /*max_context_messages*/) {
    std::vector<absl::StatusOr<ChatResponse>> responses;
    responses.push_back(absl::InternalError("llm failed"));
    return MakeAgentForResponses(std::move(responses));
  });
  TestGrpcServer server(&service);

  auto channel = grpc::CreateChannel(server.address(), grpc::InsecureChannelCredentials());
  auto stub = AgentService::NewStub(channel);

  RunTaskRequest run_req;
  run_req.set_task("do work");
  ClientContext run_ctx;
  auto reader = stub->RunTask(&run_ctx, run_req);
  RunTaskEvent event;
  std::vector<service::EventType> event_types;
  while (reader->Read(&event)) {
    event_types.push_back(event.type());
  }
  ASSERT_EQ(event_types.size(), 3u);
  EXPECT_EQ(event_types[0], service::RUN_STARTED);
  EXPECT_EQ(event_types[1], service::ITERATION_STARTED);
  EXPECT_EQ(event_types[2], service::TASK_ERROR);
  EXPECT_EQ(event.error(), "llm failed");
  EXPECT_FALSE(reader->Read(&event));
  auto status = reader->Finish();
  EXPECT_FALSE(status.ok());
  EXPECT_EQ(status.error_code(), grpc::StatusCode::INTERNAL);
}

TEST(AgentServiceTest, DispatchToSubAgentHandlesLocalPaths) {
  AgentServiceImpl success_service([](const std::string& /*model*/,
                                      const std::string& /*provider*/,
                                      const std::string& /*endpoint*/,
                                      const std::string& system,
                                      int32_t /*max_tokens*/,
                                      float /*temperature*/,
                                      int32_t /*max_iterations*/,
                                      int32_t /*max_context_messages*/) {
    EXPECT_EQ(system, "custom-system");
    ChatResponse response;
    response.message = MakeAssistantMessage("local-result");
    std::vector<absl::StatusOr<ChatResponse>> responses;
    responses.push_back(std::move(response));
    return MakeAgentForResponses(std::move(responses));
  });

  grpc::ServerContext ctx;
  SubAgentRequest request;
  request.set_task("review");
  request.mutable_runtime_config()->set_system_prompt("custom-system");
  SubAgentResponse response;
  auto status = success_service.DispatchToSubAgent(&ctx, &request, &response);
  ASSERT_TRUE(status.ok()) << status.error_message();
  EXPECT_EQ(response.result(), "local-result");

  AgentServiceImpl empty_service([](const std::string& /*model*/,
                                    const std::string& /*provider*/,
                                    const std::string& /*endpoint*/,
                                    const std::string& /*system*/,
                                    int32_t /*max_tokens*/,
                                    float /*temperature*/,
                                    int32_t /*max_iterations*/,
                                    int32_t /*max_context_messages*/) {
    ChatResponse response;
    response.message = MakeAssistantMessage("");
    std::vector<absl::StatusOr<ChatResponse>> responses;
    responses.push_back(std::move(response));
    return MakeAgentForResponses(std::move(responses));
  });
  SubAgentResponse empty_response;
  status = empty_service.DispatchToSubAgent(&ctx, &request, &empty_response);
  ASSERT_TRUE(status.ok()) << status.error_message();
  EXPECT_EQ(empty_response.result(), "(no response)");

  AgentServiceImpl error_service([](const std::string& /*model*/,
                                    const std::string& /*provider*/,
                                    const std::string& /*endpoint*/,
                                    const std::string& /*system*/,
                                    int32_t /*max_tokens*/,
                                    float /*temperature*/,
                                    int32_t /*max_iterations*/,
                                    int32_t /*max_context_messages*/) {
    std::vector<absl::StatusOr<ChatResponse>> responses;
    responses.push_back(absl::InternalError("agent-error"));
    return MakeAgentForResponses(std::move(responses));
  });
  SubAgentResponse error_response;
  status = error_service.DispatchToSubAgent(&ctx, &request, &error_response);
  ASSERT_TRUE(status.ok()) << status.error_message();
  EXPECT_EQ(error_response.error(), "agent-error");
}

TEST(AgentServiceTest, DispatchToSubAgentSurfacesRemoteErrors) {
  ErrorSubAgentService remote_service;
  TestGrpcServer remote_server(&remote_service);

  AgentServiceImpl local_service([](const std::string& /*model*/,
                                    const std::string& /*provider*/,
                                    const std::string& /*endpoint*/,
                                    const std::string& /*system*/,
                                    int32_t /*max_tokens*/,
                                    float /*temperature*/,
                                    int32_t /*max_iterations*/,
                                    int32_t /*max_context_messages*/) {
    ChatResponse response;
    response.message = MakeAssistantMessage("unused");
    std::vector<absl::StatusOr<ChatResponse>> responses;
    responses.push_back(std::move(response));
    return MakeAgentForResponses(std::move(responses));
  });

  grpc::ServerContext ctx;
  SubAgentRequest request;
  request.set_task("delegate");
  request.set_sub_agent_address(remote_server.address());
  SubAgentResponse response;
  auto status = local_service.DispatchToSubAgent(&ctx, &request, &response);
  ASSERT_TRUE(status.ok()) << status.error_message();
  EXPECT_EQ(response.error(), "Sub-agent returned error: remote-error");
}

TEST(AgentServiceTest, DispatchToSubAgentUsesRemoteAddress) {
  StaticSubAgentService remote_service;
  TestGrpcServer remote_server(&remote_service);

  AgentServiceImpl local_service([](const std::string& /*model*/,
                                    const std::string& /*provider*/,
                                    const std::string& /*endpoint*/,
                                    const std::string& /*system*/,
                                    int32_t /*max_tokens*/,
                                    float /*temperature*/,
                                    int32_t /*max_iterations*/,
                                    int32_t /*max_context_messages*/) {
    ChatResponse response;
    response.message = MakeAssistantMessage("unused");
    std::vector<absl::StatusOr<ChatResponse>> responses;
    responses.push_back(std::move(response));
    return MakeAgentForResponses(std::move(responses));
  });

  grpc::ServerContext ctx;
  SubAgentRequest request;
  request.set_task("delegate");
  request.set_sub_agent_address(remote_server.address());
  SubAgentResponse response;
  auto status = local_service.DispatchToSubAgent(&ctx, &request, &response);
  ASSERT_TRUE(status.ok()) << status.error_message();
  EXPECT_EQ(response.result(), "remote-result");
}

TEST(SubAgentClientTest, PingAndDispatchWork) {
  StaticSubAgentService service;
  TestGrpcServer server(&service);

  SubAgentClient client(server.address());
  EXPECT_TRUE(client.Ping());
  AgentRuntimeConfig runtime_config;
  runtime_config.set_model("sub-model");
  runtime_config.set_llm_provider("ollama");
  auto result = client.Dispatch("task", runtime_config);
  ASSERT_TRUE(result.ok()) << result.status();
  EXPECT_EQ(*result, "remote-result");
}

TEST(SubAgentClientTest, DispatchSurfacesRemoteAndGrpcErrors) {
  ErrorSubAgentService error_service;
  TestGrpcServer error_server(&error_service);

  SubAgentClient error_client(error_server.address());
  auto remote_error = error_client.Dispatch("task");
  EXPECT_FALSE(remote_error.ok());

  SubAgentClient grpc_error_client("127.0.0.1:1");
  auto grpc_error = grpc_error_client.Dispatch("task");
  EXPECT_FALSE(grpc_error.ok());
}

TEST(AgentFactoryTest, BuildsConfiguredProviders) {
  auto factory = MakeDefaultAgentFactory();
  auto openai = factory("m1", "openai", "http://127.0.0.1:9991", "sys", 0,
                        0.0f, 0, 0);
  ASSERT_NE(openai, nullptr);

  auto anthropic =
      factory("m2", "anthropic", "http://127.0.0.1:9992/messages", "sys",
              128, 0.2f, 9, 8);
  ASSERT_NE(anthropic, nullptr);

  auto ollama =
      factory("m3", "ollama", "http://127.0.0.1:9993/api/chat", "sys", 64,
              0.5f, 6, 4);
  ASSERT_NE(ollama, nullptr);
}

}  // namespace
}  // namespace ohc::agent