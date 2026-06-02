import { test, expect } from '@playwright/test';

test.describe('Onboarding Personas CUJ', () => {
  test.beforeEach(async ({ page }) => {
    await page.route('**/api/onboarding/state', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({})
      });
    });
  });

  test('Fatima - Food Cart Operator Persona', async ({ page }) => {
    // Mock intake for Fatima
    await page.route('**/api/onboarding/intake', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_type: "Halal Food Cart",
          business_name: "Fatima's Halal Cart",
          categories: ["food"],
          initial_products: [{ name: "Chicken Over Rice", price: "12.00" }]
        })
      });
    });

    await page.route('**/api/onboarding/start', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ message: "Successfully launched Fatima's Halal Cart!", organization_id: "org-fatima" })
      });
    });

    await page.goto('http://localhost:3000/onboarding');

    // Step 1: Business Name
    await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill("Fatima's Halal Cart");
    await page.locator('button:has-text("Next")').click();

    // Step 2: What do you sell
    await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('Delicious Halal food, chicken over rice, and falafel.');
    await page.locator('button:has-text("Next")').click();

    // Step 3: Location
    await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Queens, NY');
    await page.locator('button:has-text("Generate My Business")').click();

    // Review Details
    await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('input[value="Fatima\'s Halal Cart"]')).toBeVisible();
    await expect(page.locator('text="FOOD"')).toBeVisible();

    // Check pre-selected agents (Fatima should have Support Agent)
    await page.locator('button:has-text("Continue")').click();

    // Style & Team
    await expect(page.locator('text="Style & Team"')).toBeVisible();
    const supportAgent = page.locator('div:has-text("Support Agent")').last();
    // We expect it to be pre-selected (having the checkmark icon)
    await expect(supportAgent.locator('svg')).toBeVisible();

    // Launch
    await page.locator('button:has-text("Launch Store")').click();

    // Loading screen verification
    await expect(page.locator('text="Building Your Halal Food Cart..."')).toBeVisible();
    await expect(page.locator('text="Curating your delicious menu..."')).toBeVisible();

    // Success
    await expect(page.locator('text="You\'re Live!"')).toBeVisible({ timeout: 15000 });
  });

  test('Carlos - Freelance Handyman Persona', async ({ page }) => {
    // Mock intake for Carlos
    await page.route('**/api/onboarding/intake', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          business_type: "Handyman Services",
          business_name: "Carlos Quick Fixes",
          categories: ["services"],
          initial_products: [{ name: "Home Repair Assessment", price: "50.00" }]
        })
      });
    });

    await page.route('**/api/onboarding/start', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ message: "Successfully launched Carlos Quick Fixes!", organization_id: "org-carlos" })
      });
    });

    await page.goto('http://localhost:3000/onboarding');

    // Step 1: Business Name
    await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill("Carlos Quick Fixes");
    await page.locator('button:has-text("Next")').click();

    // Step 2: What do you sell
    await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('Plumbing, painting, and general home repairs.');
    await page.locator('button:has-text("Next")').click();

    // Step 3: Location
    await page.locator('input[placeholder="e.g. Portland, OR"]').fill('San Antonio, TX');
    await page.locator('button:has-text("Generate My Business")').click();

    // Review Details
    await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text="SERVICES"')).toBeVisible();

    await page.locator('button:has-text("Continue")').click();

    // Style & Team
    await expect(page.locator('text="Style & Team"')).toBeVisible();
    // Carlos wants "Bold" template
    await page.locator('text="Bold"').click();

    // Launch
    await page.locator('button:has-text("Launch Store")').click();

    // Loading screen verification
    await expect(page.locator('text="Building Your Handyman Services..."')).toBeVisible();
    await expect(page.locator('text="Organizing your service catalog..."')).toBeVisible();

    // Success
    await expect(page.locator('text="You\'re Live!"')).toBeVisible({ timeout: 15000 });
  });
});
