import { test, expect } from '@playwright/test';

test.describe('👂 Echo: UX Friction Elimination E2E', () => {

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Use the start business setup button as an entry point for non-authenticated users
    await page.click('button:has-text("Start Business Setup")');
    await expect(page).toHaveURL(/\/website-builder/);
  });

  test('TC1: Verify Plain Language Navigation Labels', async ({ page }) => {
    await expect(page.locator('#main-nav')).toContainText('Overview');
    await expect(page.locator('#main-nav')).toContainText('AI Assistants');
    await expect(page.locator('#main-nav')).not.toContainText('Dashboard');
    await expect(page.locator('#main-nav')).not.toContainText('Agents');
  });

  test('TC2: Verify Dashboard Sales Metric Display', async ({ page }) => {
    await page.click('a:has-text("Overview")');
    await expect(page.locator('text=Today\'s Sales')).toBeVisible();
    await expect(page.locator('text=$1,284.50')).toBeVisible();
  });

  test('TC3: Verify Contextual Hint for Quick Actions', async ({ page }) => {
    await page.click('a:has-text("Overview")');
    const hintButton = page.locator('h3:has-text("Quick Actions") button');
    await hintButton.click();
    await expect(page.locator('#quick-actions-hint')).toBeVisible();
    await expect(page.locator('#quick-actions-hint')).toContainText('shortcuts to your most common daily tasks');
  });

  test('TC4: Verify Mobile Bottom Navigation Presence', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 800 });
    await expect(page.locator('#mobile-bottom-nav')).toBeVisible();
    await expect(page.locator('#mobile-bottom-nav')).toContainText('Home');
    await expect(page.locator('#mobile-bottom-nav')).toContainText('Messages');
  });

  test('TC5: Verify Premium Loading State in Setup Wizard', async ({ page }) => {
    // Navigate to step 3
    await page.click('button:has-text("🚀 Start My Business")');
    await page.click('button:has-text("🛒 Online Store")');

    // Trigger description generation (Step 3 -> Generating)
    await page.fill('input[placeholder="What is your business called?"]', 'Maya\'s Cakes');
    await page.click('button:has-text("Generate Description")');

    await expect(page.locator('#step-generating')).toBeVisible();
    await expect(page.locator('.shimmer')).toHaveCount(2);
    await expect(page.locator('text=Designing your storefront...')).toBeVisible();
  });

});
