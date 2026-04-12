#include "srcs/server/agents/builtin/grpc/sub_agent_client.h"

#include <chrono>
#include <memory>
#include <string>
#include <utility>

#include "absl/log/log.h"
#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "grpcpp/grpcpp.h"

#include "srcs/proto/agent_service.grpc.pb.h"
#include "srcs/proto/agent_service.pb.h"

namespace ohc::agent {

using grpc::Channel;
using grpc::ClientContext;
using ohc::agent::service::AgentService;
using ohc::agent::service::PingRequest;
using ohc::agent::service::PingResponse;
using ohc::agent::service::SubAgentRequest;
using ohc::agent::service::SubAgentResponse;

SubAgentClient::SubAgentClient(absl::string_view address) {
  auto channel = grpc::CreateChannel(std::string(address),
                                     grpc::InsecureChannelCredentials());
  stub_ = AgentService::NewStub(std::move(channel));
}

absl::StatusOr<std::string> SubAgentClient::Dispatch(
    absl::string_view task,
    const service::AgentRuntimeConfig& runtime_config) {
  SubAgentRequest req;
  req.set_task(std::string(task));
  *req.mutable_runtime_config() = runtime_config;

  SubAgentResponse resp;
  ClientContext ctx;
  // Give sub-agents a generous deadline for long-running tasks.
  ctx.set_deadline(std::chrono::system_clock::now() + std::chrono::minutes(30));

  const grpc::Status status = stub_->DispatchToSubAgent(&ctx, req, &resp);
  if (!status.ok()) {
    return absl::InternalError(absl::StrCat(
        "SubAgentClient::Dispatch gRPC error: ", status.error_message()));
  }
  if (!resp.error().empty()) {
    return absl::InternalError(
        absl::StrCat("Sub-agent returned error: ", resp.error()));
  }
  return resp.result();
}

absl::StatusOr<std::string> SubAgentClient::Dispatch(
    absl::string_view task, absl::string_view model,
    absl::string_view provider, absl::string_view endpoint,
    absl::string_view system, int32_t max_tokens, float temperature,
    int32_t max_iterations, int32_t max_context_messages) {
  service::AgentRuntimeConfig runtime_config;
  runtime_config.set_model(std::string(model));
  runtime_config.set_llm_provider(std::string(provider));
  runtime_config.set_llm_endpoint(std::string(endpoint));
  runtime_config.set_system_prompt(std::string(system));
  runtime_config.set_max_tokens(max_tokens);
  runtime_config.set_temperature(temperature);
  runtime_config.set_max_iterations(max_iterations);
  runtime_config.set_max_context_messages(max_context_messages);
  return Dispatch(task, runtime_config);
}

bool SubAgentClient::Ping() {
  PingRequest req;
  PingResponse resp;
  ClientContext ctx;
  ctx.set_deadline(std::chrono::system_clock::now() + std::chrono::seconds(5));
  return stub_->Ping(&ctx, req, &resp).ok();
}

}  // namespace ohc::agent
