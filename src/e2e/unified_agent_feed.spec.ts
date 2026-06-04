import { test, expect } from './fixtures';

test.describe('Unified Agent Feed', () => {
  test('should display agent feed and allow interaction', async ({ page }) => {
    // Login
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.locator('input[type="password"]').fill('admin');
    await page.locator('button:has-text("Login")').click();

    // Verify we are on dashboard and the Unified Agent Feed is present
    await expect(page.getByRole('heading', { name: 'Agent Proposals' })).toBeVisible();

    // We expect seeded approvals to show up because of our seed data updates
    await expect(page.getByText('Draft email for review')).toBeVisible();
    await expect(page.getByText('Abandoned cart recovery: 10% discount for Sarah')).toBeVisible();

    // Click to approve the email draft
    const approveBtn = page.locator('button[aria-label="Approve proposal"]').first();
    await approveBtn.click();

    // Verify it was optimistically removed from the UI
    await expect(page.getByText('Draft email for review')).not.toBeVisible();
  });
});
