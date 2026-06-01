import { test, expect } from '@playwright/test';

test.describe('Onboarding Extended Flow', () => {
  test('allows resuming onboarding from a previous state', async ({ page }) => {
    // 1. Mock the state API to return a partial state
    await page.route('**/api/onboarding/state', async (route) => {
      if (route.request().method() === 'GET') {
        await route.fulfill({
          status: 200,
          body: JSON.stringify({
            wizardState: {
              step: 2,
              businessName: 'Maya Cakes Resumed',
              businessType: 'Bakery',
              categories: ['food'],
              firstProductName: 'Cupcake',
              firstProductPrice: '5.00'
            }
          })
        });
      } else {
        await route.fulfill({ status: 200 });
      }
    });

    await page.goto('http://localhost:3000/onboarding');

    // 2. Expect resume modal
    await expect(page.locator('text="Resume Setup?"')).toBeVisible({ timeout: 5000 });
    await page.locator('button:has-text("Resume Progress")').click();

    // 3. Should be on step 2 with data filled
    await expect(page.locator('text="Review Details"')).toBeVisible();
    await expect(page.locator('input[label="Business Name"]')).toHaveValue('Maya Cakes Resumed');

    // 4. Continue
    await page.locator('button:has-text("Continue")').click();
    await expect(page.locator('text="Style & Team"')).toBeVisible();
  });

  test('handles intake network failure gracefully', async ({ page }) => {
    await page.goto('http://localhost:3000/onboarding');

    // Step 1: Business Name
    await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill('Failure Test');
    await page.locator('button:has-text("Next")').click();

    // Step 2: What do you sell
    await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('Some cakes');
    await page.locator('button:has-text("Next")').click();

    // Step 3: Location
    await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Portland, OR');

    // Mock intake failure
    await page.route('**/api/onboarding/intake', route => route.abort('failed'));

    // Click Generate
    await page.locator('button:has-text("Generate My Business")').click();

    // Error should be shown
    await expect(page.locator('text="Backend connection failed"')).toBeVisible({ timeout: 5000 });
  });

  test('verifies mobile-first layout (375px)', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('http://localhost:3000/onboarding');

    const setupScreen = page.locator('#setup-screen');
    const box = await setupScreen.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);

    await expect(page.locator('text="Step 1 of 3"')).toBeVisible();
    await expect(page.locator('div[style*="width: 33.3333%"]')).toBeVisible();
  });

  test('shows tooltips for AI agents on step 3', async ({ page }) => {
    // Navigate to step 3
    await page.route('**/api/onboarding/state', async (route) => {
        if (route.request().method() === 'GET') {
          await route.fulfill({
            status: 200,
            body: JSON.stringify({
              wizardState: {
                step: 3,
                businessName: 'Maya Cakes',
                businessType: 'Bakery',
                categories: ['food'],
                firstProductName: 'Cupcake',
                firstProductPrice: '5.00'
              }
            })
          });
        } else {
          await route.fulfill({ status: 200 });
        }
      });

    await page.goto('http://localhost:3000/onboarding');
    await page.locator('button:has-text("Resume Progress")').click();

    await expect(page.locator('text="Style & Team"')).toBeVisible();

    // Hover over Sales Agent
    await page.locator('text="Sales Agent"').hover();

    // Tooltip should appear
    await expect(page.locator('text="Responds to product questions and closes sales 24/7."')).toBeVisible();
  });

  test('displays enhanced building screen during launch', async ({ page }) => {
    // Navigate to step 3 and trigger launch
    await page.route('**/api/onboarding/state', async (route) => {
        if (route.request().method() === 'GET') {
          await route.fulfill({
            status: 200,
            body: JSON.stringify({
              wizardState: {
                step: 3,
                businessName: 'Maya Cakes',
                businessType: 'Bakery',
                categories: ['food'],
                firstProductName: 'Cupcake',
                firstProductPrice: '5.00'
              }
            })
          });
        } else {
          await route.fulfill({ status: 200 });
        }
      });

    // Mock start API with delay
    await page.route('**/api/onboarding/start', async (route) => {
        await new Promise(resolve => setTimeout(resolve, 2000));
        await route.fulfill({
          status: 200,
          body: JSON.stringify({ message: 'Success' })
        });
    });

    await page.goto('http://localhost:3000/onboarding');
    await page.locator('button:has-text("Resume Progress")').click();

    await page.locator('button:has-text("Launch Store")').click();

    // Verify building screen
    await expect(page.locator('text="Building Your Empire..."')).toBeVisible();
    await expect(page.locator('text="Crafting your unique brand identity..."')).toBeVisible();
    await expect(page.locator('text="Igniting your digital presence..."')).toBeVisible();

    // Wait for success
    await expect(page.locator('text="You\'re Live!"')).toBeVisible({ timeout: 10000 });
  });
});
