#pragma once

// gRPC client used by the orchestrator agent to dispatch work to sub-agent
// processes.  Each sub-agent runs the same AgentService binary with a
// different --role or --system_prompt flag.

#include <cstdint>
#include <memory>
#include <string>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "grpcpp/grpcpp.h"

// Generated gRPC/proto headers (produced by proto_cc_grpc_library).
#include "srcs/proto/agent_service.grpc.pb.h"

namespace ohc::agent {

// SubAgentClient wraps a gRPC stub and provides a synchronous interface for
// delegating tasks to a remote sub-agent process.
class SubAgentClient {
 public:
  // address: host:port of the sub-agent gRPC server (e.g. "localhost:50052").
  explicit SubAgentClient(absl::string_view address);

  // Sends a task to the sub-agent and blocks until a result is returned.
  absl::StatusOr<std::string> Dispatch(
      absl::string_view task,
      absl::string_view model       = "llama3",
      absl::string_view provider    = "ollama",
      absl::string_view endpoint    = "",
      absl::string_view system      = "",
      int32_t           max_tokens  = 2048,
      float             temperature = 0.7f);

  // Returns true when the sub-agent is reachable.
  bool Ping();

 private:
  std::unique_ptr<ohc::agent::service::AgentService::Stub> stub_;
};

}  // namespace ohc::agent
