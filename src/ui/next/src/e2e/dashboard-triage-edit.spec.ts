import { test, expect } from '@playwright/test';

test.describe('Dashboard Triage Edit', () => {
  test('Dashboard Triage Edit test', async ({ page }) => {
    await page.goto(`/triage`);
  });
});
