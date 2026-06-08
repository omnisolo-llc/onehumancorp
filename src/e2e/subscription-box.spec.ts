import { test, expect } from '@playwright/test';

test.describe('Subscription Box Engine', () => {

  test('Persona: Business Owner launches subscription product and views active subscribers', async ({ page }) => {
    // 1. Owner Logs In
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('maya@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // 2. Owner navigates to the subscriptions module.
    await page.goto('/subscriptions');

    // Check for the subscription card visibility
    await expect(page.getByRole('heading', { name: 'Subscriptions' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Active Plans' })).toBeVisible();
    await expect(page.getByText('Vegan Cake')).toBeVisible();
    await expect(page.getByText('Subscribers (2)')).toBeVisible();
    await expect(page.getByText('Ship on 2024-06-05')).toBeVisible();

    // 3. Owner clicks Print Labels button (we simulate the action and alert)
    const printButton = page.getByRole('button', { name: /Print Labels/i });

    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Printing labels...');
      await dialog.accept();
    });

    await printButton.click();
  });
});
