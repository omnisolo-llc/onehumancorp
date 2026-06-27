import { test, expect } from '@playwright/test';

test.describe('Onboarding Flow', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test('should complete the onboarding flow on mobile', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('/onboarding');
    await expect(page).toHaveTitle(/OneHumanCorp|OHC/);

    // Initial Screen
    await expect(page.locator('h2', { hasText: '10-Minute Setup Wizard' })).toBeVisible({ timeout: 15000 });
    await page.click('button:has-text("Start My Business")');

    // Step 1: Name
    await expect(page.locator('h2', { hasText: "What's the name of your business?" })).toBeVisible({ timeout: 15000 });
    await page.fill('input[placeholder="e.g. Maya\'s Custom Cakes"]', 'Test Business');
    await page.click('button:has-text("Next")');

    // Step 2: What do you sell
    await expect(page.locator('h2', { hasText: "What do you sell?" })).toBeVisible({ timeout: 15000 });
    await page.fill('textarea[placeholder="e.g. I bake custom vegan cakes"]', 'I sell awesome widgets.');
    await page.click('button:has-text("Next")');

    // Step 3: Location
    await expect(page.locator('h2', { hasText: "Where are you located?" })).toBeVisible({ timeout: 15000 });
    await page.fill('input[placeholder="e.g. Portland, OR"]', 'Austin, TX');
    await page.click('button:has-text("Next")');

    // Step 4: Target Audience
    await expect(page.locator('h2', { hasText: "Who is your target audience?" })).toBeVisible({ timeout: 15000 });
    await page.fill('input[placeholder="e.g. Local families, Tech startups"]', 'Tech startups');
    await page.click('button:has-text("Next")');

    // Step 5: Review Details
    await expect(page.locator('h2', { hasText: "Review Details" })).toBeVisible({ timeout: 15000 });
    await page.click('button:has-text("Continue")');

    // Step 6: Style & Team
    await expect(page.locator('h2', { hasText: "Style & Team" })).toBeVisible({ timeout: 15000 });
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Test Admin');
    await page.fill('input[placeholder="you@example.com"]', 'admin@test-business.com');
    await page.fill('input[placeholder="••••••••"]', 'Password123!');

    // Submit
    const publishButton = page.locator('button:has-text("Approve & Publish")');
    await publishButton.waitFor({ state: 'visible' });
    await publishButton.click();

    // Loading screen
    await expect(page.locator('h2', { hasText: 'Building Your Business...' })).toBeVisible({ timeout: 15000 });

    // Success screen
    await expect(page.locator('h2', { hasText: "You're Live!" })).toBeVisible({ timeout: 30000 });
  });
});
