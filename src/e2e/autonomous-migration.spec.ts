import { test, expect } from '@playwright/test';

test.describe('Store migration capability boundary', () => {
  test('shows unavailable migration without requesting or inventing an import', async ({ page }) => {
    const migrationRequests: string[] = [];
    page.on('request', request => {
      if (/migration|myshopify|\/import(?:[/?]|$)/i.test(request.url())) migrationRequests.push(request.url());
    });

    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Migrate Existing Store' }).click();
    await expect(page.getByText('Automatic store migration is not available yet. No import has been started.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Start Migration' })).toHaveCount(0);
    await expect(page.getByText('Migration Complete!')).toHaveCount(0);
    await expect(page.locator('input[name="migration_url"]')).toHaveCount(0);

    const catalog = page.getByRole('link', { name: 'Open product catalog' });
    await expect(catalog).toHaveAttribute('href', '/products');
    await catalog.click();
    await expect(page).toHaveURL(/\/products$/);
    expect(migrationRequests).toEqual([]);
    // A real external-store importer remains unimplemented. No product fixture
    // or timer may turn that missing capability into claimed migration success.
  });
});
