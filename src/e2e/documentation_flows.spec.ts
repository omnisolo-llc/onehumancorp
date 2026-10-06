import { test, expect } from './fixtures';

test.describe('Documentation Flows', () => {

  test.beforeEach(async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
  });

  test('Help Widget interactions and Videos', async ({ page }) => {
    // Wait for the help page to load
    await page.goto('/api/v1/ui/help.html');

    // Make sure the title renders
    await expect(page.locator('h1:has-text("In-App Help Center")')).toBeVisible();

    await expect(page.locator('input[placeholder="Search for help articles and videos..."]')).toBeAttached();
    // Verify articles are rendered from the backend
    await expect(page.getByText('Connecting a bank account to accept payments').first()).toBeVisible({ timeout: 10000 });
  });

  test('Tooltips load and display properly', async ({ page }) => {
    const loaded = page.waitForResponse(response =>
      new URL(response.url()).pathname === '/api/v1/tooltips' && response.request().method() === 'GET');
    await page.goto('/api/v1/ui/dashboard.html');
    const response = await loaded;
    expect(response.status()).toBe(200);
    const tooltips = await response.json() as Record<string, string>;
    const expectedText = tooltips['help-btn-tooltip'];
    expect(expectedText).toEqual(expect.any(String));
    expect(expectedText.trim().length).toBeGreaterThan(0);

    // The maintained Help launcher owns the tooltip; Start Tour is a plain button.
    const target = page.locator('#help-btn-tooltip');
    const helpButton = target.getByRole('button', { name: 'Open help chat', exact: true });
    await expect(target).toHaveAttribute('data-tooltip', expectedText);
    await expect(helpButton).toBeVisible();
    await helpButton.dispatchEvent('touchstart');
    const tooltip = page.getByRole('tooltip');
    await expect(tooltip).toHaveText(expectedText);
    await expect(tooltip).toBeVisible();
    await expect(target).toHaveAttribute('aria-describedby', 'help-btn-tooltip-description');

    // Scrolling cancels a held touch; a later long press can still open and dismiss it.
    await helpButton.dispatchEvent('touchmove');
    await expect(tooltip).toBeHidden();
    await helpButton.dispatchEvent('touchstart');
    await expect(tooltip).toBeVisible();
    await helpButton.dispatchEvent('touchend');
    await expect(tooltip).toBeHidden();
  });
});
