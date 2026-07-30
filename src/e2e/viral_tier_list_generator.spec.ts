import { test, expect } from './fixtures';

test.describe('Viral Tier List Generator Growth Loop', () => {
  test('dashboard links to Viral Tier List Generator, which generates an embed with a viral footer', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    await page.goto('/dashboard');

    const link = page.locator('#viral-tier-list-link');
    await expect(link).toBeVisible();
    await link.click();

    await page.waitForURL('**/viral-tier-list-generator.html');
    await expect(page.locator('h1', { hasText: 'Viral Tier List Generator' })).toBeVisible();

    await page.fill('#tier-title', 'Best OHC Agents');
    await page.fill('#tier-desc', 'Ranked by utility');

    await page.click('#add-tier-btn');
    await expect(page.locator('.tier-input-group')).toHaveCount(5);

    await page.click('#theme-dark');
    await expect(page.locator('#widget-preview')).toHaveClass(/dark/);
    await page.click('#theme-light');
    await expect(page.locator('#widget-preview')).not.toHaveClass(/dark/);

    await page.click('#get-code-btn');

    const embedCodeTextarea = page.locator('#embed-code');
    await expect(embedCodeTextarea).toBeVisible();
    const embedCode = await embedCodeTextarea.inputValue();

    expect(embedCode).toContain('<iframe src="');
    expect(embedCode).toContain('/api/v1/growth/viral-tier-list');
    expect(embedCode).toContain('title=Best%20OHC%20Agents');
    expect(embedCode).toContain('desc=Ranked%20by%20utility');

    await page.click('#close-embed-btn');

    await expect(page.locator('#preview-branding')).toBeVisible();

    await page.click('#remove-branding');

    const paywallModal = page.locator('#paywall-modal');
    await expect(paywallModal).toHaveClass(/active/);

    // Soft paywall UI is present. We will not click the X share button to avoid triggering
    // external navigation or network requests that might violate the E2E "no-substitution" contract.
  });
});
