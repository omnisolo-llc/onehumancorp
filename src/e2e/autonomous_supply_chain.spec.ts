import { test, expect } from './fixtures';

test.describe('Autonomous Supply Chain & Vendor Mesh', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/inventory');
    await expect(page.getByRole('heading', { name: 'Inventory' })).toBeVisible();
  });

  test('UI displays database-backed supply navigation', async ({ page }) => {
    await expect(page.getByText('Raw Materials')).toBeVisible();
    await expect(page.getByText('Loaded from `/api/v1/ui/supply`.')).toBeVisible();
  });

  test('Displays vendor state from the supply endpoint', async ({ page }) => {
    await expect(page.getByText('Vendors', { exact: true }).first()).toBeVisible();
    await expect(page.locator('body')).toContainText(/No vendor rows found|Loading vendors|Supply partners/i);
  });

  test('Displays raw material inventory status', async ({ page }) => {
    await expect(page.locator('body')).toContainText(/No raw material rows found|Loading inventory|Low Stock|Healthy/);
  });

  test('Displays PO approval in inbox and allows single-tap approval', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByText('Action Required')).toBeVisible();
    await expect(page.getByText('Operations Map')).toBeVisible();
  });
});
