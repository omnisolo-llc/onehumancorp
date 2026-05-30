import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    await page.goto('/website-builder');

    await expect(page.getByRole('heading', { name: 'Describe your business in a sentence' })).toBeVisible({ timeout: 15000 });
    await page.getByPlaceholder(/e\.g\. I run a local bakery called Maya's Cakes\.\.\./).fill("I bake custom vegan cakes in Seattle.");
    await page.getByRole('button', { name: /Launch your business in 10 minutes/ }).click();

    await expect(page.getByRole('heading', { name: 'Edit Website' })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Publish Changes/ }).click();

    await expect(page.getByText("You're set up! Here's what to do next:")).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Go to Dashboard/i }).click();

    // Handle either case since the mock data might change
    const stripeBanner = page.locator('text=1 Action Required: Connect Stripe to accept payments.');
    const setupBanner = page.getByRole('button', { name: 'Complete Stripe Setup' });

    await expect(stripeBanner.or(setupBanner).first()).toBeVisible({ timeout: 15000 });
  });

  test('Carlos (Handyman) onboarding flow', async ({ page }) => {
    await page.goto('/website-builder');

    await expect(page.getByRole('heading', { name: 'Describe your business in a sentence' })).toBeVisible({ timeout: 15000 });
    await page.getByPlaceholder(/e\.g\. I run a local bakery called Maya's Cakes\.\.\./).fill("I am a handyman named Carlos in Miami.");
    await page.getByRole('button', { name: /Launch your business in 10 minutes/ }).click();

    await expect(page.getByRole('heading', { name: 'Edit Website' })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Publish Changes/ }).click();

    await expect(page.getByText("You're set up! Here's what to do next:")).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Go to Dashboard/i }).click();

    // Handle either case since the mock data might change
    const stripeBanner = page.locator('text=1 Action Required: Connect Stripe to accept payments.');
    const setupBanner = page.getByRole('button', { name: 'Complete Stripe Setup' });

    await expect(stripeBanner.or(setupBanner).first()).toBeVisible({ timeout: 15000 });
  });
});
