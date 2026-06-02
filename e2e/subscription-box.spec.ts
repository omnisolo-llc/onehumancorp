import { test, expect } from '@playwright/test';

test.describe('Subscription Box Engine', () => {

  test('Persona: Business Owner launches subscription product and views active subscribers', async ({ page }) => {
    // 1. Owner Logs In
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('maya@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // 2. Owner navigates to dashboard and sees the newly injected Subscription module
    await page.goto('/dashboard');

    // Check for the subscription card visibility
    await expect(page.getByRole('heading', { name: /Subscription & Memberships/i })).toBeVisible();
    await expect(page.getByText(/Active Subscribers: 42/i)).toBeVisible();
    await expect(page.getByText(/Upcoming Fulfillment: 42 boxes on Jan 5th/i)).toBeVisible();

    // 3. Owner clicks Print Labels button (we simulate the action and alert)
    const printButton = page.getByRole('button', { name: /Print Labels/i });

    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Printing 42 labels...');
      await dialog.accept();
    });

    await printButton.click();
  });
});
