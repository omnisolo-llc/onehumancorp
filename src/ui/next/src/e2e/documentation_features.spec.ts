import { test, expect } from '../../../../e2e/fixtures';

test.describe('Documentation Features', () => {

  test('Changelog page loads correctly', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.getByTestId('changelog-title')).toBeVisible();
    await expect(page.getByText('Release Notes & Changelog')).toBeVisible();
  });

  test('API Docs page loads correctly', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.getByTestId('api-docs-title')).toBeVisible();
    await expect(page.getByText('Advanced:')).toBeVisible();
  });

  test('Help Center and Chat opens', async ({ page }) => {
    await page.goto('/');
<<<<<<< HEAD
    const helpButton = page.locator('#omnisolo-floating-help-btn').first();
=======
    const helpButton = page.locator('#omnisolo-floating-help-btn');
>>>>>>> 7b2282170 (🗺️ Guide: [blocked no-work finding: F14: economics/owner outcomes] (#38156))
    await expect(helpButton).toBeVisible();
    await helpButton.scrollIntoViewIfNeeded();
    await helpButton.click({ force: true });

    // Help Widget appears
    const askAnythingTab = page.getByText('Ask anything');
    await expect(askAnythingTab).toBeVisible();
    await askAnythingTab.click({ force: true });

    // Help chat widget
    await expect(page.locator('#omnisolo-floating-help-widget')).toBeVisible();
  });

});

  test('Walkthroughs can be triggered', async ({ page }) => {
    await page.goto('/');
<<<<<<< HEAD
    const helpButton = page.locator('#omnisolo-floating-help-btn').first();
=======
    const helpButton = page.locator('#omnisolo-floating-help-btn');
>>>>>>> 7b2282170 (🗺️ Guide: [blocked no-work finding: F14: economics/owner outcomes] (#38156))
    await expect(helpButton).toBeVisible();
    await helpButton.scrollIntoViewIfNeeded();
    await helpButton.click({ force: true });

    // Check if the walkthrough start button is there
    const tourButton = page.locator('button', { hasText: 'Tour: Store Setup' });
    await expect(tourButton).toBeVisible();
  });
