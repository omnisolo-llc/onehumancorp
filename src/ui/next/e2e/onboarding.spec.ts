import { test, expect } from '@playwright/test';

test.describe('Onboarding Flow E2E', () => {
  test('Complete setup from scratch to live', async ({ page }) => {
    // Navigate to the business setup start screen
    await page.goto('/business-setup');
    await expect(page.getByText('Your business, live in minutes.')).toBeVisible();

    // Go to onboarding
    await page.click('text=Start Business Setup');
    await expect(page).toHaveURL(/\/onboarding/);

    // Initial Chat screen
    await expect(page.getByText('Welcome')).toBeVisible();
    await page.click('text=Start Onboarding');

    // Chat Step 1: Business Name
    await expect(page.getByText("What's the name of your business?")).toBeVisible();
    await page.fill('input[placeholder="e.g. Maya\'s Custom Cakes"]', 'Maya Cakes');
    await page.click('text=Next');

    // Chat Step 2: Description
    await expect(page.getByText('What do you sell?')).toBeVisible();
    await page.fill('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]', 'Vegan cakes');
    await page.click('text=Next');

    // Chat Step 3: Location
    await expect(page.getByText('Where are you located?')).toBeVisible();
    await page.fill('input[placeholder="e.g. Portland, OR"]', 'San Francisco, CA');

    // We expect a short loading process while it talks to the "backend" intake API
    // The intake API is mocked or local, but we just click Generate.
    await page.click('text=Generate My Business');

    // It should progress to Step 2: Review Details
    await expect(page.getByText('Review Details')).toBeVisible();

    // Verify some pre-filled fields from intake fallback or success
    await expect(page.locator('input[type="text"]').first()).toBeVisible();

    // Ensure First Product is filled before continuing
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Vegan Birthday Cake');
    await page.fill('input[placeholder="e.g. 50.00"]', '45.00');

    // Click Continue
    await page.click('text=Continue');

    // Step 3: Style & Team
    await expect(page.getByText('Style & Team')).toBeVisible();

    // Select domain type
    await page.click('text=Custom Domain');
    await page.click('text=Free Subdomain'); // toggle back to test it

    // Admin Account fields
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Admin');
    await page.fill('input[placeholder="you@example.com"]', 'maya@example.com');
    await page.fill('input[placeholder="••••••••"]', 'securepassword123');

    // Select an AI Agent
    await page.click('text=Sales Agent');

    // Launch store
    await page.click('text=Launch Store');

    // Should see loading spinner / Step 4
    await expect(page.getByText('Building Your Business...')).toBeVisible();

    // Eventually transition to Step 5 (Live)
    // The delay might take a few seconds
    await expect(page.getByText("You're Live!", { exact: true })).toBeVisible({ timeout: 15000 });

    // Ensure final dashboard links exist
    await expect(page.getByText('Go to Dashboard')).toBeVisible();
    await expect(page.getByText('Preview Storefront')).toBeVisible();
  });
});
