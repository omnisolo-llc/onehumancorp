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

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.getByRole('heading', { name: "Welcome to OHC" })).toBeVisible();
    await page.getByRole('button', { name: 'Start a Business', exact: true }).click();

    // Step 2 - Business Name
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Maya's Custom Cakes");
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Step 3 - Business Type
    await expect(page.getByRole('heading', { name: 'Business Type' })).toBeVisible();
    await page.getByText('Food').click();
    await page.getByRole('button', { name: /Launch Store/i }).click();

    // Step 4 - Wait for it to generate and redirect
    await expect(page.getByRole('heading', { name: "AI is building your storefront..." })).toBeVisible({ timeout: 15000 });

    // 5. Verify Dashboard redirect and action banner
    await expect(page).toHaveURL(/.*\/dashboard/);
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // Verify Preview Site button exists
    await expect(page.getByRole('link', { name: 'Preview Site' })).toBeVisible({ timeout: 15000 });

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

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.getByRole('heading', { name: "Welcome to OHC" })).toBeVisible();
    await page.getByRole('button', { name: 'Start a Business', exact: true }).click();

    // Step 2 - Business Name
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Carlos Plumbing");
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Step 3 - Business Type
    await expect(page.getByRole('heading', { name: 'Business Type' })).toBeVisible();
    await page.getByText('Service').click();
    await page.getByRole('button', { name: /Launch Store/i }).click();

    // Step 4 - Wait for it to generate and redirect
    await expect(page.getByRole('heading', { name: "AI is building your storefront..." })).toBeVisible({ timeout: 15000 });

    // 5. Verify Dashboard redirect and action banner
    await expect(page).toHaveURL(/.*\/dashboard/);
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // Verify Preview Site button exists
    await expect(page.getByRole('link', { name: 'Preview Site' })).toBeVisible({ timeout: 15000 });

    // Handle either case since the mock data might change
    const stripeBanner = page.locator('text=1 Action Required: Connect Stripe to accept payments.');
    const setupBanner = page.getByRole('button', { name: 'Complete Stripe Setup' });

    await expect(stripeBanner.or(setupBanner).first()).toBeVisible({ timeout: 15000 });
  });

  test('Priya (Boutique) onboarding flow', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('priya@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: "Welcome to OHC" })).toBeVisible();
    await page.getByRole('button', { name: 'Start a Business', exact: true }).click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Priya's Boutique");
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    await expect(page.getByRole('heading', { name: 'Business Type' })).toBeVisible();
    await page.getByText('Physical').click();
    await page.getByRole('button', { name: /Launch Store/i }).click();

    await expect(page.getByRole('heading', { name: "AI is building your storefront..." })).toBeVisible({ timeout: 15000 });
    await expect(page).toHaveURL(/.*\/dashboard/);
    await expect(page.getByRole('link', { name: 'Preview Site' })).toBeVisible({ timeout: 15000 });
  });

  test('Leo (Music Tutor) onboarding flow', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('leo@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: "Welcome to OHC" })).toBeVisible();
    await page.getByRole('button', { name: 'Start a Business', exact: true }).click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Leo's Guitar Lessons");
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    await expect(page.getByRole('heading', { name: 'Business Type' })).toBeVisible();
    await page.getByText('Service').click();
    await page.getByRole('button', { name: /Launch Store/i }).click();

    await expect(page.getByRole('heading', { name: "AI is building your storefront..." })).toBeVisible({ timeout: 15000 });
    await expect(page).toHaveURL(/.*\/dashboard/);
    await expect(page.getByRole('link', { name: 'Preview Site' })).toBeVisible({ timeout: 15000 });
  });

  test('Fatima (Food Cart) onboarding flow', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('fatima@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: "Welcome to OHC" })).toBeVisible();
    await page.getByRole('button', { name: 'Start a Business', exact: true }).click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill("Fatima's Halal Cart");
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    await expect(page.getByRole('heading', { name: 'Business Type' })).toBeVisible();
    await page.getByText('Food').click();
    await page.getByRole('button', { name: /Launch Store/i }).click();

    await expect(page.getByRole('heading', { name: "AI is building your storefront..." })).toBeVisible({ timeout: 15000 });
    await expect(page).toHaveURL(/.*\/dashboard/);
    await expect(page.getByRole('link', { name: 'Preview Site' })).toBeVisible({ timeout: 15000 });
  });
});
