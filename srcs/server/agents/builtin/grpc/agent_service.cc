#include "srcs/server/agents/builtin/grpc/agent_service.h"

#include <cstdlib>
#include <memory>
#include <string>
#include <utility>

#include "srcs/server/agents/builtin/agent.h"
#include "srcs/server/agents/builtin/llm_anthropic.h"
#include "srcs/server/agents/builtin/llm_ollama.h"
#include "srcs/server/agents/builtin/llm_openai.h"
#include "srcs/server/agents/builtin/prompt.h"
#include "srcs/server/agents/builtin/tools/all_tools.h"
#include "absl/log/log.h"
#include "absl/strings/string_view.h"

#include "srcs/proto/agent_service.grpc.pb.h"
#include "srcs/proto/agent_service.pb.h"

namespace ohc::agent {

using grpc::ServerContext;
using grpc::ServerWriter;
using grpc::Status;
using ohc::agent::service::AgentService;
using ohc::agent::service::PingRequest;
using ohc::agent::service::PingResponse;
using ohc::agent::service::RunTaskEvent;
using ohc::agent::service::RunTaskRequest;
using ohc::agent::service::SubAgentRequest;
using ohc::agent::service::SubAgentResponse;

// ---------------------------------------------------------------------------
// Constructor
// ---------------------------------------------------------------------------

AgentServiceImpl::AgentServiceImpl(AgentFactory factory)
    : factory_(std::move(factory)) {}

// ---------------------------------------------------------------------------
// RunTask
// ---------------------------------------------------------------------------

Status AgentServiceImpl::RunTask(ServerContext* /*ctx*/,
                                 const RunTaskRequest* req,
                                 ServerWriter<RunTaskEvent>* writer) {
  LOG(INFO) << "RunTask: model=" << req->model()
            << " provider=" << req->llm_provider()
            << " task=" << req->task().substr(0, 80);

  // Build the initial user message.
  Message user_msg;
  user_msg.role    = Role::kUser;
  user_msg.content = req->task();

  // Construct the agent for this request.
  auto agent = factory_(req->model(), req->llm_provider(),
                        req->llm_endpoint(),
                        req->system_prompt().empty()
                            ? std::string(kSystemPrompt)
                            : req->system_prompt(),
                        req->max_tokens(), req->temperature());

  // Wire the EventCallback to stream events over gRPC.
  auto on_event = [writer](const AgentEvent& ev) {
    RunTaskEvent event;
    switch (ev.type) {
      case AgentEvent::Type::kTextChunk:
        event.set_type(service::TEXT_CHUNK);
        event.set_content(ev.content);
        break;
      case AgentEvent::Type::kToolCall:
        event.set_type(service::TOOL_CALL);
        event.set_tool_name(ev.tool_name);
        event.set_tool_args_json(ev.tool_args_json);
        event.set_tool_result(ev.tool_result);
        break;
      case AgentEvent::Type::kTaskComplete:
        event.set_type(service::TASK_COMPLETE);
        event.set_content(ev.content);
        break;
      case AgentEvent::Type::kTaskError:
        event.set_type(service::TASK_ERROR);
        event.set_error(ev.error);
        break;
    }
    writer->Write(event);
  };

  const absl::Span<const Message> initial({user_msg});
  auto result = agent->Run(initial, std::move(on_event));
  if (!result.ok()) {
    return Status(grpc::StatusCode::INTERNAL,
                  std::string(result.status().message()));
  }
  return Status::OK;
}

// ---------------------------------------------------------------------------
// Ping
// ---------------------------------------------------------------------------

Status AgentServiceImpl::Ping(ServerContext* /*ctx*/,
                              const PingRequest* /*req*/,
                              PingResponse* resp) {
  resp->set_agent_id("builtin-cpp-agent");
  resp->set_version("1.0.0");
  return Status::OK;
}

// ---------------------------------------------------------------------------
// DispatchToSubAgent
// ---------------------------------------------------------------------------

Status AgentServiceImpl::DispatchToSubAgent(ServerContext* /*ctx*/,
                                            const SubAgentRequest* req,
                                            SubAgentResponse* resp) {
  LOG(INFO) << "DispatchToSubAgent: model=" << req->model()
            << " task=" << req->task().substr(0, 80);

  Message user_msg;
  user_msg.role    = Role::kUser;
  user_msg.content = req->task();

  auto agent = factory_(req->model(), req->llm_provider(),
                        req->llm_endpoint(),
                        req->system_prompt().empty()
                            ? std::string(kSystemPrompt)
                            : req->system_prompt(),
                        req->max_tokens(), req->temperature());

  const absl::Span<const Message> initial({user_msg});
  auto result = agent->Run(initial);
  if (!result.ok()) {
    resp->set_error(std::string(result.status().message()));
    return Status::OK;
  }

  // Return the last assistant message as the result.
  for (auto it = result->rbegin(); it != result->rend(); ++it) {
    if (it->role == Role::kAssistant && !it->content.empty()) {
      resp->set_result(it->content);
      return Status::OK;
    }
  }
  resp->set_result("(no response)");
  return Status::OK;
}

// ---------------------------------------------------------------------------
// MakeDefaultAgentFactory
// ---------------------------------------------------------------------------

AgentServiceImpl::AgentFactory MakeDefaultAgentFactory() {
  return [](const std::string& model, const std::string& provider,
            const std::string& endpoint, const std::string& system,
            int32_t max_tokens,
            float temperature) -> std::unique_ptr<Agent> {
    std::unique_ptr<LLMClient> client;
    if (provider == "anthropic") {
      // API key read from environment variable ANTHROPIC_API_KEY.
      const char* key = std::getenv("ANTHROPIC_API_KEY");
      client = std::make_unique<AnthropicClient>(key ? key : "");
    } else if (provider == "ollama") {
      client = std::make_unique<OllamaClient>(
          endpoint.empty() ? "http://localhost:11434/api/chat" : endpoint);
    } else {
      // Default: OpenAI-compatible.
      const char* key = std::getenv("OPENAI_API_KEY");
      client = std::make_unique<OpenAIClient>(
          key ? key : "",
          endpoint.empty() ? "https://api.openai.com/v1" : endpoint);
    }

    AgentConfig cfg;
    cfg.model       = model;
    cfg.system      = system;
    cfg.max_tokens  = max_tokens > 0 ? max_tokens : 2048;
    cfg.temperature = temperature > 0.0f ? temperature : 0.7f;

    return std::make_unique<Agent>(std::move(client), std::move(cfg),
                                   MakeDefaultTools());
  };
}

}  // namespace ohc::agent
