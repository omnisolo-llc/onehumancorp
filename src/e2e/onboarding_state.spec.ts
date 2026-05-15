import { test, expect } from '@playwright/test';

test.describe('Onboarding State Resumption', () => {
  test('User can sign up, start wizard, refresh, and resume state', async ({ page }) => {
    // Navigate to the app
    await page.goto('/signup');

    // Fill out signup
    await page.fill('#signup-screen input[type="email"]', 'testresume@example.com');
    await page.fill('#signup-screen input[type="password"]', 'password123');
    await page.click('#signup-screen button:has-text("Sign Up")');

    // Click verify
    await page.click('#signup-screen button:has-text("Verify")');

    // We should be on step-1 of the setup wizard
    await expect(page.locator('#step-1')).toBeVisible();

    // Progress to step 2
    await page.click('#step-1 button:has-text("🚀 Start My Business")');
    await expect(page.locator('#step-2')).toBeVisible();

    // Select business type, progresses to step 3
    await page.click('#step-2 button:has-text("🛒 Online Store")');
    await expect(page.locator('#step-3')).toBeVisible();

    // Fill in name
    await page.fill('#step-3 input', 'Test Resumed Store');

    // Progress to step 4, waiting for API to persist state to avoid race conditions
    await Promise.all([
        page.waitForResponse(response => response.url().includes('/api/onboarding/state') && response.request().method() === 'POST'),
        page.click('#step-3 button:has-text("Next →")')
    ]);
    await expect(page.locator('#step-4')).toBeVisible();

    // Now reload the page (simulate user leaving and coming back)
    await page.reload();

    // It should automatically load state and jump back to step-4 since currentStep was 4
    await expect(page.locator('#setup-screen')).toBeVisible();

    await expect(page.locator('#step-4')).toBeVisible({ timeout: 5000 });

    // Complete step 4
    await page.fill('#step-4 input', 'Vegan Custom Cakes');
    await page.click('#step-4 button:has-text("Next →")');
    await expect(page.locator('#step-5')).toBeVisible();

    // Complete step 5
    await page.fill('#step-5 input[type="text"]', 'Signature Vegan Cake');
    await page.click('#step-5 button:has-text("✨ Generate AI Description")');
    await expect(page.locator('#step-5 button:has-text("✨ Regenerate")')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('#ai-product-desc')).not.toBeEmpty();

    // Next to step 6
    await page.click('#step-5 button:has-text("Next →")');
    await expect(page.locator('#step-6')).toBeVisible();

    // Next to step 8
    await page.click('#step-6 button:has-text("Both Online & In-person")');
    await expect(page.locator('#step-8')).toBeVisible();

    // Next to step 9
    await page.click('#step-8 button:has-text("Modern (Light)")');
    await expect(page.locator('#template-preview')).toBeVisible();
    await page.click('#step-8 button:has-text("Next →")');
    await expect(page.locator('#step-9')).toBeVisible();

    // Next to step 10
    await page.click('#step-9 button:has-text("🌐 Get a Free OHC Domain (.ohc.app)")');
    await expect(page.locator('#step-10')).toBeVisible();

    // Finally to step 100
    await page.click('#step-10 button:has-text("Publish my business →")');
    await expect(page.locator('#step-100')).toBeVisible();

    // Verify Confetti loaded
    await expect(page.locator('h1:has-text("BUSINESS LIVE!")')).toBeVisible();
  });
});
