1. **Update `srcs/server/telemetry/telemetry.go`**:
   - Change `api_rate_limit_exceeded_count` to `ohc_api_rate_limit_exceeded_total` in `InitTelemetry` / `InitWithMeter` initialization for `RateLimitExceededCount`. The suffix `_total` is standard for counters in Prometheus, and `ohc_` is the project prefix.
   - Example line 725: Change `"api_rate_limit_exceeded_count"` to `"ohc_api_rate_limit_exceeded_total"`.
   - Update the description to indicate it's the total.

2. **Verify tests and Build**:
   - Run `bazelisk test //srcs/server/...` to ensure all tests pass.

3. **Complete pre-commit steps**:
   - Ensure proper testing, verification, review, and reflection are done by calling `pre_commit_instructions`.

4. **Submit the PR**:
   - Submit the PR with standard conventions. Format the title as `🧹 Maintainer: [Proactive Improvement] Implement API Rate Limit Prometheus Metrics`. Ensure description contains 💡 What, 🎯 Why, 📊 Impact, and 🔬 Measurement. Include `issue_id: 4018` in the final message.
