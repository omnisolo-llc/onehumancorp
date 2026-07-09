import { test, expect } from './fixtures';

test.describe('Viral Tier List Generator Loop', () => {
  test('Merchant uses Tier List Generator and sees soft paywall', async ({ page, request, loginAs, adminUser }) => {
    // Navigate and login
    await loginAs(page, adminUser);

    // Navigate to dashboard
    await page.goto('/dashboard');

    // Ensure pro is false
    await page.evaluate(() => {
        localStorage.setItem('has_pro', 'false');
    });

    // Find the link to tier list generator
    const tierListButton = page.locator('#tier-list-generator-link');
    await expect(tierListButton).toBeVisible();
    await tierListButton.click();

    // 1. Merchant navigates to tier list generator page
    await page.waitForURL('**/viral-tier-list-generator');

    // Check baseline: the page should be loaded
    const titleHeader = page.locator('h1', { hasText: 'Viral Tier List Generator' });
    await expect(titleHeader).toBeVisible();

    // Configure Widget
    await page.fill('input#title', 'Best E2E Test Tools');
    await page.fill('input#description', 'A definitive ranking of my favorites.');

    // Check preview updates
    await expect(page.locator('h1', { hasText: 'Best E2E Test Tools' }).nth(1)).toBeVisible();
    await expect(page.locator('p', { hasText: 'A definitive ranking of my favorites.' })).toBeVisible();

    // 2. Merchant tries to remove branding without Pro
    const removeBrandingCheckbox = page.locator('data-testid=branding-toggle');
    await removeBrandingCheckbox.click();

    // 3. Soft paywall appears
    const upgradeHeader = page.locator('h2', { hasText: 'Upgrade to Pro' });
    await expect(upgradeHeader).toBeVisible();

    // 4. Click Keep Branding
    const keepBrandingBtn = page.locator('button', { hasText: 'Keep Branding' });
    await keepBrandingBtn.click();

    // 5. Verify the soft paywall closes
    await expect(upgradeHeader).not.toBeVisible();

    // 6. Generate Link
    const generateBtn = page.locator('button', { hasText: 'Generate Share Link' });
    await generateBtn.click();

    // 7. Verify URL
    const generatedLinkInput = page.locator('data-testid=generated-link');
    await expect(generatedLinkInput).toBeVisible();

    const url = await generatedLinkInput.inputValue();
    expect(url).toContain('tier-list');
    expect(url).toContain('Best%20E2E%20Test%20Tools');
  });
});