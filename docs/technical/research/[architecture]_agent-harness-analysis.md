# [architecture] Advanced Multi-Backend Agent Harness Engine

<style>
  .glass-card {
    background: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border-radius: 16px;
    border: 1px solid rgba(255, 255, 255, 0.2);
    box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
    padding: 24px;
    margin: 16px 0;
    color: #ffffff;
    font-family: 'Outfit', 'Inter', sans-serif;
  }
</style>

<div class="glass-card">
  <h2>Problem Statement</h2>
  <p>To achieve true Absolute Autonomy and Swarm Intelligence, OHC agents need a highly robust, secure, and flexible execution environment (the "Agent Harness"). Current analysis of leading frameworks (Claude Code, Hermes Agent, Gstack, OpenClaw) reveals that OHC lacks a unified, multi-backend execution engine with reliable state synchronization and lifecycle management.</p>
</div>

## Title
Implement Advanced Multi-Backend Agent Harness Engine

## Priority
P0

## Estimated Scope
Large

<div class="glass-card">
  <h2>Research Report & Competitive Analysis</h2>

  <p>We analyzed the agent harness architectures of four leading projects:</p>
  <ul>
    <li><b>Claude Code (Leaked):</b> Features an advanced `SandboxManager` with strict security validations (path traversal, blocked commands) over terminal tasks.</li>
    <li><b>Hermes Agent:</b> Implements a flexible multi-backend environment (Docker, Modal, SSH) with a `FileSyncManager` to synchronize local and remote state transparently.</li>
    <li><b>Gstack:</b> Relies on strong browser automation (Playwright) and a robust CLI workflow, with a focus on comprehensive reviews rather than strict sandbox isolation.</li>
    <li><b>OpenClaw:</b> Provides a standardized `AgentHarness` interface (`types.ts`) managing the execution lifecycle (runAttempt, compact, reset) and a robust `SandboxBackendManager`.</li>
  </ul>

  <h3>Comparative Table: OHC vs Market</h3>
  <table>
    <thead>
      <tr>
        <th>Feature</th>
        <th>Claude Code</th>
        <th>Hermes Agent</th>
        <th>OpenClaw</th>
        <th>OHC-HA (Current)</th>
        <th>OHC-HA (Target)</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <td>Multi-Backend Support</td>
        <td>No</td>
        <td>Yes</td>
        <td>Yes</td>
        <td>Partial (K8s/Local)</td>
        <td>Full (Local, Docker, Remote)</td>
      </tr>
      <tr>
        <td>File State Sync</td>
        <td>N/A</td>
        <td>Yes</td>
        <td>Yes</td>
        <td>No</td>
        <td>Yes (Delta-sync)</td>
      </tr>
      <tr>
        <td>Harness Lifecycle API</td>
        <td>Partial</td>
        <td>Partial</td>
        <td>Yes</td>
        <td>No</td>
        <td>Yes</td>
      </tr>
      <tr>
        <td>Security Sandboxing</td>
        <td>High</td>
        <td>Medium</td>
        <td>Medium</td>
        <td>Low</td>
        <td>High</td>
      </tr>
    </tbody>
  </table>

  <h3>Architecture Flow</h3>
  <pre class="mermaid">
  graph TD
      A[Agent Orchestrator] -->|Dispatch Task| B(Harness Lifecycle)
      B --> C{Sandbox Backend Manager}
      C -->|Local| D[Local Sandbox]
      C -->|Container| E[Docker Backend]
      C -->|Remote| F[Remote K8s/Modal Backend]
      D -.->|Sync| G[File Sync Bridge]
      E -.->|Sync| G
      F -.->|Sync| G
      G --> H[(OHC-SIP Central Database)]
  </pre>
</div>

<div class="glass-card">
  <h2>Design Doc</h2>
  <p>The new Agent Harness module will reside in <code>src/server/harness/</code> and consists of three core components:</p>

  <h3>1. SandboxManager</h3>
  <p>Provides a unified <code>SandboxBackend</code> interface with implementations for Local, Docker, and K8s execution. Includes a strict validation layer for security.</p>

  <h3>2. FileSyncBridge</h3>
  <p>A high-performance utility to synchronize files between the OHC orchestrator node and the remote sandbox. Utilizes hashing/mtime to detect changes and syncs deltas. Protected by distributed Redis locks.</p>

  <h3>3. HarnessLifecycle</h3>
  <p>Defines the agent execution flow: <code>StartSession</code>, <code>RunAttempt</code>, <code>CompactContext</code>, <code>ResetSession</code>. Emits OpenTelemetry metrics for every state transition.</p>

  <h3>API Contracts</h3>
  <pre><code class="language-go">
package harness

import "context"

type SandboxBackend interface {
    ExecuteCommand(ctx async context, cmd string) (*ExecutionResult, error)
    ReadFile(ctx async context, path string) ([]byte, error)
    WriteFile(ctx async context, path string, content []byte) error
}

type HarnessLifecycle interface {
    RunAttempt(ctx async context, agentID string, prompt string) (*AttemptResult, error)
    ResetSession(ctx async context, sessionID string) error
}
  </code></pre>
</div>

<div class="glass-card">
  <h2>Implementation Prompt</h2>
  <p>Implement the Agent Harness architecture in Go under <code>src/server/harness/</code>:</p>
  <ol>
    <li>Create <code>sandbox.rs</code> defining the <code>SandboxBackend</code> interface and a <code>DockerBackend</code> implementation. Use <code>OHCMultitenant</code> env var to conditionally enable K8s support.</li>
    <li>Create <code>sync.rs</code> for the <code>FileSyncBridge</code> using a struct that hashes files and syncs deltas. Protect the state with distributed Redis locks.</li>
    <li>Create <code>lifecycle.rs</code> defining the <code>HarnessLifecycle</code> interface.</li>
    <li>Ensure 100% unit test coverage for all new files.</li>
    <li>All metrics MUST be exported using OpenTelemetry.</li>
    <li>Verify implementation via <code>bazelisk test //src/server/harness/...</code>.</li>
  </ol>
</div>
