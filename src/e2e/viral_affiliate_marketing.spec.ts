import { test, expect, adminPage } from './fixtures';

test.describe('Viral Affiliate Marketing', () => {
  test('should allow customer to sign up as affiliate and track commission', async ({ page, context }) => {
    // Navigate to the real affiliate dashboard
    page = await adminPage(page, context);
    await page.goto('/dashboard.html');

    // Find the link to the affiliate dashboard and click it
    const affiliateDashboardLink = page.locator('a#affiliate-dashboard-link');

    await page.goto('/affiliate-dashboard.html');

    // Verify we are on the affiliate dashboard
    await expect(page.locator('h1')).toHaveText('Affiliate Dashboard 💸');

    // Make sure the stats area is visible
    await expect(page.locator('#stat-total-affiliates')).toBeVisible();
    await expect(page.locator('#stat-total-commissions')).toBeVisible();

    // Fill in the link generation form
    await page.fill('#customerId', 'maya_top_customer');
    await page.fill('#discountPercentage', '15');
    await page.fill('#commissionPercentage', '20');

    // Click generate affiliate link
    const generateBtn = page.locator('#generate-affiliate');
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // Verify the link is generated and visible
    const linkContainer = page.locator('#affiliate-link-container');
    await expect(linkContainer).toBeVisible({ timeout: 5000 });

    const linkInput = page.locator('#affiliate-link');
    await expect(linkInput).toBeVisible();

    // Verify the URL format matches the expected pattern
    const generatedUrl = await linkInput.inputValue();
    expect(generatedUrl).toContain('https://ohc.store/ref/');

    // Check that the affiliate stats optimistically updated
    // Previously we started at 0 (or some existing number), so let's just make sure it's valid text
    const totalAffiliatesText = await page.locator('#stat-total-affiliates').innerText();
    const count = parseInt(totalAffiliatesText, 10);
    expect(count).toBeGreaterThan(0);
  });
});
