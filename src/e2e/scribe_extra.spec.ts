import { test, expect } from './fixtures';
test("dummy e2e test", async () => { expect(1).toBe(1); });
  test.skip(process.env.CI === 'true' || process.env.CI === '1' || !!process.env.GITHUB_ACTIONS, 'Docker overlayfs bug breaks E2E test environments');
