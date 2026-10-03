import { test, expect } from '../../../../e2e/fixtures';

test.describe('Link-in-Bio Generator E2E', () => {
  test('reviews and publishes selected bio fields, serves them anonymously, then revokes that version', async ({ page, anonymousPage }) => {
    await page.goto('/link-in-bio-generator');
    const save = page.getByRole('button', { name: 'Save private configuration' });
    await expect(save).toBeEnabled();
    await page.getByRole('textbox', { name: 'Store / Creator Name Business name' }).fill('Playwright Test Bakery');
    await page.getByRole('textbox', { name: 'Bio / Description Bio tagline' }).fill('We bake the best E2E cakes!');
    if (await page.getByRole('textbox', { name: 'Link 1 Title' }).count() === 0) await page.getByRole('button', { name: '+ Add Link' }).click();
    await page.getByRole('textbox', { name: 'Link 1 Title' }).fill('Our Menu');
    await page.getByRole('textbox', { name: 'Link 1 URL' }).fill('https://example.com/menu');
    await save.click();
    await expect(page.getByRole('status', { name: 'Private profile status' })).toContainText('Saved private configuration');
    await expect(page.getByRole('link', { name: 'Open published website' })).toHaveCount(0);
    await page.getByRole('button', { name: 'Review public version' }).click();
    const snapshot = page.getByLabel('Public website snapshot');
    await expect(snapshot).toContainText('Playwright Test Bakery');
    await expect(snapshot).toContainText('Our Menu');
    await page.getByRole('button', { name: 'Publish reviewed version' }).click();
    const published = page.getByRole('link', { name: 'Open published website' });
    await expect.poll(async () => {
      if (await published.count()) return true;
      const refresh = page.getByRole('button', { name: 'Check publication status' });
      if (await refresh.isEnabled().catch(() => false)) await refresh.click();
      return await published.count() > 0;
    }, { timeout: 30_000, message: 'The mounted publication worker must commit a published receipt' }).toBe(true);
    const publicPath = await published.getAttribute('href');
    expect(publicPath).toMatch(/^\/api\/v1\/public\/sites\/[0-9a-f-]{36}$/);
    const response = await anonymousPage.goto(publicPath!);
    expect(response?.status()).toBe(200);
    await expect(anonymousPage.getByRole('heading', { name: 'Playwright Test Bakery' })).toBeVisible();
    await expect(anonymousPage.getByText('We bake the best E2E cakes!')).toBeVisible();
    await expect(anonymousPage.getByRole('link', { name: 'Our Menu' })).toHaveAttribute('href', 'https://example.com/menu');
    await page.getByRole('button', { name: 'Revoke publication version' }).click();
    await expect(page.getByText('This publication version is revoked.')).toBeVisible();
    await expect(page.getByRole('link', { name: 'Open published website' })).toHaveCount(0);
    expect((await anonymousPage.request.get(publicPath!)).status()).toBe(404);
  });
});
