import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test('completes full onboarding flow', async ({ page }) => {
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

    // Select Template and Launch
    await page.locator('text="Classic"').click();
    await page.locator('button:has-text("Launch Store")').click();

    // 4. Loading screen
    await expect(page.locator('text="Building Your Business..."')).toBeVisible({ timeout: 5000 });

    // 5. Live Screen
    await expect(page.locator('text="You\'re Live!"')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text="Your business has been successfully launched."')).toBeVisible();
    await expect(page.locator('text="my-business.ohc.store"')).toBeVisible();

    const dashboardLink = page.locator('a:has-text("Go to Dashboard")');
    await expect(dashboardLink).toBeVisible();
    await expect(dashboardLink).toHaveAttribute('href', '/dashboard');

    await dashboardLink.click();
    await page.waitForURL('**/dashboard');

    await expect(page.locator('text="Morning Briefing"')).toBeVisible();
    await expect(page.locator('a:has-text("Add your first product")')).toBeVisible();
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

  test('restores user state from backend across devices seamlessly', async ({ page }) => {
    // Mock a backend response that has partially filled state (user started on phone and is now on desktop)
    await page.route('**/api/onboarding/state', async route => {
      if (route.request().method() === 'GET') {
        return route.fulfill({
          status: 200,
          body: JSON.stringify({
            wizardState: {
              step: 2,
              chatStep: 3,
              businessName: "Resumed Maya Bakery",
              whatYouSell: "I bake amazing resumed cakes",
              location: "New York, NY",
              businessType: "Bakery",
              categories: ["food", "dessert"],
              firstProductName: "Custom Cake",
              firstProductPrice: "50",
              aiAgents: [],
              aiAutoRespond: true
            }
          })
        });
      }
      return route.continue();
    });

    await page.goto('http://localhost:3000/onboarding');

    // Wait for the state to load and assert that the user is placed on Step 2 with the fields populated correctly
    await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 5000 });

    // Assert fields are populated from state
    await expect(page.locator('label:has-text("Business Name") + input')).toHaveValue("Resumed Maya Bakery");
    await expect(page.locator('label:has-text("Business Type") + input')).toHaveValue("Bakery");
    await expect(page.locator('label:has-text("Categories") + input')).toHaveValue("food, dessert");
    await expect(page.locator('label:has-text("First Product") + input')).toHaveValue("Custom Cake");
    await expect(page.locator('label:has-text("Price") + input')).toHaveValue("50");
  });
