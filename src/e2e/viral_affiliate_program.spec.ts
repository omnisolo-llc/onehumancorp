import { test, expect } from './fixtures';

test.describe('Viral Affiliate Program Loop', () => {
  test('should allow owner to view affiliate stats and generate links', async ({ page, loginAs, unlimitedAdminUser }) => {
    // 1. Navigate to dashboard
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // 2. Find and click the Affiliate Partner Program link
    const affiliateLink = page.locator('a[href="affiliate.html"]');
    await expect(affiliateLink).toBeVisible();
    await affiliateLink.click();

    // Verify page content
    await expect(page.getByRole('heading', { name: /Affiliate Partner Program/i })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Program Statistics' })).toBeVisible();

    // Verify stats are loaded correctly (e2e DB starts with 0 for this unless seeded differently)
    const statAffiliates = page.locator('#stat-affiliates');
    await expect(statAffiliates).toBeVisible();

    // 3. Generate a new affiliate link
    const customerIdInput = page.locator('#customer-id');
    await customerIdInput.fill('E2E_Test_Customer');

    const generateBtn = page.getByRole('button', { name: 'Generate Affiliate Link' });
    await expect(generateBtn).toBeEnabled();

    await generateBtn.click();

    // 4. Verify link is generated
    await expect(page.getByText('Your Link is Ready!')).toBeVisible({ timeout: 5000 });

    const generatedLinkInput = page.locator('#affiliate-link');
    await expect(generatedLinkInput).toBeVisible();

    const generatedUrl = await generatedLinkInput.inputValue();
    expect(generatedUrl).toContain('https://ohc.store/ref/');

    // Verify stats incremented locally
    await expect(statAffiliates).not.toHaveText('0');

    // 5. Verify Action buttons exist
    await expect(page.locator('#copy-btn')).toBeVisible();
    await expect(page.locator('#share-x-btn')).toBeVisible();
    await expect(page.locator('#share-wa-btn')).toBeVisible();
  });
});
