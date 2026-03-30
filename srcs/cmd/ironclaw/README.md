# Ironclaw Security Scanner

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.08); padding: 20px; border-radius: 12px; font-family: 'Outfit', 'Inter', sans-serif;">
  <h2 style="margin-top: 0; color: #fff;">Overview</h2>
  <p style="color: #ccc;">
    The Ironclaw component is a blazing-fast, Go-based static application security testing (SAST) and secrets scanning tool. It is designed to run natively within the Bazel build pipeline, enforcing the "Zero Secrets" mandate of One Human Corp.
  </p>
</div>

<br>

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.08); padding: 20px; border-radius: 12px; font-family: 'Outfit', 'Inter', sans-serif;">
  <h2 style="margin-top: 0; color: #fff;">Architecture Walkthrough</h2>
  <p style="color: #ccc;">
    Ironclaw scans raw source files to detect sensitive hardcoded credentials and unresolved insecure TODOs. It integrates via Bazel testing targets to block CI pipelines immediately upon violation.
  </p>

```mermaid
graph TD
    A[Bazel CI Pipeline] -->|Executes| B(Ironclaw Scanner)
    B -->|Scans| C{Source Files}
    C -->|Detected Secrets| D[Reject Build]
    C -->|Insecure TODOs| D
    C -->|Clean| E[Pass Build]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,rx:8px,ry:8px;
    class A,B,C,D,E premium;
```

</div>

<br>

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.08); padding: 20px; border-radius: 12px; font-family: 'Outfit', 'Inter', sans-serif;">
  <h2 style="margin-top: 0; color: #fff;">Usage Guidelines</h2>
  <p style="color: #ccc;">
    Ironclaw requires no manual execution. It is invoked automatically as a Bazel `go_test` or binary execution within the repository's <code>//...</code> test suites.
  </p>
</div>
