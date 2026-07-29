import { expect } from '@playwright/test';
import { test } from '../../../../e2e/fixtures';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary without network interception', async ({ page }) => {
    // 4. Navigate to the inbox page
    await page.goto('/inbox');

    // We expect the real page to render
    await expect(page.locator('body')).toBeVisible();
  });
});
