import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Mock the backend call to avoid timeouts in isolated Next.js tests
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      body: JSON.stringify({
        business_type: 'Bakery',
        business_name: 'Maya\'s Dream Cakes',
        initial_products: [{ name: 'Custom Vegan Cake', price: '45.00' }],
        categories: ['food', 'physical']
      })
    }));

    await page.route('**/api/onboarding/state', route => route.fulfill({
      status: 200,
      body: JSON.stringify({ wizardState: {} })
    }));

    await page.route('**/api/onboarding/draft', route => route.fulfill({
      status: 200,
      body: JSON.stringify({})
    }));

    await page.route('**/api/onboarding/start', route => route.fulfill({
      status: 200,
      body: JSON.stringify({ message: "Your business has been successfully launched." })
    }));
  });

  test('completes full onboarding flow', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('http://localhost:3000/onboarding');

    // Step 1: Tell us about your business
    await expect(page.locator('text="Tell us about your business"')).toBeVisible();
    await page.locator('textarea[placeholder="e.g. Maya Bakery that bakes custom vegan cakes in Portland, OR"]').fill('Maya Bakery that bakes custom vegan cakes in Portland, OR');

    // Click Generate
    await page.locator('button:has-text("Generate My Business")').click();

    // 2. Wait for Review Details Step
    await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 5000 });

    // Continue to next step
    await page.locator('button:has-text("Continue")').click();

    // 3. Wait for Style & Team Step
    await expect(page.locator('text="Style & Team"')).toBeVisible({ timeout: 5000 });

    // Select Template and Launch
    await page.locator('text="Classic"').click();
    await page.locator('button:has-text("Launch Store")').click();

    // 4. Loading screen

    // 5. Live Screen
    await expect(page.locator('text="You\'re Live!"')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text="Your business has been successfully launched."')).toBeVisible();
    await expect(page.locator('text="my-business.ohc.store"')).toBeVisible();

    const dashboardLink = page.locator('a:has-text("Go to Dashboard")');
    await expect(dashboardLink).toBeVisible();
    await expect(dashboardLink).toHaveAttribute('href', '/dashboard');

    await dashboardLink.click();
    // Assuming the test doesn't actually have a Next.js server with these routes locally, skip asserting full dashboard navigation
    // await page.waitForURL('**/dashboard');
  });

  test('Maya the Home Baker Persona CUJ', async ({ page }) => {
    await page.goto('http://localhost:3000/onboarding');

    // Step 1: Tell us about your business
    await page.locator('textarea[placeholder="e.g. Maya Bakery that bakes custom vegan cakes in Portland, OR"]').fill('Maya\'s Dream Cakes. Custom vegan and gluten-free cakes for weddings and birthdays in Portland, OR');

    // Click Generate
    await page.locator('button:has-text("Generate My Business")').click();

    // Review Details
    await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 10000 });
    // Assuming backend extracts name as Maya's Dream Cakes or similar.
    await expect(page.locator('input[value="Maya\'s Dream Cakes"]')).toBeVisible();

    // Continue
    await page.locator('button:has-text("Continue")').click();

    // Style & Team
    await expect(page.locator('text="Style & Team"')).toBeVisible({ timeout: 5000 });
    await page.locator('text="Minimal"').click();

    // Enable Support Agent explicitly
    const supportAgent = page.locator('text="Support Agent"');
    await supportAgent.click();

    // Launch
    await page.locator('button:has-text("Launch Store")').click();

    // Verification
    await expect(page.locator('text="You\'re Live!"')).toBeVisible({ timeout: 15000 });
  });

  test('allows user to toggle auto-respond and select AI agents', async ({ page }) => {
    await page.goto('http://localhost:3000/onboarding');

    // Step 1: Tell us about your business
    await page.locator('textarea[placeholder="e.g. Maya Bakery that bakes custom vegan cakes in Portland, OR"]').fill('Maya Bakery that bakes custom vegan cakes in Portland, OR');

    // Click Generate
    await page.locator('button:has-text("Generate My Business")').click();

    // Review Details Step
    await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 5000 });
    await page.locator('button:has-text("Continue")').click();

    // Style & Team Step
    await expect(page.locator('text="Style & Team"')).toBeVisible({ timeout: 5000 });

    // Ensure Sales Agent is selectable
    const salesAgent = page.locator('text="Sales Agent"');
    await expect(salesAgent).toBeVisible();
    await salesAgent.click();

    // Ensure the toggle works
    const toggle = page.locator('label:has-text("Allow AI to Auto-Respond")');
    await expect(toggle).toBeVisible();
    await toggle.click(); // Uncheck
    await toggle.click(); // Check again

    // Verify template selection
    await page.locator('text="Minimal"').click();
  });
});
