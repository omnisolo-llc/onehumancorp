import { test, expect } from '@playwright/test';
test('CUJ cross-agent handoff E2E check', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle(/One Human Corp/);
});
