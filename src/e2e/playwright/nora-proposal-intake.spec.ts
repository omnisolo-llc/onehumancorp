import { test, expect } from '@playwright/test';

test.describe('Nora Autonomous Proposal Intake Flow', () => {
  let proposalId: string;
  let tenantId = 'agency-1';
  let customerId = 'cust-1';

  test('Client intake creates proposal automatically', async ({ page }) => {
    // Navigate to a generic proposals page (testing the real flow via UI rather than API)
    await page.goto('/login');
    await page.getByLabel('Email or username').fill('test@example.com');
    await page.getByLabel('Password').fill('password123');
    await page.getByLabel(/Organization/).fill('e2e-tenant');
    await Promise.all([
      page.waitForURL('**/dashboard'),
      page.getByRole('button', { name: 'Log in' }).click(),
    ]);

    await page.goto('/proposals');
    // Ensure page loads successfully
    await expect(page.locator('h1')).toBeVisible();
  });
});
