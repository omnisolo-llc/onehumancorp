import { test, expect } from './fixtures';

test.describe('Global Help Widget', () => {
  test('should be present and functional on dashboard', async ({ page }) => {
    await page.goto('/api/v1/ui/dashboard.html');

    await expect(page).toHaveURL(url => url.pathname === '/dashboard');
    const helpBtn = page.getByRole('button', { name: 'Open help chat', exact: true });
    await expect(helpBtn).toBeVisible();
    await helpBtn.click();
    const chatWidget = page.locator('#omnisolo-floating-help-widget');
    await expect(chatWidget).toBeVisible();
    await expect(chatWidget.getByRole('heading', { name: 'Help Center', exact: true })).toBeVisible();

    await chatWidget.getByRole('button', { name: 'Close Help Widget', exact: true }).click();
    await expect(chatWidget).toBeHidden();
    await expect(helpBtn).toBeFocused();
    await helpBtn.click();
    await expect(chatWidget).toBeVisible();
    await helpBtn.press('Escape');
    await expect(chatWidget).toBeHidden();
    await expect(helpBtn).toBeFocused();
    await expect(page).toHaveURL(url => url.pathname === '/dashboard');

    const walkBtn = page.getByRole('button', { name: 'Start Tour', exact: true });
    await walkBtn.click();
    const walkthrough = page.getByRole('dialog', { name: 'Business Analytics walkthrough step', exact: true });
    await expect(walkthrough).toBeVisible();
    const overlay = page.locator('.omnisolo-walkthrough-overlay');
    await expect(overlay).toBeVisible();
    await walkthrough.getByRole('button', { name: 'Close walkthrough', exact: true }).click();
    await expect(walkthrough).toBeHidden();
    await expect(overlay).toBeHidden();
  });

  test('should be present and functional on POS', async ({ page }) => {
    await page.goto('/api/v1/ui/pos.html');

    // The floating help button should be visible here too
    const helpBtn = page.locator('#ohc-floating-help-btn').first();
    await expect(helpBtn).toBeVisible();
  });
});
