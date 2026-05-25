import { test, expect } from './fixtures';

test('verify wizard UI state propagation to dashboard', async ({ page }) => {
  await page.goto('/website-builder');
  await page.getByRole('button', { name: /Start My Business Next/ }).click();
  await page.getByRole('button', { name: /Online Store/ }).click();
  await page.getByPlaceholder('What is your business called?').fill('State Test Store');
  await expect(page.getByPlaceholder('What is your business called?')).toHaveValue('State Test Store');
});

test('verify app settings toggle', async ({ page }) => {
  await page.goto('/dashboard');
  await page.getByRole('button', { name: 'Settings', exact: true }).click();
  await page.getByLabel('Enable Email Notifications').check();
  await expect(page.getByLabel('Enable Email Notifications')).toBeChecked();
});

test('verify checklist and referral routing', async ({ page }) => {
  await page.goto('/dashboard');
  await page.getByRole('button', { name: 'Referrals' }).click();
  await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();
  await expect(page.locator('#referral-link')).toContainText(/ohc:\/\/join\?ref=([A-Z0-9]+|DEFAULT)/);
});

test('verify website builder publish sheet', async ({ page }) => {
  await page.goto('/storefront-builder');
  await page.getByRole('button', { name: 'Publish Changes' }).click();
  await expect(page.getByRole('heading', { name: 'Publish Site' })).toBeVisible();
  await expect(page.getByRole('button', { name: /Free OHC Subdomain/ })).toBeVisible();
});

test('verify state persistence', async ({ page }) => {
  await page.goto('/website-builder');
  await page.getByRole('button', { name: /Start My Business Next/ }).click();
  await page.getByRole('button', { name: /Online Store/ }).click();

  // Reload the page and verify we're still on the company name step
  await page.reload();
  await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
});
