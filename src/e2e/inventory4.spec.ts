import { test, expect, loadSupplyPage } from './support/supply_fixture';

test('supply records remain readable and keyboard-refreshable at mobile and desktop widths', async ({ page, supplyOwner }) => {
  for (const width of [375, 1280]) {
    await page.setViewportSize({ width, height: 900 });
    await loadSupplyPage(page, supplyOwner);
    const region = page.getByRole('region', { name: 'Supply records', exact: true });
    for (const name of ['Raw Materials', 'Vendors']) await expect(region.getByRole('heading', { name, exact: true })).toBeVisible();
    const bounds = await region.boundingBox();
    expect(bounds).not.toBeNull();
    expect(bounds!.width).toBeLessThanOrEqual(width);
    expect(await region.evaluate(element => element.scrollWidth <= element.clientWidth + 1)).toBe(true);
    const reload = region.getByRole('button', { name: 'Reload supply records' });
    await reload.focus(); await expect(reload).toBeFocused();
    await reload.press('Enter');
    await expect(reload).toBeEnabled();
    await expect(region.getByText(supplyOwner.vendor.name, { exact: true })).toBeVisible();
    await page.goto('/dashboard');
    await expect(page.getByText('Low Stock', { exact: true }).locator('..')).toContainText('1');
  }
});
