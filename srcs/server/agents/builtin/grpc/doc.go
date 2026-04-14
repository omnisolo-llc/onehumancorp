// Package agentgrpc implements the gRPC AgentService for the builtin agent
// using google.golang.org/adk for agent orchestration.
//
// Architecture:
//
//	gRPC stream ──► AgentServiceServer
//	                  │
//	                  ├─► adk runner.Runner
//	                  │     └─► llmagent.New (adk loop)
//	                  │           ├─► adkModelAdapter  ──► Anthropic / OpenAI / Ollama
//	                  │           └─► tool.Toolset(s)
//	                  │                 ├─► builtin functiontool wrappers  (Bash, Grep…)
//	                  │                 └─► mcptoolset.New  (one per MCPServerConfig)
//	                  │
//	                  └─► DispatchToSubAgent
//	                        ├─ in-process goroutine + channel (no sub_agent_address)
//	                        └─ remote gRPC dial               (sub_agent_address set)
package agentgrpc
