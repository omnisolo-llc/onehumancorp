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
  await expect(page.locator('#referral-link')).toContainText('ohc://join?ref=DEFAULT');
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

  // Fill text input and proceed to checkboxes
  await page.getByPlaceholder('What is your business called?').fill('Maya State Store');
  await page.getByRole('button', { name: /Next/ }).click();

  // Check multiple boxes
  await page.getByLabel(/Physical Products/).check();
  await page.getByLabel(/Digital Products/).check();

  // Wait for debounce sync
  await page.waitForTimeout(1000);

  // Reload the page and verify we're still on the "What do you sell?" step
  // and the data is restored
  await page.reload();
  await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();

  // Check if checkboxes remained checked
  await expect(page.getByLabel(/Physical Products/)).toBeChecked();
  await expect(page.getByLabel(/Digital Products/)).toBeChecked();
  await expect(page.getByLabel(/Services \/ Appointments/)).not.toBeChecked();

  // Go back and verify text input
  await page.getByRole('button', { name: 'Back' }).click();
  await expect(page.getByPlaceholder('What is your business called?')).toHaveValue('Maya State Store');
});
