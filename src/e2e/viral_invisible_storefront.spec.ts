import { test, expect } from './fixtures';

test.describe('Invisible Storefront Generator Onboarding Loop', () => {
  test('should allow user to instantly generate a storefront via 1-tap input', async ({ page }) => {
    await page.goto('/onboarding');

    // Switch to "Instant Magic Build" tab if it exists
    const magicBuildTab = page.getByRole('button', { name: 'Instant Magic Build ⚡' });
    if (await magicBuildTab.isVisible()) {
       await magicBuildTab.click();
    }

    // Identify the natural language input textarea
    await expect(page.getByPlaceholder('e.g., I am Maya. I bake vegan cakes in Austin. Prices start at $50.')).toBeVisible();

    // Fill in a business description
    await page.getByPlaceholder('e.g., I am Maya. I bake vegan cakes in Austin. Prices start at $50.').fill('I am a freelance handyman in Seattle doing plumbing and repairs.');

    // Click "Generate My Storefront"
    await page.getByRole('button', { name: 'Generate My Storefront' }).click();

    // Loading State
    await expect(page.getByText('Our Marketing Department is building your store...')).toBeVisible();

    // Review Screen
    await expect(page.getByText('Review Your Storefront Draft')).toBeVisible();

    // Action: Approve & Launch
    await page.getByRole('button', { name: 'Approve & Launch' }).click();

    // Final Success Screen
    await expect(page.getByText('You\'re Live!')).toBeVisible();
  });
});
