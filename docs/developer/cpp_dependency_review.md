# C++ Dependency Review

This review is scoped to commercially compatible libraries from
`mozilla-ai/agent.cpp`, `yhirose/cpp-httplib`, and the curated candidates in
`fffaraz/awesome-cpp`.

## Adopted now

- `cpp-httplib` (`MIT`): adopted for the builtin agent HTTP/HTTPS client. It is
  small, header-only, Bazel-friendly, and works with the existing BoringSSL
  dependency graph for TLS.
- `rules_fuzzing` (`Apache-2.0`): adopted for parser fuzz targets so the C++
  agent can exercise invalid LLM payloads under libFuzzer-compatible tooling.

## Keep as-is

- `abseil-cpp` (`Apache-2.0`): already matches the repo's utility and callback
  needs.
- `nlohmann_json` (`MIT`): still acceptable for schema-shaped request/response
  payloads.
- `gRPC` (`BSD-3-Clause`) and `protobuf` (`BSD-3-Clause`): already aligned with
  the builtin agent transport layer.

## Not adopted directly

- `mozilla-ai/agent.cpp` (`MIT`): useful as a design reference, but not a
  direct runtime dependency for this repository today.

`agent.cpp` is explicitly built around local `llama.cpp` inference, GGUF model
loading, shared model weights, and a CMake/submodule integration model. The
current builtin agent is a Bazel-native service that talks to OpenAI,
Anthropic, and Ollama over HTTP. Replacing the existing runtime with
`agent.cpp` would require a deeper model abstraction rewrite first.

The compatible parts worth borrowing later are its lifecycle callback surface
and its clean tool abstraction, not its `Model` implementation.

## Commercial-safe shortlist from awesome-cpp

### Networking

- `cpp-httplib` (`MIT`): good fit for the current HTTP client/server needs.
- `ada` (`Apache-2.0`/`MIT`): strong candidate if URL parsing becomes hot or
  needs stricter WHATWG compliance.
- `c-ares` (`MIT`): optional if the repo needs explicit async DNS control.

### JSON

- `simdjson` (`Apache-2.0`): good candidate if JSON parsing becomes a proven
  hotspot.
- `yyjson` (`MIT`): another strong option for high-throughput parsing.

### CLI and config

- `CLI11` (`BSD`): good candidate if the builtin agent binary grows more
  command-line surface area.
- `toml++` (`MIT`): suitable if typed local config files become necessary.

### Logging and diagnostics

- `spdlog` (`MIT`): reasonable if structured file logging is needed beyond the
  current Abseil usage.
- `backward-cpp` (`MIT`): good candidate for crash-time stack traces.

### Memory and runtime diagnostics

- `mimalloc` (`MIT`): candidate allocator for performance or fragmentation
  testing.
- `heaptrack` (`LGPL-2.1`): acceptable as a development-only profiler, not as a
  bundled runtime dependency.
- `Valgrind` (`GPL`): acceptable as a development-only tool, not as a linked
  production dependency.

## License guardrails

Avoid adding copyleft or proprietary-encumbered runtime dependencies to the
core agent path without an explicit product decision.

That means avoiding or isolating libraries in categories such as:

- `AGPL` server/runtime libraries
- `GPL` or `LGPL` runtime dependencies unless dynamically isolated on purpose
- dual-license packages that require paid commercial terms for normal use

## Proto policy

- `//srcs/proto:agent_service_go_proto` is the Go source of truth for the agent
  service protobufs.
- `AgentRuntimeConfig` and `BuiltinAgentProcessConfig` are the preferred
  config surfaces for the standalone C++ builtin agent and its inter-process
  control plane.
- Checked-in generated `*.pb.go` files should stay out of the repository.

## Fuzzing policy

- Fuzz pure parsers and request normalizers first.
- Keep sockets, network timing, and external services out of the fuzz harness.
- Run parser fuzz targets under `bazelisk run --config=fuzz` so
  AddressSanitizer and LeakSanitizer stay enabled during fuzzing.
