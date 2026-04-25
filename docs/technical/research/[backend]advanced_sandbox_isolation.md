# [backend] KAIROS Advanced Sandbox Isolation

## Problem Statement
The current `bash_sandbox` in OHC uses simple regex-based filtering (`regexp.MustCompile`) to block dangerous commands. This is brittle and easily bypassed. We need a robust, kernel-level isolation mechanism (Namespaces, Cgroups, or lightweight VMs) that provides a "Gold Standard" of security for autonomous agents.

## Research Report
- **Competitor Analysis**: Claude Code uses native sandboxing (likely gVisor or macOS App Sandbox). OpenClaw uses Docker containers.
- **Vulnerabilities**: Regex filters can be bypassed with obfuscated bash commands, symlinks, or environment variable manipulation.
- **Target Architecture**: Transition to `nsjail` or a custom Go implementation using `syscall.Unshare` (CLONE_NEWNS, CLONE_NEWNET, CLONE_NEWPID).

## Design Doc
- **Harness Interface**: Update `ExecutionEnvironment` to support resource limits and network isolation.
- **Namespace Provider**: Create `LinuxNamespaceSandbox` implementing the `ExecutionEnvironment` interface.
- **Resource Limits**: Integrate `cgroups` to limit CPU and Memory usage per agent task.
- **RootFS**: Use a minimal Alpine-based or Scratch-based root filesystem for execution.

## Implementation Prompt
Implement a Linux Namespace-based sandbox in `src/server/bash_sandbox/namespace_sandbox.go`.
1. Use `os/exec` with `SysProcAttr` to set `Cloneflags` for NEWNS, NEWNET, NEWPID, NEWUTS, and NEWIPC.
2. Implement a `pivot_root` or `chroot` mechanism to isolate the filesystem to a temporary workspace.
3. Configure `cgroups` (v2) to limit memory to 256MB and CPU to 0.5 cores.
4. Disable network access by default, or provide a virtual ethernet pair with strict iptables rules.
5. Provide unit tests in `namespace_sandbox_test.go` verifying that the agent cannot see host processes or access host files outside the workspace.

## Priority
P0

## Estimated Scope
Large
