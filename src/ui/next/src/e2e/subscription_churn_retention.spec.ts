import { expect, test } from '@playwright/test';

test.describe('Agentic Subscription Retention & Churn Prediction Feed E2E', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display subscription churn win-back recommendation in the feed and allow approval', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in as Leo
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('leo@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Navigate to the unified agent feed
    await page.goto('/feed');

    // Wait for the feed items to populate
    await expect(page.getByTestId('agent-feed').first()).toBeVisible({ timeout: 25000 });

    // Assert that we see a churn risk Action Card
    const churnCard = page.locator('div', { hasText: 'at risk of churning' }).first();
    await expect(churnCard).toBeVisible({ timeout: 15000 });

    const approveBtn = churnCard.locator('button', { hasText: 'Approve' }).first();
    await expect(approveBtn).toBeVisible({ timeout: 15000 });

    await approveBtn.click();

    // Assert that the card is removed after approval
    await expect(approveBtn).not.toBeVisible({ timeout: 15000 });
  });
});
