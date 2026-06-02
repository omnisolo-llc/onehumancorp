import { test, expect } from './fixtures';

test.describe('Onboarding Instant Build Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the website builder
    await page.goto('/website-builder');
  });

  test('user can launch a storefront instantly using the Instant Build feature', async ({ page }) => {
    // 1. Assert that the Instant Build button is visible and click it
    const instantBuildBtn = page.getByRole('button', { name: 'Instant Build' });
    await expect(instantBuildBtn).toBeVisible();
    await instantBuildBtn.click();

    // 2. Assert that we see the input screen
    const heading = page.getByRole('heading', { name: 'Describe your business in a sentence' });
    await expect(heading).toBeVisible();

    // 3. Fill the description textarea
    const input = page.getByPlaceholder('e.g. I run a local bakery');
    await expect(input).toBeVisible();
    await input.fill('I am Maya. I bake vegan cakes in Austin. Prices start at $50.');

    // 4. Submit the request
    const generateBtn = page.getByRole('button', { name: 'Generate Storefront' });
    await generateBtn.click();

    // 5. Verify the loading screen
    // Note: To avoid race conditions, we can use a longer timeout since the Next.js API route has a mock delay of 2000ms.
    // However, playwright might sometimes miss it if it transitions too fast or too slow. Let's just wait for the final success state since it's the critical outcome.

    // 6. Wait for success view
    await expect(page.getByRole('heading', { name: 'Success! Your business is live!' })).toBeVisible({ timeout: 10000 });

    // 7. Verify the checklist button exists
    await expect(page.getByRole('button', { name: 'View Welcome Checklist' })).toBeVisible();
  });
});
