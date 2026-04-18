1. **Cloud HPA/VPA Optimization**:
   - Improve `deploy/helm/ohc/values.yaml` scaling values for `backend`, `ohcCore`, `powersync`, and `chatwoot` to be more reactive (CPU/Mem targets updated).
2. **Standalone Desktop wrapper Optimization**:
   - Improve `srcs/server/standalone_ohc.sh` local process footprint by adjusting `GOGC` and constraining `GOMAXPROCS`.
3. **Multi-tenant PII guardrails initialization**:
   - Fix the PII redacting logger in `srcs/server/main.go` so it properly wraps the global handler within the `init()` function instead of overwriting `slog.SetDefault` in `main()`, ensuring early initialization compatibility.
4. **Pre-commit Checks**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
5. **Submit**:
   - Submit the PR with the title 🧹 Maintainer: [Infra] Optimise Orchestration & Observability.
