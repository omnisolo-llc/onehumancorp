import { test, expect } from '@playwright/test';

test.describe('Subscription Box Engine', () => {

  test('Persona: Business Owner launches subscription product and views active subscribers', async ({ page }) => {
    // 1. Owner Logs In
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('maya@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

<<<<<<< HEAD
    // 2. Owner navigates to the subscriptions module.
    await page.goto('/subscriptions');

    // Check for the subscription card visibility
    await expect(page.getByRole('heading', { name: 'Subscriptions' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Active Plans' })).toBeVisible();
    await expect(page.getByText('Vegan Cake')).toBeVisible();
    await expect(page.getByText('Subscribers (2)')).toBeVisible();
    await expect(page.getByText('Ship on 2024-06-05')).toBeVisible();
=======
    // 2. Owner navigates to dashboard and sees the newly injected Subscription module
    await page.goto('/dashboard');

    // Check for the subscription card visibility
    await expect(page.getByRole('heading', { name: /Subscription & Memberships/i })).toBeVisible();
    await expect(page.getByText(/Active Subscribers: 42/i)).toBeVisible();
    await expect(page.getByText(/Upcoming Fulfillment: 42 boxes on Jan 5th/i)).toBeVisible();
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))

    // 3. Owner clicks Print Labels button (we simulate the action and alert)
    const printButton = page.getByRole('button', { name: /Print Labels/i });

    page.on('dialog', async dialog => {
<<<<<<< HEAD
      expect(dialog.message()).toContain('Printing labels...');
=======
      expect(dialog.message()).toContain('Printing 42 labels...');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
      await dialog.accept();
    });

    await printButton.click();
  });
});
