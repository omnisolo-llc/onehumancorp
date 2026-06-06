import { expect, test } from './fixtures';

test.describe('Brand Studio workflow', () => {
  test('creates a brand toolbox and publishes a website from it', async ({ page }) => {
    const confusingCompetitorName = new RegExp(['po', 'melli'].join(''), 'i');

    await page.goto('/brand-studio');

    await expect(page.getByRole('heading', { name: 'Create Brand Toolbox' })).toBeVisible();
    await expect(page.locator('body')).not.toContainText(confusingCompetitorName);

    await page.locator('#brand-toolbox-description').fill(
      'Luna Loaf is a local bakery selling custom cakes, weekend dessert boxes, and warm pickup experiences for families.',
    );
    await page.locator('#brand-toolbox-website').fill('https://luna-loaf.example');
    await page.locator('#brand-toolbox-product').fill('https://luna-loaf.example/weekend-box');
    await page.locator('#brand-toolbox-campaign').fill('launch the summer dessert box');

    await page.getByRole('button', { name: 'Generate Toolbox' }).click();
    await expect(page.getByText('Brand DNA').first()).toBeVisible({
      timeout: 30_000,
    });

    await expect(page.getByText('Brand DNA').first()).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Brand Book', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Logo Concepts' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter Catalog' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Campaign Ideas' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Social Calendar' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Creative Assets' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Photoshoot' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Website Draft' })).toBeVisible();
    await expect(page.locator('svg').first()).toBeVisible();

    await page.getByRole('button', { name: 'Publish Website' }).click();
    await expect(page.getByText(/Published domain: .*\.ohc\.store/)).toBeVisible({
      timeout: 30_000,
    });
    await expect(page.locator('body')).not.toContainText(confusingCompetitorName);
  });
});
