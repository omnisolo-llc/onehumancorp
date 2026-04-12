// C++ Builtin Agent – main entry point.
//
// This binary exposes an AgentService gRPC server.  The main Go server
// connects to it via gRPC to dispatch tasks; sub-agent processes run
// identical binaries with different --llm_provider / --system_prompt flags.
//
// Usage:
//   agent --port=50051 --llm_provider=ollama --model=llama3
//   agent --port=50052 --llm_provider=openai  --model=gpt-4o \
//         --system_prompt="You are a security specialist."

#include <atomic>
#include <csignal>
#include <cstdlib>
#include <fstream>
#include <memory>
#include <string>
#include <unistd.h>

#include "srcs/server/agents/builtin/grpc/agent_service.h"
#include "srcs/proto/agent_service.pb.h"
#include "absl/flags/flag.h"
#include "absl/flags/parse.h"
#include "absl/flags/usage.h"
#include "absl/log/initialize.h"
#include "absl/log/log.h"
#include "absl/strings/str_cat.h"
#include "grpcpp/grpcpp.h"

// --------------------------------------------------------------------------
// Flags
// --------------------------------------------------------------------------

ABSL_FLAG(int32_t, port, 50051,
          "TCP port on which the gRPC server listens.");

ABSL_FLAG(std::string, config_proto_path, "",
          "Path to a binary BuiltinAgentProcessConfig protobuf written by the "
          "Go server.");

ABSL_FLAG(std::string, llm_provider, "ollama",
          "LLM backend: 'openai' | 'anthropic' | 'ollama'. "
          "Defaults to 'ollama' (local, zero egress cost – ideal for Pi).");

ABSL_FLAG(std::string, model, "llama3",
          "Model name passed to the LLM backend.");

ABSL_FLAG(std::string, llm_endpoint, "",
          "Optional endpoint URL override (e.g. http://localhost:11434/api/chat "
          "for a custom Ollama installation).");

ABSL_FLAG(std::string, system_prompt, "",
          "Custom system prompt.  Empty → use the built-in default.");

ABSL_FLAG(int32_t, max_tokens, 2048,
          "Maximum number of tokens per LLM response.");

ABSL_FLAG(float, temperature, 0.7f,
          "Sampling temperature in [0, 2].");

ABSL_FLAG(int32_t, max_iterations, 50,
          "Maximum ReAct iterations per task before giving up.");

ABSL_FLAG(int32_t, max_context_messages, 100,
          "Trim oldest messages when the context window exceeds this count.");

// --------------------------------------------------------------------------
// Signal handling
// --------------------------------------------------------------------------

namespace {

namespace service = ohc::agent::service;

constexpr int32_t kDefaultPort = 50051;
constexpr absl::string_view kDefaultProvider = "ollama";
constexpr absl::string_view kDefaultModel = "llama3";
constexpr int32_t kDefaultMaxTokens = 2048;
constexpr float kDefaultTemperature = 0.7f;
constexpr int32_t kDefaultMaxIterations = 50;
constexpr int32_t kDefaultMaxContextMessages = 100;

// Atomic pointer to the gRPC server; written once from main, read from the
// signal handler.  std::atomic<T*> is signal-safe for load/store on all
// platforms that provide lock-free pointer atomics (i.e. all modern 32/64-bit
// platforms including Raspberry Pi ARMv7 and AArch64).
std::atomic<grpc::Server*> g_server{nullptr};  // NOLINT(cppcoreguidelines-avoid-non-const-global-variables)

void HandleSignal(int /*sig*/) {
  // LOG is not async-signal-safe; use write() directly for safety.
  const char msg[] = "Signal received – shutting down agent.\n";
  (void)write(STDERR_FILENO, msg, sizeof(msg) - 1);
  grpc::Server* srv = g_server.load(std::memory_order_acquire);
  if (srv) {
    srv->Shutdown();
  }
}

bool HasStringFlagOverride(const std::string& value,
                           absl::string_view default_value) {
  return value != default_value;
}

void ApplyBuiltInDefaults(service::AgentRuntimeConfig* runtime_config) {
  if (runtime_config->llm_provider().empty()) {
    runtime_config->set_llm_provider(std::string(kDefaultProvider));
  }
  if (runtime_config->model().empty()) {
    runtime_config->set_model(std::string(kDefaultModel));
  }
  if (runtime_config->max_tokens() <= 0) {
    runtime_config->set_max_tokens(kDefaultMaxTokens);
  }
  if (runtime_config->temperature() <= 0.0f) {
    runtime_config->set_temperature(kDefaultTemperature);
  }
  if (runtime_config->max_iterations() <= 0) {
    runtime_config->set_max_iterations(kDefaultMaxIterations);
  }
  if (runtime_config->max_context_messages() <= 0) {
    runtime_config->set_max_context_messages(kDefaultMaxContextMessages);
  }
}

bool LoadProcessConfig(service::BuiltinAgentProcessConfig* process_config) {
  const std::string config_proto_path = absl::GetFlag(FLAGS_config_proto_path);
  if (config_proto_path.empty()) {
    return true;
  }

  std::ifstream input(config_proto_path, std::ios::binary);
  if (!input.is_open()) {
    LOG(ERROR) << "Failed to open config protobuf: " << config_proto_path;
    return false;
  }
  if (!process_config->ParseFromIstream(&input)) {
    LOG(ERROR) << "Failed to parse config protobuf: " << config_proto_path;
    return false;
  }
  return true;
}

service::AgentRuntimeConfig ResolveRuntimeConfig(
    const service::BuiltinAgentProcessConfig& process_config) {
  service::AgentRuntimeConfig runtime_config = process_config.runtime_config();

  const std::string provider = absl::GetFlag(FLAGS_llm_provider);
  if (HasStringFlagOverride(provider, kDefaultProvider)) {
    runtime_config.set_llm_provider(provider);
  }

  const std::string model = absl::GetFlag(FLAGS_model);
  if (HasStringFlagOverride(model, kDefaultModel)) {
    runtime_config.set_model(model);
  }

  const std::string endpoint = absl::GetFlag(FLAGS_llm_endpoint);
  if (!endpoint.empty()) {
    runtime_config.set_llm_endpoint(endpoint);
  }

  const std::string system_prompt = absl::GetFlag(FLAGS_system_prompt);
  if (!system_prompt.empty()) {
    runtime_config.set_system_prompt(system_prompt);
  }

  const int32_t max_tokens = absl::GetFlag(FLAGS_max_tokens);
  if (max_tokens != kDefaultMaxTokens) {
    runtime_config.set_max_tokens(max_tokens);
  }

  const float temperature = absl::GetFlag(FLAGS_temperature);
  if (temperature != kDefaultTemperature) {
    runtime_config.set_temperature(temperature);
  }

  const int32_t max_iterations = absl::GetFlag(FLAGS_max_iterations);
  if (max_iterations != kDefaultMaxIterations) {
    runtime_config.set_max_iterations(max_iterations);
  }

  const int32_t max_context_messages =
      absl::GetFlag(FLAGS_max_context_messages);
  if (max_context_messages != kDefaultMaxContextMessages) {
    runtime_config.set_max_context_messages(max_context_messages);
  }

  ApplyBuiltInDefaults(&runtime_config);
  return runtime_config;
}

std::string ResolveListenAddress(
    const service::BuiltinAgentProcessConfig& process_config) {
  std::string address = process_config.listen_address();
  if (address.empty()) {
    address = absl::StrCat("0.0.0.0:", kDefaultPort);
  }

  const int32_t port = absl::GetFlag(FLAGS_port);
  if (port != kDefaultPort) {
    address = absl::StrCat("0.0.0.0:", port);
  }

  return address;
}

}  // namespace

// --------------------------------------------------------------------------
// main
// --------------------------------------------------------------------------

int main(int argc, char** argv) {
  absl::SetProgramUsageMessage(
      "OHC C++ Builtin Agent – autonomous AI agent gRPC server.");
  absl::ParseCommandLine(argc, argv);
  absl::InitializeLog();

  // Install signal handlers for graceful shutdown and config reload.
  std::signal(SIGTERM, HandleSignal);
  std::signal(SIGINT,  HandleSignal);
  // SIGHUP could trigger a config reload; for now just log it.
  std::signal(SIGHUP, [](int) { LOG(INFO) << "SIGHUP received."; });

  service::BuiltinAgentProcessConfig process_config;
  if (!LoadProcessConfig(&process_config)) {
    return EXIT_FAILURE;
  }

  const service::AgentRuntimeConfig runtime_config =
      ResolveRuntimeConfig(process_config);
  const std::string address = ResolveListenAddress(process_config);

  // Build the service with the default factory.
  ohc::agent::AgentServiceImpl service(
      ohc::agent::MakeDefaultAgentFactory(), runtime_config);

  grpc::ServerBuilder builder;
  builder.AddListeningPort(address, grpc::InsecureServerCredentials());
  builder.RegisterService(&service);

  // Tune channel limits for large LLM outputs.
  builder.SetMaxReceiveMessageSize(64 * 1024 * 1024);  // 64 MiB
  builder.SetMaxSendMessageSize(64 * 1024 * 1024);

  const std::unique_ptr<grpc::Server> server = builder.BuildAndStart();
  if (!server) {
    LOG(ERROR) << "Failed to start gRPC server on " << address;
    return EXIT_FAILURE;
  }

  g_server.store(server.get(), std::memory_order_release);

  LOG(INFO) << "OHC Builtin Agent listening on " << address
            << " [provider=" << runtime_config.llm_provider()
            << " model="    << runtime_config.model() << "]";

  server->Wait();

  LOG(INFO) << "Agent server stopped.";
  return EXIT_SUCCESS;
}
