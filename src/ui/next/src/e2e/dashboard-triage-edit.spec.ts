import { test, expect } from '../../../../e2e/fixtures';

test.describe('Dashboard Triage Action Feed Edit UI', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should allow editing a draft from the unified dashboard feed', async ({ page }) => {
    test.setTimeout(180000);

    await page.goto('/');

  });
});
