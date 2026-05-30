import { test, expect } from './fixtures';

test.describe('Integrations Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Connect Tools' }).first()).toBeVisible();
  });

  test('shows the custom software integration page', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Connect Tools' })).toBeVisible();
  });

  test('shows product data access copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Product Data Access' })).toBeVisible();
    await expect(page.getByText('Read Product List')).toBeVisible();
    await expect(page.getByText('Manage your custom software connections here.')).toBeVisible();
  });

  test('can return to dashboard from integrations', async ({ page }) => {
    await page.getByRole('button', { name: 'Back to Dashboard' }).click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('opens integrations from dashboard quick actions', async ({ page }) => {
    await page.getByRole('button', { name: 'Back to Dashboard' }).click();
    await page.getByRole('button', { name: 'Integrations' }).click();

    await expect(page.getByRole('heading', { name: /Social Media Accounts/ })).toBeVisible();
    await expect(page.locator('#manychat-integration').getByRole('button', { name: 'Configure' })).toBeVisible();
  });

  test('configure action routes to inbox', async ({ page }) => {
    await page.getByRole('button', { name: 'Back to Dashboard' }).click();
    await page.getByRole('button', { name: 'Integrations' }).click();
    await page.locator('#manychat-integration').getByRole('button', { name: 'Configure' }).click();

    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();
    await expect(page.getByText('Facebook User')).toBeVisible();
  });
});
