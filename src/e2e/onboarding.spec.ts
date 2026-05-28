import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    // Intercept intake and start API calls
    await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Maya's Cakes", business_type: "Bakery", categories: ["food", "physical"], initial_products: [{ name: "Custom Vegan Cake", price: "45.00" }] }) }));
    await page.route('**/api/onboarding/start', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ message: "Your business has been successfully launched.", organization_id: "org_123" }) }));

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
    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();

    // Fill in the description
    await page.getByPlaceholder('e.g. I bake custom vegan cakes in Portland, OR...').fill('I bake custom vegan cakes in Portland, OR');

    // Click Generate
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    // 2. Simplified Mobile First Onboarding - wait for it to generate (Step 2 & 3)
    // Step 2 is automatic, so wait for Step 3 directly
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    // Verify shareable link is present
    await expect(page.getByText('my-business.ohc.store')).toBeVisible();

    // 4. Verify Dashboard redirect and action banner
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // Handle either case since the mock data might change
    const stripeBanner = page.locator('text=1 Action Required: Connect Stripe to accept payments.');
    const setupBanner = page.locator('text=Complete Stripe Setup');

    await expect(stripeBanner.or(setupBanner)).toBeVisible({ timeout: 15000 });
  });

  test('Carlos (Handyman) onboarding flow', async ({ page }) => {
    // Intercept intake and start API calls
    await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Carlos Plumbing", business_type: "Service", categories: ["service"], initial_products: [{ name: "Pipe Fix", price: "80.00" }] }) }));
    await page.route('**/api/onboarding/start', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ message: "Your business has been successfully launched.", organization_id: "org_456" }) }));

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
    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();

    // Fill in the description
    await page.getByPlaceholder('e.g. I bake custom vegan cakes in Portland, OR...').fill('I am a freelance handyman in Miami');

    // Click Generate
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    // 2. Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    // 4. Verify Dashboard redirect and action banner
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // Handle either case since the mock data might change
    const stripeBanner = page.locator('text=1 Action Required: Connect Stripe to accept payments.');
    const setupBanner = page.locator('text=Complete Stripe Setup');

    await expect(stripeBanner.or(setupBanner)).toBeVisible({ timeout: 15000 });
  });
});
