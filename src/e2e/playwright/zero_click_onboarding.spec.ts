import { test, expect } from '@playwright/test';
import path from 'path';

test.describe('Zero-Click Agentic Onboarding Flow', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('successfully onboard via unstructured text input within 60s', async ({ page }) => {
    // Navigate to the setup UI - using file:// protocol for local static HTML testing without a server
    await page.goto(`file://${path.resolve('src/ui/tauri/src/ui/setup.html')}`);

    // Click "Instant Build" button
    const instantBuildBtn = page.locator('button', { hasText: 'Instant Build' });
    await expect(instantBuildBtn).toBeVisible();
    await instantBuildBtn.click();

    // Verify we are on the instant step
    await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible();

    // Input unstructured text description
    const bioTextarea = page.locator('#instant-bio');
    await expect(bioTextarea).toBeVisible();
    await bioTextarea.fill('I sell custom cakes in Austin, here is my menu (photo)');

    // Start Generation
    const generateBtn = page.locator('#generate-storefront-btn');
    await expect(generateBtn).toBeEnabled();

    // Mock the backend API calls for stability and speed in UI test
    await page.route('**/api/onboarding/intake', async route => {
      const json = {
        business_name: "Austin Custom Cakes",
        business_type: "Bakery",
        categories: ["food"],
        location: "Austin, TX",
        target_audience: "Anyone",
        initial_products: [
            { name: "Custom Cake", price: "50.00" }
        ]
      };
      await route.fulfill({ json });
    });

    await page.route('**/api/onboarding/start', async route => {
      const json = {
        organization_id: "test-org-123",
        status: "success",
        user_id: "test-user-123"
      };
      await route.fulfill({ json });
    });

    await generateBtn.click();

    // Check loading indicator appears
    await expect(generateBtn).toContainText('Building Your Business...');

    // Wait for the redirect to success page
    await page.waitForURL('**/success.html', { timeout: 60000 });
  });
});
