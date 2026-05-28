import { test, expect } from './fixtures';

test.describe('Integrations Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible();
  });

  test('shows the tool integration page', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Tool Integrations' })).toBeVisible();
  });

  test('shows ayrshare integration copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Ayrshare' })).toBeVisible();
    await expect(page.getByText('Unified API for posting and retrieving messages across social networks.')).toBeVisible();
  });

  test('can return to dashboard from integrations', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('h1').filter({ hasText: 'Dashboard' })).toBeVisible();
  });
});
