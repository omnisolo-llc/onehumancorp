import { test, expect } from './fixtures';

test.describe('Integrations Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.getByRole('button', { name: 'Connect Custom Software' }).first().click();
    await expect(page.getByRole('heading', { name: 'Connect Custom Software' }).first()).toBeVisible();
  });

  test('shows the custom software integration page', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Custom Integration' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Custom Software', exact: true })).toBeVisible();
  });

  test('shows product data access copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Product Data Access' })).toBeVisible();
    await expect(page.getByText('Read Product List')).toBeVisible();
    await expect(page.getByText('Manage your custom software connections here.')).toBeVisible();
  });

  test('can return to dashboard from integrations', async ({ page }) => {
    await page.getByRole('button', { name: 'Back to Dashboard' }).first().click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('opens integrations from dashboard quick actions', async ({ page }) => {
    await page.getByRole('button', { name: 'Back to Dashboard' }).first().click();
    await page.getByRole('button', { name: 'Connect Custom Software' }).first().click();

    await expect(page.getByRole('heading', { name: /Facebook/ })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Configure' })).toBeVisible();
  });

  test('configure action routes to inbox', async ({ page }) => {
    await page.getByRole('button', { name: 'Back to Dashboard' }).first().click();
    await page.getByRole('button', { name: 'Connect Custom Software' }).first().click();
    await page.getByRole('button', { name: 'Configure' }).first().click();

    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();
    await expect(page.getByText('Facebook User')).toBeVisible();
  });

  test('shows Brevo integration option', async ({ page }) => {
    await page.getByRole('button', { name: 'Back to Dashboard' }).first().click();
    await page.getByRole('button', { name: 'Connect Custom Software' }).first().click();
    await expect(page.getByText('Brevo Conversations')).toBeVisible();
  });

  test('shows Cal.com integration option', async ({ page }) => {
    await page.getByRole('button', { name: 'Back to Dashboard' }).first().click();
    await page.getByRole('button', { name: 'Connect Custom Software' }).first().click();
    await expect(page.getByText('Cal.com Scheduling')).toBeVisible();
  });

  test('shows Mailchimp integration option', async ({ page }) => {
    await page.getByRole('button', { name: 'Back to Dashboard' }).first().click();
    await page.getByRole('button', { name: 'Connect Custom Software' }).first().click();
    await expect(page.getByText('Mailchimp Marketing')).toBeVisible();
  });

  test('shows Shippo integration option', async ({ page }) => {
    await page.getByRole('button', { name: 'Back to Dashboard' }).first().click();
    await page.getByRole('button', { name: 'Connect Custom Software' }).first().click();
    await expect(page.getByText('Shippo Shipping')).toBeVisible();
  });

  test('shows Daily.co integration option', async ({ page }) => {
    await page.getByRole('button', { name: 'Back to Dashboard' }).first().click();
    await page.getByRole('button', { name: 'Connect Custom Software' }).first().click();
    await expect(page.getByText('Daily.co Video')).toBeVisible();
  });
});
