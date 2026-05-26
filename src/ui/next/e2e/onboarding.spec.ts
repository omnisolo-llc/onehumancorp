import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow', () => {
  test('completes full onboarding flow', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('http://localhost:3000/onboarding');

    // Step 1: Business Type
    await expect(page.locator('text="What do you do?"')).toBeVisible();
    const typeInput = page.locator('input[placeholder="e.g. Sell cakes, plumbing"]');
    await typeInput.fill('Products');
    await page.locator('button:has-text("Next")').click();

    // Step 2: Business Name
    await expect(page.locator('text="What\'s the name of your business?"')).toBeVisible();
    const nameInput = page.locator('input[placeholder="e.g. Maya\'s Cakes"]');
    await nameInput.fill('Maya\'s Bakery');
    await page.locator('button:has-text("Next")').click();

    // Step 3: Niche
    await expect(page.locator('text="What\'s your niche?"')).toBeVisible();
    const nicheInput = page.locator('input[placeholder="e.g. I bake custom wedding cakes"]');
    await nicheInput.fill('I bake custom vegan cakes');

    // Step 4: Generate Draft
    // The test was failing to find "Generate Draft" because it might be inside the Next button logic or something. Wait, the button has text "Generate Draft"

    // Create an interception for the Generate Draft API call
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      json: { initial_products: [{ name: 'Custom Cake', price: '25.00' }] }
    }));
    await page.locator('button:has-text("Generate Draft")').click();

    // The component does an API call which could take time, so we wait for the heading
    await expect(page.locator('text="Ready to Launch!"')).toBeVisible({ timeout: 10000 });

    // Choose Template & Domain
    await page.locator('button:has-text("Elegant")').click();
    await page.locator('button:has-text("Connect Custom Domain")').click();

    // Create an interception for the Publish Now API call
    await page.route('**/api/onboarding/start', route => route.fulfill({
      status: 200,
      json: { message: "Your business has been successfully launched." }
    }));
    await page.locator('button:has-text("Publish Now")').click();

    // Step 5: Live Screen
    await expect(page.locator('text="You\'re Live!"')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text="Your business has been successfully launched."')).toBeVisible();

    const dashboardLink = page.locator('a:has-text("Go to Dashboard")');
    await expect(dashboardLink).toBeVisible();
    await expect(dashboardLink).toHaveAttribute('href', '/dashboard');

    await dashboardLink.click();
    await page.waitForURL('**/dashboard');

    await expect(page.locator('text="Morning Briefing"')).toBeVisible();
    await expect(page.locator('a:has-text("Add your first product")')).toBeVisible();
  });
});
