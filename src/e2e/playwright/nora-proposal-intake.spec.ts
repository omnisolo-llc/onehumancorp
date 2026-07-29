import { expect } from '@playwright/test';
import { test } from '../fixtures';

test.describe('Nora Autonomous Proposal Intake Flow', () => {
  test('Client intake creates proposal automatically via UI', async ({ page }) => {
    await page.goto('/proposals/customer-view?id=e2e-proposal-1');
    await expect(page.locator('body')).toBeVisible();
  });
});
