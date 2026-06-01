import { test, expect } from './fixtures';

test.describe('Viral Trial Extension Soft Paywall Loop', () => {
  test('user hits soft paywall and claims trial extension via sharing', async ({ page }) => {
    // Navigate to the Seasonal Promos screen
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Clear local storage 'has_pro' flag to trigger soft paywall
    await page.evaluate(() => {
      localStorage.setItem('has_pro', 'false');
    });

    // Go to Seasonal Promos
    await page.getByRole('button', { name: 'Seasonal Promos ✨' }).click();
    await expect(page.getByRole('heading', { name: 'Seasonal Promotion Generator ✨' })).toBeVisible();

    // Fill the inputs
    await page.locator('#promo-occasion').fill('Spring Sale');
    await page.locator('#promo-discount').fill('30');

    // Attempt to Generate Campaign, which should trigger soft paywall
    await page.getByRole('button', { name: 'Generate Campaign' }).click();

    // Verify Soft Paywall is visible
    const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywallHeading).toBeVisible();
    await expect(page.getByText('Seasonal Promotion Generator is a Pro feature')).toBeVisible();

    // Mock window.open to avoid actually opening a new tab for Twitter
    await page.evaluate(() => {
      (window as any).open = function() { return null; };
    });

    // Click the share button loop
    await page.getByRole('button', { name: 'Share on X to claim 14 Days Free' }).click();

    // Wait for the modal to disappear and generation to start/finish
    await expect(paywallHeading).toBeHidden();

    // The result should now be visible because handleGenerate() is called after claiming
    const resultCard = page.locator('#promo-result');
    await expect(resultCard).toBeVisible();

    const resultText = await resultCard.textContent();
    expect(resultText).toContain('Spring Sale Special!');
    expect(resultText).toContain('30% OFF');

    // Verify local storage is updated
    const hasPro = await page.evaluate(() => localStorage.getItem('has_pro'));
    expect(hasPro).toBe('true');
  });
});
