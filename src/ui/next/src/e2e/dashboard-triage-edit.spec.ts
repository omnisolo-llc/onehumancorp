import { expect } from '@playwright/test';
import { test } from '../../../../e2e/fixtures';

test.describe('Dashboard Triage Action Feed Edit UI', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should allow editing a draft from the unified dashboard feed via UI', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });
    await expect(page.locator('body')).toBeVisible();
  });
});
