import { test, expect } from '@playwright/test';

test.describe('Pricing Page Loop', () => {
  test('Pricing page loads and displays tiers correctly', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 10000 });
    await expect(page.locator('h3', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' })).toBeVisible();
  });

  test('Pricing page displays My Plan usage statistics', async ({ page }) => {
      await page.goto('/pricing');
      await expect(page.locator('h2', { hasText: 'My Plan: Free' })).toBeVisible();
      await expect(page.locator('p', { hasText: 'AI Actions Used' })).toBeVisible();
      await expect(page.locator('p', { hasText: 'Storage Used' })).toBeVisible();
      await expect(page.locator('p', { hasText: 'Estimated Next Bill' })).toBeVisible();
      await expect(page.locator('button', { hasText: 'Manage Plan & Billing' })).toBeVisible();
    });

    test('Pricing page displays upgrade buttons', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('button', { hasText: 'Upgrade to Starter via Stripe' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade to Pro via Stripe' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade to Business via Stripe' })).toBeVisible();
  });

  test('Pricing page Back button navigates to dashboard', async ({ page }) => {
    await page.goto('/pricing');
    await page.locator('a', { hasText: 'Back to Dashboard' }).click();
    await expect(page).toHaveURL('/dashboard');
  });
});

  test('Pricing page displays soft limit reached message', async ({ page }) => {
    // Route mock to trigger the soft limit message
    await page.route('/api/billing/my-plan', async route => {
      await route.fulfill({
        json: {
          current_plan: 'Free',
          ai_actions_used: 150,
          ai_actions_limit: 100,
          storage_used_bytes: 2 * 1024 * 1024,
          storage_limit_bytes: 500 * 1024 * 1024,
          next_bill_estimated: 0,
          soft_limit_reached: true,
          user_message: "You've reached your Free tier limit of 100 AI actions. Upgrade to unlock more power!"
        }
      });
    });

    await page.goto('/pricing');
    await expect(page.locator('text="You\'ve reached your Free tier limit of 100 AI actions. Upgrade to unlock more power!"')).toBeVisible();
  });
});
