import { test, expect } from '@playwright/test';

test.describe('Instant Build Flow', () => {
  test('should go from login to instant build generation', async ({ page }) => {
    // Start at login
    await page.goto('/login');
    await expect(page.locator('text="One Human Corp"')).toBeVisible();

    // Click Sign Up toggle
    await page.locator('button:has-text("Don\'t have an account? Sign Up")').click();

    // Fill in sign up details
    await page.fill('input[type="email"]', 'newuser_instant@example.com');
    await page.fill('input[type="password"]', 'StrongPass123!');

    // Click Sign Up button
    await page.locator('button:has-text("Sign Up")').click();

    // Now it should redirect to Setup Wizard step 1
    await expect(page.locator('text=/Welcome/i')).toBeVisible();

    // Choose Instant Build
    await page.locator('text="⚡ Instant Build (AI) →"').click();

    // Step 11: Instant Build bio entry
    await expect(page.locator('text=/Tell us about your business/i')).toBeVisible();

    // Enter a bio about a bakery
    await page.fill('input[placeholder="e.g. I run a local bakery called Maya\'s Cakes..."]', 'I run a lovely bakery that makes custom cakes for events.');

    // Click Generate Storefront
    await page.locator('text="Generate Storefront →"').click();

    // Wait for the backend process and go to Step 9 (Review and Launch)
    await expect(page.locator('text="Ready to launch!"')).toBeVisible({ timeout: 10000 });

    // Validate the heuristic values we set in the backend for 'bakery'
    await expect(page.locator('text="Business: AI Generated Bakery"')).toBeVisible();
    await expect(page.locator('text="Type: Online Store"')).toBeVisible();
    await expect(page.locator('text="Template: Modern"')).toBeVisible();
    await expect(page.locator('text="First Product: Custom Cake"')).toBeVisible();

    // Launch!
    await page.locator('button:has-text("Publish My Business →")').click();

    // Assert that we reached Welcome Checklist
    await expect(page.locator('text="Welcome Checklist"')).toBeVisible({ timeout: 10000 });
  });
});
