import { test, expect } from './fixtures';

test.describe('Seasonal Promotion Generator Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('user can open seasonal promo generator and create a campaign', async ({ page }) => {
    // Navigate to the Seasonal Promos screen
    await page.getByRole('button', { name: 'Seasonal Promos ✨' }).click();

    await expect(page.getByRole('heading', { name: 'Seasonal Promotion Generator ✨' })).toBeVisible();

    // Fill the inputs
    await page.locator('#promo-occasion').fill('Winter Wonderland');
    await page.locator('#promo-discount').fill('25');

    // Generate Campaign
    await page.getByRole('button', { name: 'Generate Campaign' }).click();

    // Verify the result
    const resultCard = page.locator('#promo-result');
    await expect(resultCard).toBeVisible();

    const resultText = await resultCard.textContent();
    expect(resultText).toContain('Winter Wonderland Special!');
    expect(resultText).toContain('25% OFF');
    expect(resultText).toContain('Use code: WINTERWO25');
  });
});
