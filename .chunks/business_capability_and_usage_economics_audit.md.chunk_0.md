# OneHumanCorp: capability, owner-needs and usage-economics audit

**Review date:** 2026-09-18 (America/Los_Angeles). **Revision:** 2026-09-18-usage-audit.

**Purpose:** establish what the product actually contains, what owners describe needing, how current AI business products overlap, and what metered compute/API or customer-funded inference would require. This is an evidence review, not a new implementation roadmap or a published price list.

The previous $99 subscription, 300-step allowance, $299 setup, fixed pilot conversion/margin thresholds and exclusive web/design/marketing segment are **suspended hypotheses**, not accepted requirements. Preserve useful earlier findings and OHC target IDs, but do not dispatch work from those assumptions. The user now asks us to evaluate compute/API charging and BYOK, including provider-permitted subscription access, before committing to a plan.

## 1. Scope and evidence discipline

Source baseline: `f8e9d8dd5c099f417df0c32f6131e9b465e5fb20`, branch `fix/bazel-modernization-and-cleanup`. Existing documentation changes were preserved. This review inspected selected critical implementation paths, route registration, billing, provider access, business workflows, UI packaging and test definitions. It did not inspect every source file or exercise the full running product. Line numbers below refer to that source baseline.

Use these distinct labels: **source present**, **mounted route/caller traced**, **unit/integration tested**, **provider-sandbox verified**, **real owner verified**. A class name, screenshot, generated link or smoke-test title cannot establish the last three. No new interviews, owner identity verification, payment commitments, provider charges or production measurements were obtained.

The attempted existing check was:

```sh
bazel test //src/server/pricing:server_pricing_unit_test --test_filter=budget --test_output=errors --jobs=4
```

It **timed out after 180 seconds during Bazel analysis**, before test results