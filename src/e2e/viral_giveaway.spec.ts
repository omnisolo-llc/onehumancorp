import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';



test.describe('Viral Giveaway Loop', () => {
  test('should allow owner to create a giveaway and user to enter it', async ({ page, context }) => {
    // 1. Navigate to dashboard
    await page.goto('/dashboard');

    // 2. Find and click the Giveaway Generator link
    const giveawayLink = page.locator('a[href="giveaway/index.html"]');
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

    const generatorFooterLink = page.locator('a', { hasText: '⚡ Powered by OHC' }).first();
    await expect(generatorFooterLink).toBeVisible();

    const generateBtn = page.getByRole('button', { name: 'Generate Giveaway Link' });
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // 5. Capture the URL
    await expect(page.getByText('Your Viral Link is Ready!')).toBeVisible();    const linkInput = page.locator('input[readonly]');
    const generatedUrl = await linkInput.inputValue();
    expect(generatedUrl).toContain('/giveaway/enter');
    expect(generatedUrl).toContain('Win+a+Free+iPad%21');

    // 6. Navigate to the generated public URL
    // Open a new page context to simulate a public user
    const publicPage = await context.newPage();
    await publicPage.goto(generatedUrl);

    // Verify the public entry page content
    await expect(publicPage.getByRole('heading', { name: 'Win a Free iPad!' })).toBeVisible();
    await expect(publicPage.getByText('Enter your email to win an iPad')).toBeVisible();

    // Verify "Powered by OHC" footer
    const footerLink = publicPage.locator('a', { hasText: '⚡ Powered by OHC' }).first();
    await expect(footerLink).toBeVisible();
    const footerHref = await footerLink.getAttribute('href');
    expect(footerHref).toContain('/api/v1/growth/referrals/click');

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

  test('should show soft paywall when attempting to remove branding without pro', async ({ page }) => {
    await page.goto('/giveaway');
    await page.evaluate(() => {
        localStorage.setItem('tenant', 'e2e-test-store');
        localStorage.setItem('has_pro', 'false');
    });
    await page.reload();


    await page.locator('.toggle-switch').click({ force: true });
    // Soft paywall should appear
    await expect(page.locator('text=Pro Feature')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Upgrade to Pro' }).first()).toBeVisible();
  });

  test('should hide branding when pro is enabled and toggle is clicked', async ({ page, context }) => {
    await page.goto('/giveaway');
    await page.evaluate(() => {
        localStorage.setItem('tenant', 'e2e-test-store');
        localStorage.setItem('has_pro', 'true');
    });
    await page.reload();

    // 3. Fill out the giveaway configuration
    const titleInput = page.getByLabel('Prize / Title');
    await titleInput.fill('Win a Free iPad');


    await page.locator('.toggle-switch').click({ force: true });
    // Soft paywall should not appear
    await expect(page.locator('text=Pro Feature')).not.toBeVisible();

    // Preview section should hide the branding
    await expect(page.locator('a', { hasText: '⚡ Powered by OHC' })).not.toBeVisible();

    const generateBtn = page.getByRole('button', { name: 'Generate Giveaway Link' });
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // 5. Capture the URL
    await expect(page.getByText('Your Viral Link is Ready!')).toBeVisible();    const linkInput = page.locator('input[readonly]');
    const generatedUrl = await linkInput.inputValue();
    expect(generatedUrl).toContain('branding=false');

    // Navigate to the generated public URL
    const publicPage = await context.newPage();
    await publicPage.goto(generatedUrl);

    // Verify "Powered by OHC" footer is not present
    await expect(publicPage.locator('a', { hasText: '⚡ Powered by OHC' })).not.toBeVisible();

    await publicPage.close();
  });
  test('should dismiss soft paywall when Maybe Later is clicked', async ({ page }) => {
    await page.goto('/giveaway');
    await page.evaluate(() => {
        localStorage.setItem('tenant', 'e2e-test-store');
        localStorage.setItem('has_pro', 'false');
    });
    await page.reload();

    await page.locator('.toggle-switch').click({ force: true });

    await expect(page.locator('text=Pro Feature')).toBeVisible();
    await page.getByRole('button', { name: 'Maybe Later' }).click();
    await expect(page.locator('text=Pro Feature')).not.toBeVisible();
  });

  test('should hide footer when branding=false is in the url', async ({ page }) => {
    await page.goto('/giveaway/enter?branding=false');

    // Verify "Powered by OHC" footer is not present
    await expect(page.locator('a', { hasText: '⚡ Powered by OHC' })).not.toBeVisible();
  });
});
