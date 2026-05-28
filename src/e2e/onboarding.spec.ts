import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    // 0. Start from UI Login
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('maya@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    // Wait for Dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // 1. Acquisition & Onboarding start
    await page.goto('/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1 - Chat 1)
    // Wait for the Smart Builder welcome screen
    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();

    // Fill in the description (incorporating Maya to trigger mock)
    await page.getByPlaceholder("e.g. I am Maya. I bake vegan cakes in Austin. Prices start at $50.").fill("I am Maya. I bake custom vegan cakes for weddings and parties in Portland, OR.");

    // Click Generate
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    // Wait for the loading screen
    await expect(page.getByRole('heading', { name: "Our Marketing Department is building your store..." })).toBeVisible();

    // Wait for it to generate
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    // Verify shareable link is present
    await expect(page.getByText('my-business.ohc.store')).toBeVisible();

    // 4. Verify Dashboard redirect and action banner
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // Handle either case since the mock data might change
    const stripeBanner = page.locator('text=1 Action Required: Connect Stripe to accept payments.');
    const setupBanner = page.getByRole('button', { name: 'Complete Stripe Setup' });

    await expect(stripeBanner.or(setupBanner).first()).toBeVisible({ timeout: 15000 });
  });

  test('Carlos (Handyman) onboarding flow', async ({ page }) => {
    // 0. Start from UI Login
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('carlos@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    // Wait for Dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // 1. Acquisition & Onboarding start
    await page.goto('/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1 - Chat 1)
    // Wait for the Smart Builder welcome screen
    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();

    // Fill in the description (incorporating Carlos to trigger mock)
    await page.getByPlaceholder("e.g. I am Maya. I bake vegan cakes in Austin. Prices start at $50.").fill("I am Carlos. I offer plumbing and handyman services in Miami, FL.");

    // Click Generate
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    // Wait for the loading screen
    await expect(page.getByRole('heading', { name: "Our Marketing Department is building your store..." })).toBeVisible();

    // Wait for it to generate
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    // 4. Verify Dashboard redirect and action banner
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // Handle either case since the mock data might change
    const stripeBanner = page.locator('text=1 Action Required: Connect Stripe to accept payments.');
    const setupBanner = page.getByRole('button', { name: 'Complete Stripe Setup' });

    await expect(stripeBanner.or(setupBanner).first()).toBeVisible({ timeout: 15000 });
  });
});
