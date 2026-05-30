import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    // 0. Start from UI Login

    // 1. Acquisition & Onboarding start
    await page.goto('/onboarding');
    await page.waitForTimeout(1000);

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.getByRole('heading', { name: "Welcome to OneHumanCorp", exact: false })).toBeVisible({ timeout: 15000 });

    // Fill in the description (incorporating Maya to trigger mock)
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Maya's Cakes");
    await page.getByPlaceholder('e.g. Custom Cakes').fill('maya bakes custom cakes');
    await page.getByPlaceholder('e.g. Elegant, Playful').fill('Playful');

    // Click Generate
    await page.getByRole('button', { name: /Create My Business/i }).click();

    // Step 2 - Magic Loading is quick, we should end up on Step 3
    await expect(page.getByRole('heading', { name: "Let's add your first item.", exact: false })).toBeVisible({ timeout: 15000 });

    // Verify AI generated values
    await expect(page.locator('input[value="Custom Vegan Cake"]')).toBeVisible();
    await expect(page.locator('input[value="45.00"]')).toBeVisible();

    // Step 3 - Go Live
    await page.getByRole('button', { name: /Looks Good! Go Live./i }).click();

    // Step 4/5 - You're Live!
    await expect(page.getByRole('heading', { name: "You're Live!", exact: false })).toBeVisible({ timeout: 15000 });

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

    // 1. Acquisition & Onboarding start
    await page.goto('/onboarding');
    await page.waitForTimeout(1000);

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.getByRole('heading', { name: "Welcome to OneHumanCorp", exact: false })).toBeVisible({ timeout: 15000 });

    // Fill in the description (incorporating Carlos to trigger mock)
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Carlos Plumbing");
    await page.getByPlaceholder('e.g. Custom Cakes').fill('carlos does handyman services');
    await page.getByPlaceholder('e.g. Elegant, Playful').fill('Professional');

    // Click Generate
    await page.getByRole('button', { name: /Create My Business/i }).click();

    // Step 2 - Magic Loading is quick, we should end up on Step 3
    await expect(page.getByRole('heading', { name: "Let's add your first item.", exact: false })).toBeVisible({ timeout: 15000 });

    // Verify AI generated values
    await expect(page.locator('input[value="Pipe Fix"]')).toBeVisible();
    await expect(page.locator('input[value="80.00"]')).toBeVisible();

    // Step 3 - Go Live
    await page.getByRole('button', { name: /Looks Good! Go Live./i }).click();

    // Step 4/5 - You're Live!
    await expect(page.getByRole('heading', { name: "You're Live!", exact: false })).toBeVisible({ timeout: 15000 });

    // 4. Verify Dashboard redirect and action banner
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // Handle either case since the mock data might change
    const stripeBanner = page.locator('text=1 Action Required: Connect Stripe to accept payments.');
    const setupBanner = page.getByRole('button', { name: 'Complete Stripe Setup' });

    await expect(stripeBanner.or(setupBanner).first()).toBeVisible({ timeout: 15000 });
  });
});
