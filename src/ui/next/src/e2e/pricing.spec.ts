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

  test('Pricing page displays upgrade buttons', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('button', { hasText: 'Upgrade to Starter via Stripe' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade to Pro via Stripe' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade to Business via Stripe' })).toBeVisible();
  });

  test('Pricing page Back button navigates to dashboard', async ({ page }) => {
    await page.goto('/pricing');
    await page.locator('button', { hasText: 'Back' }).click();
    await expect(page).toHaveURL('/dashboard');
  });
});
