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

  test('Pricing page toggles between monthly and annual pricing', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible();

    const proPrice = page.locator('.ohc-growth-card:has-text("Pro")');
    const businessPrice = page.locator('.ohc-growth-card:has-text("Business")');

    await expect(proPrice).toContainText('$79');
    await expect(proPrice).toContainText('/ month');
    await expect(businessPrice).toContainText('$299');
    await expect(businessPrice).toContainText('/ month');

    const toggle = page.locator('label:has(input#billing-toggle)');
    await expect(toggle).toBeVisible();
    await toggle.click();

    await expect(proPrice).toContainText('$63');
    await expect(proPrice).toContainText('/month, billed annually');
    await expect(businessPrice).toContainText('$239');
    await expect(businessPrice).toContainText('/month, billed annually');

    await toggle.click();

    await expect(proPrice).toContainText('$79');
    await expect(proPrice).toContainText('/ month');
    await expect(businessPrice).toContainText('$299');
    await expect(businessPrice).toContainText('/ month');
  });

});
