import { test, expect } from './fixtures';
test("dummy e2e test", async () => { expect(1).toBe(1); });
  test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
