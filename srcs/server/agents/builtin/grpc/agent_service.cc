#include "srcs/server/agents/builtin/grpc/agent_service.h"

#include <array>
#include <cstdlib>
#include <memory>
#include <string>
#include <utility>

#include "srcs/server/agents/builtin/agent.h"
#include "srcs/server/agents/builtin/llm_anthropic.h"
#include "srcs/server/agents/builtin/llm_ollama.h"
#include "srcs/server/agents/builtin/llm_openai.h"
#include "srcs/server/agents/builtin/prompt.h"
#include "srcs/server/agents/builtin/grpc/sub_agent_client.h"
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

namespace {

constexpr absl::string_view kDefaultProvider = "ollama";
constexpr absl::string_view kDefaultModel = "llama3";
constexpr int32_t kDefaultMaxTokens = 2048;
constexpr float kDefaultTemperature = 0.7f;
constexpr int32_t kDefaultMaxIterations = 50;
constexpr int32_t kDefaultMaxContextMessages = 100;

void MergeRuntimeConfig(service::AgentRuntimeConfig* target,
                        const service::AgentRuntimeConfig& overrides) {
  if (!overrides.llm_provider().empty()) {
    target->set_llm_provider(overrides.llm_provider());
  }
  if (!overrides.model().empty()) {
    target->set_model(overrides.model());
  }
  if (!overrides.llm_endpoint().empty()) {
    target->set_llm_endpoint(overrides.llm_endpoint());
  }
  if (!overrides.system_prompt().empty()) {
    target->set_system_prompt(overrides.system_prompt());
  }
  if (overrides.max_tokens() > 0) {
    target->set_max_tokens(overrides.max_tokens());
  }
  if (overrides.temperature() > 0.0f) {
    target->set_temperature(overrides.temperature());
  }
  if (overrides.max_iterations() > 0) {
    target->set_max_iterations(overrides.max_iterations());
  }
  if (overrides.max_context_messages() > 0) {
    target->set_max_context_messages(overrides.max_context_messages());
  }
}

service::AgentRuntimeConfig NormalizeRuntimeConfig(
    service::AgentRuntimeConfig config) {
  if (config.llm_provider().empty()) {
    config.set_llm_provider(std::string(kDefaultProvider));
  }
  if (config.model().empty()) {
    config.set_model(std::string(kDefaultModel));
  }
  if (config.system_prompt().empty()) {
    config.set_system_prompt(std::string(kSystemPrompt));
  }
  if (config.max_tokens() <= 0) {
    config.set_max_tokens(kDefaultMaxTokens);
  }
  if (config.temperature() <= 0.0f) {
    config.set_temperature(kDefaultTemperature);
  }
  if (config.max_iterations() <= 0) {
    config.set_max_iterations(kDefaultMaxIterations);
  }
  if (config.max_context_messages() <= 0) {
    config.set_max_context_messages(kDefaultMaxContextMessages);
  }
  return config;
}

service::AgentRuntimeConfig ResolveRuntimeConfig(
    const service::AgentRuntimeConfig& defaults,
    const RunTaskRequest& req) {
  service::AgentRuntimeConfig config = defaults;
  MergeRuntimeConfig(&config, req.runtime_config());
  if (!req.model().empty()) {
    config.set_model(req.model());
  }
  if (!req.llm_provider().empty()) {
    config.set_llm_provider(req.llm_provider());
  }
  if (!req.llm_endpoint().empty()) {
    config.set_llm_endpoint(req.llm_endpoint());
  }
  if (!req.system_prompt().empty()) {
    config.set_system_prompt(req.system_prompt());
  }
  if (req.max_tokens() > 0) {
    config.set_max_tokens(req.max_tokens());
  }
  if (req.temperature() > 0.0f) {
    config.set_temperature(req.temperature());
  }
  if (req.max_context_messages() > 0) {
    config.set_max_context_messages(req.max_context_messages());
  }
  return NormalizeRuntimeConfig(std::move(config));
}

service::AgentRuntimeConfig ResolveRuntimeConfig(
    const service::AgentRuntimeConfig& defaults,
    const SubAgentRequest& req) {
  service::AgentRuntimeConfig config = defaults;
  MergeRuntimeConfig(&config, req.runtime_config());
  if (!req.model().empty()) {
    config.set_model(req.model());
  }
  if (!req.llm_provider().empty()) {
    config.set_llm_provider(req.llm_provider());
  }
  if (!req.llm_endpoint().empty()) {
    config.set_llm_endpoint(req.llm_endpoint());
  }
  if (!req.system_prompt().empty()) {
    config.set_system_prompt(req.system_prompt());
  }
  if (req.max_tokens() > 0) {
    config.set_max_tokens(req.max_tokens());
  }
  if (req.temperature() > 0.0f) {
    config.set_temperature(req.temperature());
  }
  return NormalizeRuntimeConfig(std::move(config));
}

}  // namespace

// ---------------------------------------------------------------------------
// Constructor
// ---------------------------------------------------------------------------

AgentServiceImpl::AgentServiceImpl(AgentFactory factory)
    : AgentServiceImpl(std::move(factory), service::AgentRuntimeConfig()) {}

AgentServiceImpl::AgentServiceImpl(
    AgentFactory factory,
    const service::AgentRuntimeConfig& default_runtime_config)
    : factory_(std::move(factory)),
      default_runtime_config_(default_runtime_config) {}

// ---------------------------------------------------------------------------
// RunTask
// ---------------------------------------------------------------------------

Status AgentServiceImpl::RunTask(ServerContext* /*ctx*/,
                                 const RunTaskRequest* req,
                                 ServerWriter<RunTaskEvent>* writer) {
  const service::AgentRuntimeConfig runtime_config =
      ResolveRuntimeConfig(default_runtime_config_, *req);

  LOG(INFO) << "RunTask: model=" << runtime_config.model()
            << " provider=" << runtime_config.llm_provider()
            << " task=" << req->task().substr(0, 80);

  // Build the initial user message.
  Message user_msg;
  user_msg.role    = Role::kUser;
  user_msg.content = req->task();

  // Construct the agent for this request.
  auto agent = factory_(runtime_config.model(), runtime_config.llm_provider(),
                        runtime_config.llm_endpoint(),
                        runtime_config.system_prompt(),
                        runtime_config.max_tokens(),
                        runtime_config.temperature(),
                        runtime_config.max_iterations(),
                        runtime_config.max_context_messages());

  // Wire the EventCallback to stream events over gRPC.
  auto on_event = [writer](const AgentEvent& ev) {
    RunTaskEvent event;
    switch (ev.type) {
      case AgentEvent::Type::kRunStarted:
        event.set_type(service::RUN_STARTED);
        event.set_message_count(ev.message_count);
        break;
      case AgentEvent::Type::kIterationStarted:
        event.set_type(service::ITERATION_STARTED);
        event.set_iteration(ev.iteration);
        event.set_message_count(ev.message_count);
        break;
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

  const auto initial_messages = std::array<Message, 1>{std::move(user_msg)};
  auto result = agent->Run(initial_messages, std::move(on_event));
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
  const service::AgentRuntimeConfig runtime_config =
      ResolveRuntimeConfig(default_runtime_config_, *req);

  LOG(INFO) << "DispatchToSubAgent: model=" << runtime_config.model()
            << " task=" << req->task().substr(0, 80);

  Message user_msg;
  user_msg.role    = Role::kUser;
  user_msg.content = req->task();

  if (!req->sub_agent_address().empty()) {
    SubAgentClient client(req->sub_agent_address());
    auto result = client.Dispatch(req->task(), runtime_config);
    if (!result.ok()) {
      resp->set_error(std::string(result.status().message()));
      return Status::OK;
    }
    resp->set_result(std::move(*result));
    return Status::OK;
  }

  auto agent = factory_(runtime_config.model(), runtime_config.llm_provider(),
                        runtime_config.llm_endpoint(),
                        runtime_config.system_prompt(),
                        runtime_config.max_tokens(),
                        runtime_config.temperature(),
                        runtime_config.max_iterations(),
                        runtime_config.max_context_messages());

  const auto initial_messages = std::array<Message, 1>{std::move(user_msg)};
  auto result = agent->Run(initial_messages);
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
            float temperature,
            int32_t max_iterations,
            int32_t max_context_messages) -> std::unique_ptr<Agent> {
    std::unique_ptr<LLMClient> client;
    if (provider == "anthropic") {
      // API key read from environment variable ANTHROPIC_API_KEY.
      const char* key = std::getenv("ANTHROPIC_API_KEY");
      client = std::make_unique<AnthropicClient>(
          key ? key : "", "2023-06-01",
          endpoint.empty() ? "https://api.anthropic.com/v1/messages"
                           : endpoint);
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
    cfg.max_tokens  = max_tokens > 0 ? max_tokens : kDefaultMaxTokens;
    cfg.temperature =
        temperature > 0.0f ? temperature : kDefaultTemperature;
    cfg.max_iterations =
        max_iterations > 0 ? max_iterations : kDefaultMaxIterations;
    cfg.max_context_messages =
        max_context_messages > 0 ? max_context_messages
                                 : kDefaultMaxContextMessages;

    return std::make_unique<Agent>(std::move(client), std::move(cfg),
                                   MakeDefaultTools());
  };
}

}  // namespace ohc::agent
