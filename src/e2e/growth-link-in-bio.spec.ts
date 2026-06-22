import { test, expect } from './fixtures';

test.describe('Link in Bio Generator', () => {
  test('should allow creating and visiting a link in bio', async ({ page, adminUser }) => {
    // 1. Visit the dashboard and click the Link in Bio Generator card
    await page.goto('/dashboard');
    await page.getByRole('link', { name: /Link in Bio Generator/i }).click();
    await expect(page).toHaveURL(/\/link-in-bio-generator/);

    // 2. Configure the Link in Bio
    const storeNameInput = page.getByRole('textbox', { name: 'Store / Creator Name' });
    await storeNameInput.fill('My Awesome Creator Store');

    const bioInput = page.getByRole('textbox', { name: 'Bio / Description' });
    await bioInput.fill('This is a test bio description.');

    await page.getByRole('button', { name: 'Dark' }).click();

    // The first link is already there, let's just change it
    await page.getByPlaceholder('Link Title (e.g. Shop My Collection)').fill('My Custom Link');
    await page.getByPlaceholder('URL (e.g. https://...)').fill('https://example.com/shop');

    // 3. Save the config
    await page.getByRole('button', { name: 'Save & Publish' }).click();
    await expect(page.getByRole('button', { name: 'Saved! ✅' })).toBeVisible();

    // 4. Visit the public page and verify content
    await page.goto('/bio/e2e-tenant');

    await expect(page.getByRole('heading', { name: 'My Awesome Creator Store' })).toBeVisible();
    await expect(page.getByText('This is a test bio description.')).toBeVisible();

    const customLink = page.getByRole('link', { name: 'My Custom Link' });
    await expect(customLink).toBeVisible();
    await expect(customLink).toHaveAttribute('href', 'https://example.com/shop');

    // 5. Verify the viral loop footer
    const poweredBy = page.getByRole('link', { name: '⚡ Powered by OHC' });
    await expect(poweredBy).toBeVisible();
    await expect(poweredBy).toHaveAttribute('href', /\/onboarding\?ref=linkinbio_e2e-tenant/);
  });
});
