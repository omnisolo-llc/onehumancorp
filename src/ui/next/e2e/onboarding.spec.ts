import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Clear local storage by navigating to a blank page first
    await page.goto('http://localhost:3000');
    await page.evaluate(() => localStorage.clear());
  });

  test('1. Test successful storefront creation with all fields filled correctly', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('http://localhost:3000/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.locator('text="Launch your creator business"')).toBeVisible();

    // Fill in the description
    const descriptionInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes in Portland, OR..."]');
    await descriptionInput.fill('I am a freelance handyman in Miami');

    // Fill Instagram
    const instaInput = page.locator('input[placeholder="your.handle"]');
    await instaInput.fill('miamihandyman');

    // Click Stripe toggle
    await page.locator('text="Connect Stripe for deposits"').click();

    // Intercept API calls
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      json: { initial_products: [{ name: 'Custom Repair', price: '100.00' }] }
    }));

    await page.route('**/api/onboarding/start', route => route.fulfill({
      status: 200,
      json: { message: "Your business has been successfully launched." }
    }));

    // Click Generate
    await page.locator('button:has-text("Generate Storefront")').click();

    // Loading screen
    await expect(page.locator('text="Building Your Business..."')).toBeVisible({ timeout: 5000 });

    // Live Screen (Should skip steps 2 and 3)
    await expect(page.locator('text="You\'re Live!"')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text="Your business has been successfully launched."')).toBeVisible();
    await expect(page.locator('text="my-business.ohc.store"')).toBeVisible();
    await expect(page.locator('text=/Marketing Agent is now optimizing/')).toBeVisible();
  });

  test('2. Test empty fields validation', async ({ page }) => {
    await page.goto('http://localhost:3000/onboarding');
    await expect(page.locator('text="Launch your creator business"')).toBeVisible();

    // Description is empty, button should be disabled
    const button = page.locator('button:has-text("Generate Storefront")');
    await expect(button).toBeDisabled();

    const descriptionInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes in Portland, OR..."]');
    await descriptionInput.fill('x');
    await expect(button).toBeEnabled();

    await descriptionInput.fill('');
    await expect(button).toBeDisabled();
  });

  test('3. Test API failure handling for the intake call', async ({ page }) => {
    await page.goto('http://localhost:3000/onboarding');
    await expect(page.locator('text="Launch your creator business"')).toBeVisible();

    const descriptionInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes in Portland, OR..."]');
    await descriptionInput.fill('Testing API Failure');

    // Intercept intake API call to fail
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 500
    }));

    await page.locator('button:has-text("Generate Storefront")').click();

    // Wait for error to appear and step to remain/go back to 1
    await expect(page.locator('text="Failed to process business details"')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text="Launch your creator business"')).toBeVisible();
  });

  test('4. Test API failure handling for the start call', async ({ page }) => {
    await page.goto('http://localhost:3000/onboarding');
    await expect(page.locator('text="Launch your creator business"')).toBeVisible();

    const descriptionInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes in Portland, OR..."]');
    await descriptionInput.fill('Testing Start Failure');

    // Intercept intake to succeed
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      json: { }
    }));

    // Intercept start to fail
    await page.route('**/api/onboarding/start', route => route.fulfill({
      status: 500
    }));

    await page.locator('button:has-text("Generate Storefront")').click();

    // Should see loading briefly
    await expect(page.locator('text="Building Your Business..."')).toBeVisible({ timeout: 5000 });

    // Wait for error to appear and step to go back to 1
    await expect(page.locator('text="Failed to start onboarding"')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text="Launch your creator business"')).toBeVisible();
  });

  test('5. Test navigating to the dashboard post-creation successfully', async ({ page }) => {
    // We mock success and navigate to dashboard, creating the DOM structure for it since Next backend won't have it natively here without mocks
    await page.route('**/api/dashboard*', route => route.fulfill({
      status: 200,
      json: { } // Dummy mock for dashboard APIs
    }));

    await page.goto('http://localhost:3000/onboarding');
    const descriptionInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes in Portland, OR..."]');
    await descriptionInput.fill('Dashboard test');

    await page.route('**/api/onboarding/intake', route => route.fulfill({ status: 200, json: {} }));
    await page.route('**/api/onboarding/start', route => route.fulfill({ status: 200, json: {} }));

    await page.locator('button:has-text("Generate Storefront")').click();

    await expect(page.locator('text="You\'re Live!"')).toBeVisible({ timeout: 10000 });

    const dashboardLink = page.locator('a:has-text("Go to Dashboard")');
    await expect(dashboardLink).toBeVisible();
    await expect(dashboardLink).toHaveAttribute('href', '/dashboard');

    await dashboardLink.click();
    await page.waitForURL('**/dashboard');

    // Assert we landed on dashboard
    await expect(page.locator('text="Morning Briefing"')).toBeVisible({ timeout: 5000 });
  });
});
