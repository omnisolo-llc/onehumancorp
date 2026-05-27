import { test, expect } from './fixtures';

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
    // Navigate to step 3 by filling in description and clicking generate
    await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Test Company", business_type: "Online Store", categories: ["physical"], initial_products: [{ name: "Custom Cookies", price: "24.99" }] }) }));
    await page.route('**/api/onboarding/start', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ message: "Your business has been successfully launched.", organization_id: "org_123" }) }));

    await page.fill('textarea[placeholder="e.g. I bake custom vegan cakes in Portland, OR..."]', 'Maya\'s Cakes');
    await page.click('button:has-text("Generate Storefront")');

    await expect(page.locator('#step-2')).toBeVisible();
    await page.click('button:has-text("Continue")');
    await expect(page.locator('#step-3')).toBeVisible();

    const requestPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/start') && request.method() === 'POST'
    );
    await page.click('button:has-text("Launch Store")');

    await expect(page.locator('#step-4')).toBeVisible();
    await expect(page.locator('.spin')).toHaveCount(1);
    await expect(page.locator('text=Building Your Business...')).toBeVisible();
  });

});
