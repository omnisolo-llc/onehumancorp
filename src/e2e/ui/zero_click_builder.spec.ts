import { test, expect } from '@playwright/test';

test.describe('Zero-Click Business Generator CUJ', () => {
  test('User can generate a business with a single prompt', async ({ page }) => {
    // Navigate to the Zero-Click Builder page
    await page.goto('/zero-click-builder');

    // Expect the title to be present
    await expect(page.getByText('Zero-Click Business Generator')).toBeVisible();

    // Fill the prompt
    await page.fill('textarea#prompt', "I am a home baker in Austin selling custom vegan cakes and cupcakes.");

    // Submit the form
    await page.click('button[type="submit"]:has-text("Generate My Business")');

    // Wait for generation to complete and the success message to appear
    await expect(page.getByText('Your business is live!')).toBeVisible({ timeout: 15000 });

    // Verify the generated data is shown
    await expect(page.getByText('Mock Business')).toBeVisible();
    await expect(page.getByText('Products Generated')).toBeVisible();
    // The generated product count may vary between 3, 4, or 5 since process_intake is LLM-driven.
    await expect(page.locator('dt:has-text("Products Generated") + dd')).not.toBeEmpty();
  });
});
