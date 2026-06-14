import { test, expect } from './fixtures';

test.describe('Viral Flash Sale Generator Loop', () => {
  test('should allow owner to create a flash sale widget, copy embed code, and load the public widget', async ({ page, context, adminUser, loginAs }) => {
    // 1. Navigate to dashboard
    await loginAs(page, adminUser);
    await page.goto('/dashboard');

    // 2. Find and click the Flash Sale Generator link
    const flashSaleLink = page.locator('a[href="flash-sale-generator.html"]');
    await expect(flashSaleLink).toBeVisible();
    await flashSaleLink.click();

    // Verify page content
    await expect(page.getByRole('heading', { name: /Flash Sale Generator/i })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Widget Settings' })).toBeVisible();

    // Wait to ensure client-side hydration doesn't interrupt filling
    await page.waitForTimeout(500);

    // 3. Fill out the flash sale configuration
    const titleInput = page.locator('#saleTitle');
    await titleInput.fill('Midweek Flash Sale');

    const codeInput = page.locator('#discountCode');
    await codeInput.fill('MIDWEEK50');

    const percentInput = page.locator('#discountPercent');
    await percentInput.fill('50');

    // 4. Click generate embed code
    const generateBtn = page.getByRole('button', { name: 'Generate Embed Code' });
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // 5. Capture the embed code
    await expect(page.getByRole('heading', { name: 'Embed Flash Sale' })).toBeVisible();
    const linkInput = page.locator('textarea[readonly]');
    await expect(linkInput).toBeVisible();
    const generatedHtml = await linkInput.inputValue();

    // Check if the clipboard content is a valid iframe with the viral link
    expect(generatedHtml).toContain('<iframe');
    expect(generatedHtml).toContain('src="http'); // It should be an absolute URL
    expect(generatedHtml).toContain('/api/v1/growth/flash-sale/embed');
    expect(generatedHtml).toContain('⚡ Powered by OHC');
    expect(generatedHtml).toContain('/api/v1/growth/referrals/click?target=/onboarding&ref=');
    expect(generatedHtml).toContain('MIDWEEK50');
    expect(generatedHtml).toContain('Midweek%20Flash%20Sale');
    expect(generatedHtml).toContain('50');

    // 6. Test the Public Embed Route directly
    const iframeSrcMatch = generatedHtml.match(/src="([^"]+)"/);
    expect(iframeSrcMatch).not.toBeNull();
    if (!iframeSrcMatch) throw new Error("Could not parse iframe src");
    const iframeSrc = iframeSrcMatch[1];

    const publicPage = await context.newPage();
    await publicPage.goto(iframeSrc);

    await expect(publicPage.locator('text=Midweek Flash Sale')).toBeVisible();
    await expect(publicPage.locator('text=MIDWEEK50')).toBeVisible();
    await expect(publicPage.locator('text=50% OFF')).toBeVisible();
    await expect(publicPage.locator('text=Powered by OHC')).toBeVisible();

    // Click the referral link in the embed and verify
    const referralLink = publicPage.locator('a:has-text("Powered by OHC")');
    await expect(referralLink).toBeVisible();
    // It should point to the /api/v1/growth/referrals/click route
    const href = await referralLink.getAttribute('href');
    expect(href).toContain('/api/v1/growth/referrals/click?target=/onboarding&ref=');

    await publicPage.close();

    // Close the modal
    const closeBtn = page.getByRole('button', { name: 'Close' });
    await closeBtn.click();
    await expect(page.getByRole('heading', { name: 'Embed Flash Sale' })).not.toBeVisible();
  });
});
