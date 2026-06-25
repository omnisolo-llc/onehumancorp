import { test, expect } from '@playwright/test';

test.describe('Onboarding Flow', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test('should complete the onboarding flow on mobile', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('/onboarding');
    await expect(page).toHaveTitle(/OneHumanCorp|OHC/);

    // Step 1: Tell Us About Your Business
    await expect(page.locator('h1')).toHaveText(/Tell Us About Your Business|What do you do\?/i);
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Business');
    await page.fill('textarea[placeholder="What do you sell or what service do you provide?"]', 'I sell awesome widgets.');
    await page.click('button:has-text("Next")');

    // Step 2: More details
    await expect(page.locator('h2')).toHaveText(/Where are you located\?/i);
    await page.click('button:has-text("Next")');

    // Step 3: Admin Details & Launch
    await expect(page.locator('label:has-text("Admin Name")')).toBeVisible({ timeout: 10000 });
    await page.fill('input[placeholder="Jane Doe"]', 'Test Admin');
    await page.fill('input[placeholder="you@example.com"]', 'admin@test-business.com');
    await page.fill('input[type="password"]', 'Password123');

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
