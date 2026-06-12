import { test, expect } from '@playwright/test';
import { memberPage } from './fixtures';

test.describe('OHC Setup Wizard Flow', () => {
  test('should complete the interactive setup wizard flow smoothly on desktop', async ({ page }) => {
    // Go to setup page (assuming index redirects to setup or we can load setup directly)
    // We navigate to /setup.html
    await page.goto('http://localhost:3000/setup.html');

    // Check initial UI loading
    await expect(page.locator('h1').first()).toBeVisible();
    await expect(page.getByText('How do you work?')).toBeVisible();

    // Context step
    await page.locator('.radio-option', { hasText: 'Storefront or Cafe' }).click();
    await page.locator('.next-step-btn[data-next="step-categories"]').click();

    // Categories step
    const categorySelect = page.locator('#business-categories');
    await expect(categorySelect).toBeVisible();
    // Wait for populateCategories to fill options
    await page.waitForTimeout(100);
    await categorySelect.selectOption('Bakery');
    await page.locator('.next-step-btn[data-next="step-name"]').click();

    // Name step
    await page.locator('#business-name').fill('Test Bakery');
    await page.locator('.next-step-btn[data-next="step-assistant"]').click();

    // Assistant step
    await page.locator('#assistant-name').fill('Buddy');
    await page.locator('#assistant-tone').selectOption('Friendly');
    await page.locator('.next-step-btn[data-next="step-admin"]').click();

    // Admin step
    await page.locator('#admin-email').fill('admin@testbakery.local');
    await page.locator('#admin-password').fill('SuperSecretPassword123');
    await page.locator('.next-step-btn[data-next="step-offer"]').click();

    // Offer step
    await page.locator('#first-offer').fill('Chocolate Cake');
    await page.locator('.next-step-btn[data-next="step-template"]').click();

    // Template step
    await page.locator('#template-selection').selectOption('Modern');

    // Make sure finish btn is visible before interacting
    await expect(page.locator('#finish-btn')).toBeVisible();

    // Intercept backend call so it doesn't try to invoke Tauri or real unconfigured backend in tests
    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ organization_id: 'test-org-123' })
      });
    });

    // Save state endpoint for "Save Draft" test
    await page.route('**/api/onboarding/state', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    // Click Save Draft
    const saveDraftBtn = page.locator('.save-draft-btn').last();
    await expect(saveDraftBtn).toBeVisible();
    await saveDraftBtn.click();
    await expect(saveDraftBtn).toHaveText('Saved!', { timeout: 3000 });

    // Submit setup
    await page.locator('#finish-btn').click();

    // Assuming we redirect to success.html on complete
    await page.waitForURL('**/success.html', { timeout: 10000 });
    await expect(page.url()).toContain('success.html');
  });

  test('should support 375px mobile view without horizontal scroll and minimum 44px touch targets', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('http://localhost:3000/setup.html');

    // Evaluate horizontal scroll
    const hasHorizontalScroll = await page.evaluate(() => {
        return document.documentElement.scrollWidth > window.innerWidth;
    });
    expect(hasHorizontalScroll).toBe(false);

    // Verify touch targets height
    const btnBox = await page.locator('.next-step-btn').first().boundingBox();
    expect(btnBox?.height).toBeGreaterThanOrEqual(44);

    const inputbox = await page.locator('.radio-option').first().boundingBox();
    expect(inputbox?.height).toBeGreaterThanOrEqual(44);
  });
});
