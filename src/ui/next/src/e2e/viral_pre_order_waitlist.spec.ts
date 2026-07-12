import { test, expect } from '../../../../e2e/fixtures';

test.describe('Pre-Order Waitlist Engine', () => {
  test('should load the dashboard and click the waitlist link', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard');

    // Check that the waitlist link is visible
    const waitlistLink = page.locator('a[id="pre-order-waitlist-link"]');
    await expect(waitlistLink).toBeVisible();

    // Click the link and wait for navigation
    await waitlistLink.click();
    await page.waitForURL('**/pre-order-widget');

    // Verify the widget page loaded
    await expect(page.locator('h1:has-text("Pre-Order Waitlist Engine")')).toBeVisible();
  });

  test('should configure the widget and show live preview', async ({ page }) => {
    await page.goto('/pre-order-widget');

    // Fill out the configuration
    await page.fill('input[placeholder="e.g. The Vegan Chocolate Cake"]', 'Super limited t-shirt');
    await page.fill('input[placeholder="e.g. Get 10% off your pre-order!"]', 'Get 15% off when it drops!');

    // Verify live preview updates
    await expect(page.locator('h2:has-text("Super limited t-shirt")')).toBeVisible();
    await expect(page.locator('p:has-text("Get 15% off when it drops!")')).toBeVisible();

    // Change theme to dark
    await page.click('button:has-text("Dark")');

    // Open embed modal
    await page.click('button:has-text("Get Widget Embed Code")');
    await expect(page.locator('h2:has-text("Embed Your Waitlist")')).toBeVisible();

    // Verify embed code has correct attributes
    const embedCode = await page.textContent('.font-mono');
    expect(embedCode).toContain('data-product="Super limited t-shirt"');
    expect(embedCode).toContain('data-offer="Get 15% off when it drops!"');
    expect(embedCode).toContain('data-theme="dark"');
  });
});
