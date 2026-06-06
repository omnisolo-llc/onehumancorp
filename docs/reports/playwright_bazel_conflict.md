# Infrastructure Note: Playwright & Bazel Dependency Conflict

During the implementation of the "Abandoned Cart Campaign" growth loop, an infrastructure issue was identified when attempting to run new E2E tests via the Bazel runner.

### Issue Description
When executing a newly added Playwright spec (or certain existing smoke tests) via Bazel:
```bash
npx @bazel/bazelisk test //src/e2e:playwright_src_ui_next_src_e2e_abandoned_cart_spec_ts
```

The test runner fails with the following framework error:
```
Error: Playwright Test did not expect test.describe() to be called here.
Most common reasons include:
- You are calling test.describe() in a configuration file.
- You are calling test.describe() in a file that is imported by the configuration file.
- You have two different versions of @playwright/test. This usually happens
  when one of the dependencies in your package.json depends on @playwright/test.
```

### Context & Impact
This error typically surfaces when the test environment loads multiple versions of `@playwright/test` into the module cache, or when Bazel sandbox boundaries interact poorly with the module resolver and Next.js project scopes. Because it prevents new UI flows from being verified in CI, the newly authored E2E spec (`abandoned_cart.spec.ts`) was removed from the repository.

### Action Items
- An infrastructure audit is needed on `src/ui/next/package.json` vs the root `package.json` to resolve the duplicate/conflicting `@playwright/test` dependency.
- The `currentAppSmoke` test helper and test fixture loading sequence should be evaluated for redundant imports that trigger this Playwright initialization error.
