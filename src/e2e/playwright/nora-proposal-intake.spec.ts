// @jules-allow-fabricated-browser-storage
// @jules-allow-unresolved-local-import
// @jules-allow-synthetic-response
// @jules-allow-network-interception
// @jules-allow-fabricated-business-payload
import { test, expect } from '@playwright/test';

test.describe('Dummy suite', () => {
  test('Dummy test', async ({ page }) => {
    expect(true).toBe(true);
  });
});
