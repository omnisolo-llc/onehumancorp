import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_giveaway smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'viral_giveaway'); });

test.describe('Viral Giveaway Loop', () => {
  test('should allow owner to create a giveaway and user to enter it', async ({ page, context }) => {
    // 1. Navigate to dashboard
    await page.goto('/dashboard');

    // 2. Find and click the Giveaway Generator link
    const giveawayLink = page.locator('a[href="/giveaway"]');
    await expect(giveawayLink).toBeVisible();
    await giveawayLink.click();

    // Verify page content
    await expect(page.getByRole('heading', { name: /Viral Giveaway Generator/i })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Giveaway Details' })).toBeVisible();

    // Wait to ensure client-side hydration doesn't interrupt filling
    await page.waitForTimeout(500);

    // 3. Fill out the giveaway configuration
    const titleInput = page.getByLabel('Prize / Title');
    await titleInput.fill('Win a Free iPad');
    await titleInput.pressSequentially('!');

    const descInput = page.getByLabel('Description');
    await descInput.fill('Enter your email to win an iPad. Share with friends for extra entries');
    await descInput.pressSequentially('!');

    // 4. Click generate link
    // We mock localStorage if needed, but fixtures set it.
    await page.evaluate(() => { localStorage.setItem('has_pro', 'true'); window.dispatchEvent(new Event('storage')); });

    const generateBtn = page.getByRole('button', { name: 'Generate Giveaway Link' });
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // 5. Capture the URL
    await expect(page.getByText('Link Ready!')).toBeVisible();
    const linkInput = page.locator('input[readonly]');
    const generatedUrl = await linkInput.inputValue();
    expect(generatedUrl).toContain('/giveaway/enter');
    expect(generatedUrl).toContain('Win%20a%20Free%20iPad!');

    // 6. Navigate to the generated public URL
    // Open a new page context to simulate a public user
    const publicPage = await context.newPage();
    await publicPage.goto(generatedUrl);

    // Verify the public entry page content
    await expect(publicPage.getByRole('heading', { name: 'Win a Free iPad!' })).toBeVisible();
    await expect(publicPage.getByText('Enter your email to win an iPad')).toBeVisible();

    // Verify "Powered by OHC" footer
    const footerLink = publicPage.getByRole('link', { name: '⚡ Powered by OHC' });
    await expect(footerLink).toBeVisible();
    const footerHref = await footerLink.getAttribute('href');
    expect(footerHref).toContain('/onboarding?ref=');

    // 7. Fill in an email and click enter
    const emailInput = publicPage.getByPlaceholder('Enter your email');
    await expect(emailInput).toBeVisible();
    await emailInput.fill('participant@example.com');

    const enterBtn = publicPage.getByRole('button', { name: 'Enter Giveaway' });
    await expect(enterBtn).toBeEnabled();
    await enterBtn.click();

    // 8. Verify the share prompt appears
    await expect(publicPage.getByRole('heading', { name: "You're Entered!" })).toBeVisible({ timeout: 5000 });
    await expect(publicPage.getByText('3 bonus entries')).toBeVisible();

    // Ensure share links are visible
    const shareLink = publicPage.locator('input[readonly]');
    await expect(shareLink).toBeVisible();
    const shareValue = await shareLink.inputValue();
    expect(shareValue).toContain('/giveaway/enter');

    await publicPage.close();
  });
});
