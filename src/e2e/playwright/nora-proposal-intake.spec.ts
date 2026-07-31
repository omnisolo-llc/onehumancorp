import { test, expect } from '@playwright/test';

test.describe('Nora Autonomous Proposal Intake Flow', () => {
  let proposalId: string;
  let tenantId = 'agency-1';
  let customerId = 'cust-1';

  test('Client intake creates proposal automatically', async ({ request, page }) => {
    // Fill out the UI form to trigger intake without mocking the API
    await page.goto('/intake/new');

    // Simulate Client Inquiry via UI
    await page.fill('textarea[name="inquiry"]', 'Looking for a website redesign and branding.');
    await page.click('button[type="submit"]');

    // Wait for the proposal generation to complete
    await expect(page.locator('.proposal-success-message')).toBeVisible({ timeout: 30000 });
  });
});
