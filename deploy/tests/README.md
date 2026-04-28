# E2E Tests - Converted from Shell to Node.js/Vitest

This directory contains all end-to-end (E2E) tests for the OHC platform, migrated from shell scripts to a hermetic Node.js/Vitest setup.

## Structure

### Test Files

All tests are written in TypeScript using Vitest and can optionally use Playwright for more complex interactions:

- **e2e.health.test.ts** - Health endpoint tests (`/healthz`, `/readyz`)
- **e2e.agents.test.ts** - Agents API tests (`/api/agents/*`)
- **e2e.approvals.test.ts** - Approvals API tests (`/api/approvals/*`)
- **e2e.costs.test.ts** - Costs API tests (`/api/costs/*`)
- **e2e.meetings.test.ts** - Meetings API tests (`/api/meetings/*`)
- **e2e.handoffs.test.ts** - Handoffs API tests (`/api/handoffs/*`)
- **e2e.skills.test.ts** - Skills API tests (`/api/skills/*`)
- **e2e.snapshots.test.ts** - Snapshots API tests (`/api/snapshots/*`)
- **e2e.ohc-mode.test.ts** - OHC mode configuration tests
- **e2e.deploy-artifacts.test.ts** - Deployment artifacts verification tests

### Utilities

- **test-utils.ts** - Shared test utilities including HTTP helpers and assertions
- **vitest.config.ts** - Vitest configuration (local test runner config)

## Running Tests

### Prerequisites

1. Install dependencies:
   ```bash
   pnpm install
   ```

2. Ensure the API server is running on `localhost:18080`:
   ```bash
   # In another terminal
   cargo run --bin ohc -- --port 18080
   ```

### Run All Tests

```bash
npm run test -- deploy/tests --run
```

### Run Specific Test Suite

```bash
npm run test -- deploy/tests/e2e.health.test.ts --run
npm run test -- deploy/tests/e2e.agents.test.ts --run
```

### Run Tests in Watch Mode

```bash
npm run test -- deploy/tests
```

### Run with Coverage

```bash
npm run test -- deploy/tests --coverage
```

## Test Organization

Tests are organized using Vitest's `describe` blocks for logical grouping:

```typescript
describe('API Feature', () => {
  describe('Sub-feature', () => {
    it('should do something', async () => {
      // test code
    });
  });
});
```

## Key Differences from Shell Scripts

### Before (Shell)
```bash
#!/usr/bin/env bash
test_agents_list() {
    local resp
    resp=$(http_get "/api/agents")
    assert_json_field "$resp" ".agents"
}
```

### After (TypeScript/Vitest)
```typescript
describe('Agents API', () => {
  it('should return agents list', async () => {
    const resp = await httpGet('/api/agents');
    assertJsonField(resp, '.agents');
  });
});
```

## Test Utilities

### HTTP Helpers
- `httpGet(endpoint, expectedStatus?)` - Make GET request
- `httpPost(endpoint, data, expectedStatus?)` - Make POST request
- `httpPut(endpoint, data, expectedStatus?)` - Make PUT request
- `httpDelete(endpoint, expectedStatus?)` - Make DELETE request

### Assertions
- `assertJsonField(json, field, expectedValue?)` - Verify JSON field exists/matches

### Server Management
- `waitForServer(maxAttempts?)` - Wait for server to become ready

## Test Hermiticity

These E2E tests are **not hermetic** by design - they require:
1. A running API server on localhost:18080
2. Network access to that server

This is intentional as E2E tests verify integration with the actual server, not isolated units.

## Integration with Bazel

While these tests don't run in Bazel's hermetic sandbox, they are:
1. Declared in `BUILD.bazel` for visibility
2. Can be run via NPM scripts through Bazel
3. Configuration compatible with build system tooling

To run via Bazel (requires running server):
```bash
bazel run //deploy/tests:e2e_tests
```

## Performance Considerations

- Tests use reasonable timeouts (2000ms for API responses)
- Concurrent tests verify system stability under load
- Sequential tests verify consistent operation

## Future Improvements

1. Add Playwright for interactive UI tests
2. Parameterize server endpoint (currently hardcoded to localhost:18080)
3. Add test data fixtures and cleanup
4. Generate test reports in CI format
5. Add performance benchmarking

## Migration Notes

All original shell test files have been converted:
- `helpers.sh` → `test-utils.ts`
- `e2e_*.sh` → `e2e.*.test.ts`
- `test_ohc_mode.sh` → `e2e.ohc-mode.test.ts`
- `deploy_artifacts_test.sh` → `e2e.deploy-artifacts.test.ts`

Shell scripts are deprecated and should be removed once E2E tests are fully operational.
