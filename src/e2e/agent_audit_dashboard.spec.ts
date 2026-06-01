import { test, expect } from './fixtures';

test.describe('Agent Audit Dashboard E2E', () => {
  test('navigates to dashboard and displays correct layout', async ({ page }) => {
    // Navigate from an authenticated path per rules. We use /dashboard as home.
    await page.goto('/dashboard');

    // Explicit wait to ensure the page has loaded enough to be interactive
    await page.waitForLoadState('domcontentloaded');

    // Click the visible nav link
    const link = page.locator('a[data-testid="agent-audit-link"]');
    await expect(link).toBeVisible({ timeout: 15000 });
    await link.click();

    // Verify URL change
    await page.waitForURL('**/agent-audit-dashboard**');

    // Test 1: Header visibility and Back link
    await expect(page.getByRole('heading', { name: 'Agent Audit Dashboard', exact: true })).toBeVisible();
    await expect(page.getByRole('link', { name: '< Back to Inbox' })).toBeVisible();

    // Test 2: Cost Tracker module
    await expect(page.getByRole('heading', { name: 'Cost Tracker' })).toBeVisible();
    await expect(page.getByText('$1,245.00')).toBeVisible();
    await expect(page.getByText('Total organizational spend')).toBeVisible();

    // Test 3: Operations module
    await expect(page.getByRole('heading', { name: 'Operations' })).toBeVisible();
    await expect(page.locator('text=Agent Health:').locator('..').getByText('Optimal')).toBeVisible();

    // Test 4: Marketing & Advertising module
    await expect(page.getByRole('heading', { name: 'Marketing & Advertising' })).toBeVisible();
    await expect(page.locator('text=Campaigns Sync:').locator('..').getByText('Active')).toBeVisible();

    // Test 5: Violation Feed module
    await expect(page.getByRole('heading', { name: 'Violation Feed' })).toBeVisible();
    await expect(page.getByText('Sandbox memory limit exceeded in Agent #452')).toBeVisible();
    await expect(page.getByText('Unauthorized network access attempt blocked')).toBeVisible();
  });
});
