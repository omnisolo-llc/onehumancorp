import { test, expect } from './fixtures';

test.describe('Manychat Integration', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/integrations');
  });

  test('can see the Manychat connect option heading', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Manychat' })).toBeVisible();
  });

  test('can see the Manychat connect description text', async ({ page }) => {
    await expect(page.getByText('Unified Customer Inbox. Manage all your messages and posts from one place.').first()).toBeVisible();
  });

  test('can see the Connect Instagram button', async ({ page }) => {
    const connectButton = page.getByRole('button', { name: 'Connect Instagram' });
    await expect(connectButton).toBeVisible();
  });

  test('can click the Connect Instagram button', async ({ page }) => {
    const connectButton = page.getByRole('button', { name: 'Connect Instagram' });
    await connectButton.click();
  });

  test('can view the Customer Inbox after connection', async ({ page }) => {
    const connectButton = page.getByRole('button', { name: 'Connect Instagram' });
    await connectButton.click();
    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();
  });
});
