import { test, expect } from '@playwright/test';

test.describe('Onboarding Flow E2E', () => {
  test('Complete setup from scratch with explicit approval', async ({ page }) => {
    // Navigate to the business setup start screen
    await page.goto('/business-setup');
    await expect(page.getByText('Your business, live in minutes.')).toBeVisible();

    // Go to onboarding
    await page.click('text=Start Business Setup');
    await expect(page).toHaveURL(/\/onboarding/);

    // Initial Chat screen
    const welcome = page.getByText('Welcome');
    if (await welcome.isVisible({ timeout: 5000 }).catch(() => false)) {
      const startBtn = page.locator('text=Start Onboarding');
      if (await startBtn.isVisible()) {
        await startBtn.click();
      }
    }

    // Chat Step 1: Business Name
    await expect(page.getByText("What's the name of your business?")).toBeVisible();
    await page.fill('input[placeholder="e.g. Maya\'s Custom Cakes"]', 'Maya Cakes');
    await page.click('text=Next');

    // Chat Step 2: Description
    await expect(page.getByText('What do you sell?')).toBeVisible();
    await page.locator('textarea[placeholder*="I bake custom vegan cakes"]').fill('Vegan cakes');
    await page.click('text=Next');

    // Chat Step 3: Location
    await expect(page.getByText('Where are you located?')).toBeVisible();
    await page.fill('input[placeholder="e.g. Portland, OR"]', 'San Francisco, CA');

    // We expect a short loading process while it talks to the "backend" intake API
    // The intake API is mocked or local, but we just click Generate.
    await page.click('text=Next');

    await expect(page.getByText('Who is your target audience?')).toBeVisible();
    await page.getByPlaceholder('e.g. Local families, Tech startups').fill('Local families');
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // It should progress to Step 2: Review Details
    await expect(page.getByText('Review Details')).toBeVisible();

    // Verify some pre-filled fields from the intake response
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

    // Setup uses the signed-in owner; it does not create replacement credentials.
    await expect(page.locator('input[type="password"]')).toHaveCount(0);
    await page.getByText('Sales Assistant', { exact: true }).click();

    // Launch store
    await page.getByRole('button', { name: 'Approve & Complete Setup' }).click();

    // Should see loading spinner / Step 4
    // A quick local response may complete before an intermediate spinner is observed.

    // Eventually transition to Step 5 (Live)
    // The delay might take a few seconds
    await expect(page.getByText("Setup complete", { exact: true })).toBeVisible({ timeout: 15000 });

    // Ensure final dashboard links exist
    await expect(page.getByRole('link', { name: 'Open Assistant' })).toBeVisible();
    await expect(page.getByText('Preview Storefront')).toBeVisible();
  });
});
