import { test, expect } from '@playwright/test';

test.describe('Nora Autonomous Proposal Intake Flow', () => {
  test('Client intake creates proposal automatically', async ({ page }) => {
    await page.goto('/intake');
    await page.fill('textarea[name="inquiry"]', 'Looking for a website redesign and branding.');
    await page.click('button[type="submit"]');
  });
});
