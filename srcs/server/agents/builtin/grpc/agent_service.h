#pragma once

// gRPC service implementation for AgentService.
// Runs inside the C++ agent process and handles requests from the Go main
// server (RunTask) as well as sub-agent dispatch requests.

#include <cstdint>
#include <memory>
#include <string>

#include "srcs/server/agents/builtin/agent.h"
#include "absl/functional/any_invocable.h"
#include "absl/strings/string_view.h"
#include "grpcpp/grpcpp.h"

// Generated gRPC/proto headers (produced by proto_cc_grpc_library).
#include "srcs/proto/agent_service.grpc.pb.h"

namespace ohc::agent {

// AgentServiceImpl is the gRPC server-side handler.
// It is constructed with a factory function so that each RPC call can create
// a fresh Agent instance (fresh context window, no cross-request state).
class AgentServiceImpl final
    : public ohc::agent::service::AgentService::Service {
 public:
  using AgentFactory =
      absl::AnyInvocable<std::unique_ptr<Agent>(const std::string& model,
                                                const std::string& provider,
                                                const std::string& endpoint,
                                                const std::string& system,
                                                int32_t max_tokens,
                                                float temperature,
                                                int32_t max_iterations,
                                                int32_t max_context_messages)>;

  explicit AgentServiceImpl(AgentFactory factory);
  AgentServiceImpl(
      AgentFactory factory,
      const ohc::agent::service::AgentRuntimeConfig& default_runtime_config);

  // Streams RunTaskEvent messages back to the caller as the agent works.
  grpc::Status RunTask(
      grpc::ServerContext* ctx,
      const ohc::agent::service::RunTaskRequest* req,
      grpc::ServerWriter<ohc::agent::service::RunTaskEvent>* writer) override;

  // Health check.
  grpc::Status Ping(
      grpc::ServerContext* ctx,
      const ohc::agent::service::PingRequest* req,
      ohc::agent::service::PingResponse* resp) override;

  // Dispatches a task to a sub-agent and returns its final answer.
  grpc::Status DispatchToSubAgent(
      grpc::ServerContext* ctx,
      const ohc::agent::service::SubAgentRequest* req,
      ohc::agent::service::SubAgentResponse* resp) override;

 private:
  AgentFactory factory_;
  ohc::agent::service::AgentRuntimeConfig default_runtime_config_;
};

// Returns a default AgentFactory that creates agents with MakeDefaultTools().
AgentServiceImpl::AgentFactory MakeDefaultAgentFactory();

}  // namespace ohc::agent
