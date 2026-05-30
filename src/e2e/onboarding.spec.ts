import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    // 0. Start from UI Login


    // 1. Acquisition & Onboarding start
    await page.goto('/onboarding');
    await page.waitForTimeout(1000);

    // Wait for the Smart Builder welcome screen (Step 1 - Chat 1)
    await expect(page.getByRole('heading', { name: "What's the name of your business?", exact: false })).toBeVisible({ timeout: 15000 });

    // Fill in the description (incorporating Maya to trigger mock)
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Maya's Custom Cakes");
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Step 1 - Chat 2
    await expect(page.getByRole('heading', { name: 'What do you sell?', exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByPlaceholder('e.g. I bake custom vegan cakes for weddings and parties...').fill('maya bakes custom cakes');
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Step 1 - Chat 3
    await expect(page.getByRole('heading', { name: 'Where are you located?', exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByPlaceholder('e.g. Portland, OR').fill('Portland, OR');

    // Click Generate
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    // Step 2 - Review
    await expect(page.getByRole('heading', { name: "Review Details", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Continue/i }).click();

    // Step 3 - Style
    await expect(page.getByRole('heading', { name: "Style & Team", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Approve & Launch/i }).click();

    // Simplified Mobile First Onboarding - wait for it to generate
    // Step 2 is automatic, so wait for Step 3 directly
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

    // Wait for the Smart Builder welcome screen (Step 1 - Chat 1)
    await expect(page.getByRole('heading', { name: "What's the name of your business?", exact: false })).toBeVisible({ timeout: 15000 });

    // Fill in the description (incorporating Carlos to trigger mock)
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Carlos Plumbing");
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Step 1 - Chat 2
    await expect(page.getByRole('heading', { name: 'What do you sell?', exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByPlaceholder('e.g. I bake custom vegan cakes for weddings and parties...').fill('carlos does handyman services');
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Step 1 - Chat 3
    await expect(page.getByRole('heading', { name: 'Where are you located?', exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByPlaceholder('e.g. Portland, OR').fill('Miami, FL');

    // Click Generate
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    // Step 2 - Review
    await expect(page.getByRole('heading', { name: "Review Details", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Continue/i }).click();

    // Step 3 - Style
    await expect(page.getByRole('heading', { name: "Style & Team", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Approve & Launch/i }).click();

    // Simplified Mobile First Onboarding - wait for it to generate
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
