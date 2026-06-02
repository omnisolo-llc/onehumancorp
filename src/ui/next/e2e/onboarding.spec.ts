import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Mock the backend API calls
    await page.route('**/api/onboarding/intake', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_type: "Bakery",
          business_name: "Maya Bakery",
          categories: ["food"],
          initial_products: [{ name: "Cake", price: "20.00" }]
        })
      });
    });

    await page.route('**/api/onboarding/start', route => {
      // Simulate delay for loading screen
      setTimeout(() => {
        route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ message: "Your business has been successfully launched." })
        });
      }, 1000);
    });

    await page.route('**/api/onboarding/state', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({})
      });
    });
  });

  test('completes full onboarding flow', async ({ page }) => {
    // Mock the APIs
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      body: JSON.stringify({
        business_type: 'Bakery',
        business_name: 'Maya Bakery',
        categories: ['food'],
        initial_products: [{ name: 'Cake', price: '20' }]
      })
    }));

    await page.route('**/api/onboarding/start', route => route.fulfill({
      status: 200,
      body: JSON.stringify({ message: 'Success!' })
    }));

    // Navigate to onboarding page
    await page.goto('http://localhost:3000/onboarding');

    // Step 1: Business Name
    await expect(page.locator('text="Tell us about your business"')).toBeVisible();
    await expect(page.locator('text="What\'s the name of your business?"')).toBeVisible();
    await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill('Maya Cakes');
    await page.locator('button:has-text("Next")').click();

    // Step 2: What do you sell
    await expect(page.locator('text="What do you sell?"')).toBeVisible();
    await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('I bake custom vegan cakes in Portland, OR...');
    await page.locator('button:has-text("Next")').click();

    // Step 3: Location
    await expect(page.locator('text="Where are you located?"')).toBeVisible();
    await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Portland, OR');

    // Click Generate
    await page.locator('button:has-text("Generate My Business")').click();

    // 2. Wait for Review Details Step
    await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 5000 });

    // Continue to next step
    await page.locator('button:has-text("Continue")').click();

    // 3. Wait for Style & Team Step
    await expect(page.locator('text="Style & Team"')).toBeVisible({ timeout: 5000 });

    // Select Web Address and Template
    await expect(page.locator('text="Web Address"')).toBeVisible();
    await page.locator('text="Custom Domain"').click();
    await page.locator('text="Classic"').click();

    // Launch
    const [response] = await Promise.all([
      page.waitForResponse('**/api/onboarding/start'),
      page.locator('button:has-text("Launch Store")').click()
    ]);

    // 5. Live Screen
    await expect(page.locator('text="You\'re Live!"')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text="Success!"')).toBeVisible();
    await expect(page.locator('text="my-business.ohc.store"')).toBeVisible();

    const dashboardLink = page.locator('a:has-text("Go to Dashboard")');
    await expect(dashboardLink).toBeVisible();
    await expect(dashboardLink).toHaveAttribute('href', '/dashboard');

    await dashboardLink.click();
    await page.waitForURL('**/dashboard');

    await expect(page.locator('text="Morning Briefing"')).toBeVisible();
    await expect(page.locator('a:has-text("Add your first product")')).toBeVisible();
  });

  test('Maya the Home Baker Persona CUJ', async ({ page }) => {
    // This test simulates Maya, a home baker, setting up her custom cake business.
    // Business: "Maya's Dream Cakes"
    // Sell: "Custom vegan and gluten-free cakes for weddings and birthdays."
    // Location: "Portland, OR"

    await page.goto('http://localhost:3000/onboarding');

    // Step 1: Business Name
    await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill("Maya's Dream Cakes");
    await page.locator('button:has-text("Next")').click();

    // Step 2: What do you sell
    await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('Custom vegan and gluten-free cakes for weddings and birthdays.');
    await page.locator('button:has-text("Next")').click();

    // Step 3: Location
    await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Portland, OR');

    // Click Generate
    await page.locator('button:has-text("Generate My Business")').click();

    // Review Details
    await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('input[value="Maya Bakery"]')).toBeVisible();

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
    await expect(page.locator('text="Your business has been successfully launched."')).toBeVisible();
  });

  test('fails gracefully when intake API returns error', async ({ page }) => {
    // Mock intake API to return 500 error
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 500,
      body: JSON.stringify({ error: 'Internal Server Error' })
    }));

    await page.goto('http://localhost:3000/onboarding');

    // Step 1: Business Name
    await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill('Maya Cakes');
    await page.locator('button:has-text("Next")').click();

    // Step 2: What do you sell
    await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('Cakes');
    await page.locator('button:has-text("Next")').click();

    // Step 3: Location
    await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Portland, OR');

    // Click Generate
    await page.locator('button:has-text("Generate My Business")').click();

    // Error should be shown on the same step
    await expect(page.locator('text="Failed to process business details"')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text="Where are you located?"')).toBeVisible();
  });

  test('allows user to toggle auto-respond and select AI agents', async ({ page }) => {
    // Mock the APIs
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      body: JSON.stringify({
        business_type: 'Bakery',
        business_name: 'Maya Bakery',
        categories: ['food'],
        initial_products: [{ name: 'Cake', price: '20' }]
      })
    }));

    await page.goto('http://localhost:3000/onboarding');

    // Step 1: Business Name
    await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill('Maya Cakes');
    await page.locator('button:has-text("Next")').click();

    // Step 2: What do you sell
    await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('Cakes');
    await page.locator('button:has-text("Next")').click();

    // Step 3: Location
    await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Portland, OR');

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
