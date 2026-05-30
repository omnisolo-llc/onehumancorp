import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    // 0. Start from UI Login


    // 1. Acquisition & Onboarding start
    await page.goto('/onboarding');
    await page.waitForTimeout(1000);

    // Wait for the Smart Builder welcome screen
    await expect(page.locator('h2:has-text("Tell us about your business")')).toBeVisible({ timeout: 15000 });

    // Fill in the description (incorporating Maya to trigger mock)
    await page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties. My business is called Maya's Custom Cakes and I am located in Portland, OR.").fill("I bake custom vegan cakes. My business is called Maya's Custom Cakes and I am located in Portland, OR.");

    // Click Generate
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    // Step 2 - Review
    await expect(page.getByRole('heading', { name: "Review Details", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Continue/i }).click();

    // Step 3 - Style
    await expect(page.getByRole('heading', { name: "Style & Team", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Launch Store/i }).click();

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

    // Wait for the Smart Builder welcome screen
    await expect(page.locator('h2:has-text("Tell us about your business")')).toBeVisible({ timeout: 15000 });

    // Fill in the description (incorporating Carlos to trigger mock)
    await page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties. My business is called Maya's Custom Cakes and I am located in Portland, OR.").fill("carlos does handyman services. My business is called Carlos Plumbing and I am located in Miami, FL.");

    // Click Generate
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    // Step 2 - Review
    await expect(page.getByRole('heading', { name: "Review Details", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Continue/i }).click();

    // Step 3 - Style
    await expect(page.getByRole('heading', { name: "Style & Team", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Launch Store/i }).click();

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

  test('Priya (Boutique Owner) onboarding flow', async ({ page }) => {
    // 1. Acquisition & Onboarding start
    await page.goto('/onboarding');
    await page.waitForTimeout(1000);

    // Wait for the Smart Builder welcome screen
    await expect(page.locator('h2:has-text("Tell us about your business")')).toBeVisible({ timeout: 15000 });

    // Fill in the description
    // Use 'priya' in the prompt to ensure the mock endpoint resolves
    await page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties. My business is called Maya's Custom Cakes and I am located in Portland, OR.").fill("priya sells dresses. My business is called Priya Boutique and I am located in New York, NY.");

    // Click Generate
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    // Step 2 - Review
    await expect(page.getByRole('heading', { name: "Review Details", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Continue/i }).click();

    // Step 3 - Style
    await expect(page.getByRole('heading', { name: "Style & Team", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Launch Store/i }).click();

    // Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.getByRole('heading', { name: "You're Live!", exact: false })).toBeVisible({ timeout: 15000 });

    // 4. Verify Dashboard redirect and action banner
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('Leo (Music Tutor) onboarding flow', async ({ page }) => {
    await page.goto('/onboarding');
    await page.waitForTimeout(1000);

    await expect(page.locator('h2:has-text("Tell us about your business")')).toBeVisible({ timeout: 15000 });
    // Use 'leo' to ensure mock handles it
    await page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties. My business is called Maya's Custom Cakes and I am located in Portland, OR.").fill("leo teaches guitar lessons. My business is called Leo Music and I am located in Austin, TX.");
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    await expect(page.getByRole('heading', { name: "Review Details", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Continue/i }).click();

    await expect(page.getByRole('heading', { name: "Style & Team", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Launch Store/i }).click();

    await expect(page.getByRole('heading', { name: "You're Live!", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('Fatima (Food Cart) onboarding flow', async ({ page }) => {
    await page.goto('/onboarding');
    await page.waitForTimeout(1000);

    await expect(page.locator('h2:has-text("Tell us about your business")')).toBeVisible({ timeout: 15000 });
    // Use 'fatima' to ensure mock handles it
    await page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties. My business is called Maya's Custom Cakes and I am located in Portland, OR.").fill("fatima makes halal food. My business is called Fatima Halal Food and I am located in Chicago, IL.");
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    await expect(page.getByRole('heading', { name: "Review Details", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Continue/i }).click();

    await expect(page.getByRole('heading', { name: "Style & Team", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Launch Store/i }).click();

    await expect(page.getByRole('heading', { name: "You're Live!", exact: false })).toBeVisible({ timeout: 15000 });
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });
});
