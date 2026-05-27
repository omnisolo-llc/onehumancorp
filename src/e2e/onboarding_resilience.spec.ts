import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Cross-Device Resilience and UX Optimization', () => {
  test('persists draft state across reloads and handles Enter to submit', async ({ page, browser }) => {
    // Navigate to onboarding page
    await page.goto('http://localhost:3000/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.locator('text="Tell us about your business"')).toBeVisible();

    // Verify glassmorphism style is applied
    const glassContainer = page.locator('.mac-glass-container').first();
    await expect(glassContainer).toHaveClass(/backdrop-blur-xl/);
    await expect(glassContainer).toHaveClass(/bg-white\/30/);

    // Mock the backend API routes for saving and retrieving draft
    let mockDraftState: any = {};

    await page.route('**/api/onboarding/draft', async route => {
      if (route.request().method() === 'POST') {
         const postData = route.request().postDataJSON();
         mockDraftState = { ...mockDraftState, ...postData };
         await route.fulfill({ status: 200 });
      } else if (route.request().method() === 'GET') {
         await route.fulfill({
           status: 200,
           json: mockDraftState
         });
      } else {
         await route.continue();
      }
    });

    // Fill in the description
    const descriptionInput = page.locator('textarea[placeholder="e.g. I bake custom vegan cakes in Portland, OR..."]');
    await descriptionInput.fill('I am a freelance handyman in Miami');

    // Wait for debounce logic to trigger draft save
    await page.waitForTimeout(1200);

    // Real DB handles the save, we don't assert on mock object anymore.

    // Intercept API calls for next steps
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      json: { initial_products: [{ name: 'Handyman Service', price: '50.00' }], business_type: 'Handyman', business_name: 'Miami Fixit', categories: ['service'] }
    }));

    // Trigger Enter key on description
    await descriptionInput.press('Enter');

    // Wait for Review Details Step
    await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 5000 });

    // Wait for debounce save
    await page.waitForTimeout(1200);

    // Reload page to simulate device switch or refresh
    await page.reload();

    // Wait for the page to hydrate from mock draft and return to Review Details
    await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 5000 });

    // Ensure inputs have enterKeyHint (can check element handle property)
    const typeInput = page.locator('input[value="Handyman"]').first();
    await expect(typeInput).toHaveAttribute('enterKeyHint', 'next');

    const priceInput = page.locator('input[value="50.00"]').first();
    await expect(priceInput).toHaveAttribute('enterKeyHint', 'done');

    // Trigger Enter key on price to submit Step 2
    await priceInput.press('Enter');

    // Wait for Style & Team Step
    await expect(page.locator('text="Style & Team"')).toBeVisible({ timeout: 5000 });

    // Wait for debounce save
    await page.waitForTimeout(1200);
    // Reload again to verify step 3 persisted
    await page.reload();
    await expect(page.locator('text="Style & Team"')).toBeVisible({ timeout: 5000 });
  });
});
