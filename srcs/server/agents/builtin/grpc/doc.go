// Package agentgrpc provides the gRPC client for connecting to the builtin
// agent service (the standalone Rust ohc-builtin-agent binary).
//
// Architecture:
//
//	Go server ──► agentgrpc.Client ──► gRPC stream ──► Rust ohc-builtin-agent
//
// The Client connects to the Rust agent binary running at OHC_AGENT_ADDRESS
// (default: 127.0.0.1:50051) and dispatches tasks via the AgentService proto.
package agentgrpc
