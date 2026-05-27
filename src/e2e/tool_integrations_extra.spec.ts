import { test, expect } from './fixtures';

test.describe('Tool Integrations Categories and Responsiveness', () => {
  test.beforeEach(async ({ page }) => {
    // Dismiss the upgrade modal if it appears
    page.on('dialog', dialog => dialog.accept());
    await page.goto('/');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Connect Tools' }).first()).toBeVisible();
  });

  test('filters by marketing category', async ({ page }) => {
    await page.getByRole('button', { name: 'Marketing' }).click();
    await expect(page.locator('div.card.glass')).toHaveCount(2); // Ayrshare, Listmonk
    await expect(page.locator('div.card.glass').filter({ hasText: 'Social Media Accounts' })).toBeVisible();
    await expect(page.locator('div.card.glass').filter({ hasText: 'Customer Emails' })).toBeVisible();
  });

  test('filters by operations category', async ({ page }) => {
    await page.getByRole('button', { name: 'Operations' }).click();
    await expect(page.locator('div.card.glass')).toHaveCount(4); // Cal.com, EasyPost, Whereby, Twilio
    await expect(page.locator('div.card.glass').filter({ hasText: 'Customer Booking' })).toBeVisible();
    await expect(page.locator('div.card.glass').filter({ hasText: 'Shipping Labels' })).toBeVisible();
  });

  test('filters by finance category', async ({ page }) => {
    await page.getByRole('button', { name: 'Finance' }).click();
    await expect(page.locator('div.card.glass')).toHaveCount(1); // Stripe
    await expect(page.locator('div.card.glass').filter({ hasText: 'Payment Processing' })).toBeVisible();
  });

  test('marks integrations as connected and updates UI', async ({ page }) => {
    const btn = page.locator('div.card.glass').filter({ hasText: 'Social Media Accounts' }).getByRole('button', { name: 'Connect my Instagram and Facebook' });

    page.once('dialog', dialog => dialog.accept());
    await btn.click();

    // Validate the button changes to "Manage" and status chip to "connected"
    await expect(page.locator('div.card.glass').filter({ hasText: 'Social Media Accounts' }).getByRole('button', { name: 'Manage' })).toBeVisible();
    await expect(page.locator('div.card.glass').filter({ hasText: 'Social Media Accounts' }).getByText('connected', { exact: true })).toBeVisible();
  });

  test('renders properly on mobile viewport', async ({ page }) => {
    // Set viewport to mobile size
    await page.setViewportSize({ width: 375, height: 812 });

    // Check elements are still visible
    await expect(page.getByRole('heading', { name: 'Connect Tools' }).first()).toBeVisible();
    await expect(page.locator('div.card.glass').first()).toBeVisible();

    // Ensure the tabs are accessible
    await expect(page.getByRole('button', { name: 'Finance' })).toBeVisible();
  });
});
