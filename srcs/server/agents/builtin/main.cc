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

#include <csignal>
#include <cstdlib>
#include <memory>
#include <string>

#include "srcs/server/agents/builtin/grpc/agent_service.h"
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

// grpc::Server pointer for graceful shutdown.
grpc::Server* g_server = nullptr;  // NOLINT(cppcoreguidelines-avoid-non-const-global-variables)

void HandleSignal(int sig) {
  LOG(INFO) << "Received signal " << sig << " – initiating graceful shutdown.";
  if (g_server) {
    g_server->Shutdown();
  }
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

  const std::string address =
      absl::StrCat("0.0.0.0:", absl::GetFlag(FLAGS_port));

  // Build the service with the default factory.
  ohc::agent::AgentServiceImpl service(
      ohc::agent::MakeDefaultAgentFactory());

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

  g_server = server.get();

  LOG(INFO) << "OHC Builtin Agent listening on " << address
            << " [provider=" << absl::GetFlag(FLAGS_llm_provider)
            << " model="    << absl::GetFlag(FLAGS_model) << "]";

  server->Wait();

  LOG(INFO) << "Agent server stopped.";
  return EXIT_SUCCESS;
}
