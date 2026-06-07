import { test, expect } from '@playwright/test';

test.describe.configure({ timeout: 60000 });

test.describe('Zero-Click Onboarding CUJ', () => {
  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });

    // Mock the backend API responses for zero-click flow
    await page.route('/api/onboarding/intake', async route => {
      const json = {
        business_type: "Online Store",
        business_name: "Mocked Business",
        categories: ["physical"],
        initial_products: [{ name: "Test Product", price: "10.00" }],
        location: "Mock City",
        target_audience: "Everyone"
      };
      await route.fulfill({ json });
    });

    await page.route('/api/onboarding/start', async route => {
      const json = {
        success: true,
        message: "Your business has been successfully launched.",
        organization_id: "mock-tenant-id"
      };
      await route.fulfill({ json });
    });
  });

  test('Maya the Baker can complete the onboarding flow in one step', async ({ page }) => {
    await page.goto('/onboarding');
    await page.getByText('Start Onboarding').click();

    await page.getByPlaceholder(/e.g. I sell vegan cakes in Austin/i).fill('Maya Bakery selling custom vegan cakes for weddings and parties in Seattle, WA.');

    // Check loading state visually, but since we have mocked the API or use a fast local API, we'll wait for final success
    await page.getByRole('button', { name: 'Generate My Storefront' }).click();

    // Verify it proceeds to success screen
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 35000 });
    const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
    expect(storedTenantId).not.toBeNull();
  });

  test('Carlos the Handyman sets up his repair business', async ({ page }) => {
    await page.goto('/onboarding');
    await page.getByText('Start Onboarding').click();

    await page.getByPlaceholder(/e.g. I sell vegan cakes in Austin/i).fill('Carlos Fixes It doing plumbing and general repairs in Austin, TX.');

    await page.getByRole('button', { name: 'Generate My Storefront' }).click();

    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 35000 });
    const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
    expect(storedTenantId).not.toBeNull();
  });

  test('Leo the Music Tutor configures online bookings', async ({ page }) => {
    await page.goto('/onboarding');
    await page.getByText('Start Onboarding').click();

    await page.getByPlaceholder(/e.g. I sell vegan cakes in Austin/i).fill('Leo Guitar Lessons doing guitar tutoring online remotely.');

    await page.getByRole('button', { name: 'Generate My Storefront' }).click();

    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 35000 });
    const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
    expect(storedTenantId).not.toBeNull();
  });

  test('Fatima the Food Cart Operator on a slower network', async ({ page }) => {
    await page.goto('/onboarding');
    await page.getByText('Start Onboarding').click();

    await page.getByPlaceholder(/e.g. I sell vegan cakes in Austin/i).fill('Fatima Halal Food doing halal food cart pickup orders in New York, NY.');

    await page.getByRole('button', { name: 'Generate My Storefront' }).click();

    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 35000 });
    const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
    expect(storedTenantId).not.toBeNull();
  });

  test('Validation error prevents launching without description', async ({ page }) => {
    await page.goto('/onboarding');
    await page.getByText('Start Onboarding').click();

    // Expect button to be disabled without description
    await expect(page.getByRole('button', { name: 'Generate My Storefront' })).toBeDisabled();

    // Ensure it hasn't progressed to the success screen
    await expect(page.getByText("You're Live!")).toBeHidden();
  });
});
