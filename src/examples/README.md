<div align="center" style="font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.05); backdrop-filter: blur(30px) saturate(210%); border: 1px solid rgba(255, 255, 255, 0.1); padding: 40px; border-radius: 16px; margin-bottom: 24px;">
  <h1 style="margin-bottom: 12px; font-weight: 700;">One Human Corp Examples</h1>
  <p style="font-size: 1.1em; color: #888; font-weight: 400;"><strong>Pre-configured, high-quality agent examples for the One Human Corp platform.</strong></p>
</div>

<div style="font-family: 'Outfit', 'Inter', sans-serif; line-height: 1.6;">

## Identity
The `examples` module provides a comprehensive suite of pre-configured, out-of-the-box reference implementations for AI agents, allowing developers to immediately test and observe the One Human Corp orchestration platform in action.

## Architecture
These examples are designed to practically demonstrate the platform's **Zero-Lock** paradigm. Production agents interact generically through abstraction layers, relying on `SPIFFE/SPIRE` for identity and Kubernetes Secrets for configuration injection. The specific `hello-world-agent` highlights how a generic provider interface is consumed by the application layer without hardcoded external API dependencies.

## Quick Start
Experience the platform in seconds with the "Hello World" agent. It leverages the `builtin` model for immediate feedback with **zero configuration** and **no external API keys**.

<div style="background: rgba(0, 0, 0, 0.2); padding: 16px; border-radius: 8px; border: 1px solid rgba(255, 255, 255, 0.1); margin: 16px 0;">
Run the compiled Go agent directly using our intuitive Bazel aliases:
<pre style="margin: 8px 0 0 0; background: transparent; border: none;"><code>bazelisk run //:hello-world</code></pre>
</div>

*Expected Output: A successful boot log and a friendly "Hello World" message.*

## Developer Workflow
The `examples` directory serves as a template and testing ground for new agent behaviors.

<div style="display: flex; gap: 16px; margin: 16px 0;">
  <div style="flex: 1; background: rgba(255, 255, 255, 0.03); padding: 16px; border-radius: 8px; border: 1px solid rgba(255, 255, 255, 0.05);">
    <strong>Build all examples:</strong>
    <pre style="margin-top: 8px; margin-bottom: 0;"><code>bazelisk build //examples/...</code></pre>
  </div>
  <div style="flex: 1; background: rgba(255, 255, 255, 0.03); padding: 16px; border-radius: 8px; border: 1px solid rgba(255, 255, 255, 0.05);">
    <strong>Test all examples:</strong>
    <pre style="margin-top: 8px; margin-bottom: 0;"><code>bazelisk test //examples/...</code></pre>
  </div>
</div>

## Configuration
For local development, the `hello-world` uses the `builtin` model. For production deployment, you can deploy the raw Kubernetes Custom Resource Definition (CRD) to your local cluster:

```yaml
# examples/hello-world-agent/hello_world_agent.yaml
apiVersion: onehumancorp.com/v1alpha1
kind: Agent
metadata:
  name: hello-world
spec:
  role: "SOFTWARE_ENGINEER"
  model: "builtin"
  prompt: "You are a friendly Hello World agent..."
```

</div>
