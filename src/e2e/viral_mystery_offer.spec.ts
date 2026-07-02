import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Viral Mystery Offer Generator', () => {
  test('dashboard links to Viral Mystery Offer Generator, which generates a viral link and soft paywall functions', async ({ page, adminUser, loginAs, context }) => {
    // 1. Log in
    await loginAs(page, adminUser);

    // 2. Navigate to dashboard
    await page.goto('/dashboard.html');
    let content = await page.content();
    if (!content.includes('OneHumanCorp')) {
        await page.goto('/tauri_out/dashboard.html');
        content = await page.content();
    }
    if (!content.includes('OneHumanCorp')) {
        await page.goto('/ui/dashboard.html');
        content = await page.content();
    }
    if (!content.includes('OneHumanCorp')) {
        await page.goto('/dashboard');
    }

    // 3. Click the Viral Mystery Offer Generator link
    await page.getByRole('link', { name: 'Viral Mystery Offer Generator 🎁' }).click();

    // Verify we are on the generator page
    await expect(page.getByRole('heading', { name: 'Viral Mystery Offer Generator 🎁' })).toBeVisible();

    // 4. Modify some config values
    const titleInput = page.locator('#offer-title');
    await titleInput.fill('Scratch for a Winter Deal!');

    const descInput = page.locator('#offer-desc');
    await descInput.fill('Enter your email to win big this winter!');

    const discountInput = page.locator('#offer-discount');
    await discountInput.fill('40% OFF');

    // 5. Verify the live preview updates
    await expect(page.locator('#preview-title')).toHaveText('Scratch for a Winter Deal!');
    await expect(page.locator('#preview-desc')).toHaveText('Enter your email to win big this winter!');
    await expect(page.locator('#preview-branding')).toHaveText('⚡ Powered by OHC');

    // 6. Verify the generated link contains the viral link and correct text
    const generateBtn = page.locator('#generate-btn');
    await generateBtn.click();

    const urlOutput = page.locator('#generated-url');
    const embedUrl = await urlOutput.inputValue();
    expect(embedUrl).toContain('Scratch%20for%20a%20Winter%20Deal!');
    expect(embedUrl).toContain('Enter%20your%20email%20to%20win%20big%20this%20winter!');
    expect(embedUrl).toContain('40%25%20OFF');
    expect(embedUrl).toContain('hideBranding=false');

    // 7. Test removing branding soft paywall
    // Ensure the user doesn't have pro
    await page.evaluate(() => { localStorage.setItem('has_pro', 'false'); window.dispatchEvent(new Event('storage')); });

    // Try to click "Remove branding"
    const toggleInput = page.locator('#branding-toggle-label');
    await toggleInput.click();

    // Verify soft paywall modal is shown
    const modal = page.locator('#paywall-modal');
    await expect(modal).toHaveClass(/active/);
    await expect(page.locator('#paywall-modal').getByText('Upgrade to Pro')).toBeVisible();

    // Dismiss modal
    await page.locator('#close-paywall').click();
    await expect(modal).not.toHaveClass(/active/);

    // 8. Test removing branding successfully as a Pro user
    await page.evaluate(() => { localStorage.setItem('has_pro', 'true'); window.dispatchEvent(new Event('storage')); });

    // The toggle should now work without modal
    await toggleInput.click();
    await expect(modal).not.toHaveClass(/active/);

    // Watermark should be gone in preview
    await expect(page.locator('#preview-branding')).not.toBeVisible();

    // 9. Re-generate link to confirm hideBranding=true
    await generateBtn.click();
    const newEmbedUrl = await urlOutput.inputValue();
    expect(newEmbedUrl).toContain('hideBranding=true');

    // 10. Open consumer page (Join link) in a new context
    const publicPage = await context.newPage();
    await publicPage.goto(newEmbedUrl);

    // Verify consumer UI
    await expect(publicPage.locator('#display-title')).toHaveText('Scratch for a Winter Deal!');
    await expect(publicPage.locator('#display-desc')).toHaveText('Enter your email to win big this winter!');
    await expect(publicPage.locator('#display-discount')).toHaveText('40% OFF');

    // Check that scratch cover is present and not revealed yet
    const scratchCover = publicPage.locator('#scratch-cover');
    await expect(scratchCover).toBeVisible();
    await expect(scratchCover).not.toHaveClass(/revealed/);

    // Verify "Powered by OHC" footer is NOT present since hideBranding=true
    let footerLink = publicPage.locator('#footer-branding');
    await expect(footerLink).not.toBeVisible();

    // 11. Consumer enters email to reveal
    const emailInput = publicPage.locator('#email-input');
    await emailInput.fill('consumer@example.com');

    const revealBtn = publicPage.locator('#reveal-btn');
    await expect(revealBtn).toHaveText('Reveal My Offer');
    await revealBtn.click();

    // 12. Verify the share section appears and deal unlocks
    const shareSection = publicPage.locator('#share-section');
    await expect(shareSection).toBeVisible({ timeout: 5000 });
    await expect(shareSection.locator('h3')).toHaveText('You got it! 🎉');

    // Verify scratch cover is now hidden
    await expect(scratchCover).toHaveClass(/revealed/);

    await publicPage.close();
  });
});
