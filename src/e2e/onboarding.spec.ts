import { test, expect } from '@playwright/test';

test.describe('End-to-End Onboarding Flow', () => {
  test('should go from login to welcome checklist in one deterministic flow', async ({ page }) => {
    // Start at login
    await page.goto('/login');
    await expect(page.locator('text="One Human Corp"')).toBeVisible();

    // Click Sign Up toggle
    await page.locator('button:has-text("Don\'t have an account? Sign Up")').click();

    // Fill in sign up details
    await page.fill('input[type="email"]', 'newuser@example.com');
    await page.fill('input[type="password"]', 'StrongPass123!');

    // Click Sign Up button
    await page.locator('button:has-text("Sign Up")').click();

    // Now it should redirect to Setup Wizard step 1
    await expect(page.locator('text=/Welcome/i')).toBeVisible();
    await page.locator('button:has-text("Next")').click();

    // Step 2: What kind of business
    await expect(page.locator('text=/kind of business/i')).toBeVisible();
    await page.locator('text="🛒 Online Store"').click();
    await page.locator('button:has-text("Next")').click();

    // Step 3: Company Name
    await expect(page.locator('text=/company called/i')).toBeVisible();
    await page.fill('input[type="text"]', 'My Awesome Bakery');
    await page.locator('button:has-text("Next")').click();

    // Step 4: What do you sell
    await expect(page.locator('text=/what do you sell/i')).toBeVisible();
    await page.locator('text="📦 Physical products"').click();
    await page.locator('button:has-text("Next")').click();

    // Step 5: Payment preferences
    await expect(page.locator('text=/payment/i')).toBeVisible();
    await page.locator('text="🌐 Online only"').click();
    await page.locator('button:has-text("Next")').click();

    // Step 6: Choose Template
    await expect(page.locator('text="Choose a Template"')).toBeVisible();
    await page.locator('text="✨ Modern"').click();
    await page.locator('button:has-text("Next")').click();

    // Step 7: Add first product
    await expect(page.locator('text=/Add your first product/i')).toBeVisible();
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Cake');
    // Simulate AI generation click
    await page.locator('text="✨ Auto-suggest Description"').click();
    // Verify AI generated description
    await expect(page.locator('input[placeholder="Product description"]')).toHaveValue(/AI Generated Description/i);
    // Click upload photo
    await page.locator('text="📷 Upload Photo (Crop)"').click();
    await expect(page.locator('text="✅ Photo Uploaded"')).toBeVisible();
    await page.locator('button:has-text("Next")').click();

    // Step 8: Domain selection
    await expect(page.locator('text=/domain/i')).toBeVisible();
    await page.locator('text="🌐 Free OHC Domain"').click();
    await page.locator('button:has-text("Next")').click();

    // Step 9: Review and Launch
    await expect(page.locator('text="Ready to launch!"')).toBeVisible();
    await page.locator('button:has-text("Publish My Business →")').click();

    // Assert that we reached Welcome Checklist
    await expect(page.locator('text="Welcome Checklist"')).toBeVisible({ timeout: 10000 });

    // Assert progress text starts at 0 of 3
    await expect(page.locator('text="0 of 3 completed"')).toBeVisible();
  });
});
