import { test, expect } from './fixtures';

test.describe('Google Business Profile Integration UI', () => {
  test.beforeEach(async ({ page }) => {
    // Dismiss the upgrade modal if it appears
    page.on('dialog', dialog => dialog.accept());
    await page.goto('/integrations');
  });

  test('shows Google Business Profile integration card in "All" view', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Google Business Profile' })).toBeVisible();
    await expect(page.getByText('Automated Local SEO and Reputation Management.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).first()).toBeVisible();
  });

  test('shows Google Business Profile integration card in "Marketing" view', async ({ page }) => {
    await page.getByRole('button', { name: 'Marketing' }).click();
    await expect(page.getByRole('heading', { name: 'Google Business Profile' })).toBeVisible();
    await expect(page.getByText('Automated Local SEO and Reputation Management.')).toBeVisible();
  });

  test('does not show Google Business Profile in "Finance" view', async ({ page }) => {
    await page.getByRole('button', { name: 'Finance' }).click();
    await expect(page.getByRole('heading', { name: 'Google Business Profile' })).not.toBeVisible();
  });

  test('can connect to Google Business Profile via OAuth mock', async ({ page }) => {
    // Setup dialog handler for the connect alert
    let alertMessage = '';
    page.on('dialog', dialog => {
      alertMessage = dialog.message();
      dialog.accept();
    });

    // We filter by marketing to make sure we click the right one, though it is not strictly necessary
    await page.getByRole('button', { name: 'Marketing' }).click();

    // Click the connect button for Google Business Profile
    // We target the parent card to make sure we click the right connect button
    const card = page.locator('div').filter({ hasText: 'Google Business Profile' }).first();
    await card.getByRole('button', { name: 'Connect' }).click();

    // Verify alert message was shown
    expect(alertMessage).toBe('Connecting Google Business Profile via OAuth...');

    // Verify it redirects to dashboard after connection
    await expect(page).toHaveURL('/dashboard');
  });
});
