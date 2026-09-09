import { expect, test } from '../../../../e2e/fixtures';

test.describe('Global Commerce live settings', () => {
  test('reads and persists tenant currency settings through the real API', async ({ page }) => {
    const response = await page.goto('/settings/global-commerce', { waitUntil: 'domcontentloaded' });
    expect(response?.status()).toBeLessThan(500);
    await expect(page.getByText('Enabled Currencies', { exact: true })).toBeVisible();

    const settingsResponse = await page.request.get('/api/v1/settings/global-commerce');
    expect(settingsResponse.status()).toBe(200);
    const settings = await settingsResponse.json();
    expect(settings.tenant.base_currency).toMatch(/^[A-Z]{3}$/);
    expect(settings.tenant.enabled_currencies).toContain(settings.tenant.base_currency);

    const baseCurrency = page.locator('select').first();
    await expect(baseCurrency).toHaveValue(settings.tenant.base_currency);
    page.on('dialog', (dialog) => dialog.accept());
    await page.getByRole('button', { name: 'Save Changes' }).click();
    await expect(page.getByRole('button', { name: 'Save Changes' })).toBeEnabled();

    const persistedResponse = await page.request.get('/api/v1/settings/global-commerce');
    expect(persistedResponse.status()).toBe(200);
    await expect(persistedResponse).toBeOK();
    const persisted = await persistedResponse.json();
    expect(persisted.tenant).toEqual(settings.tenant);
  });
});
