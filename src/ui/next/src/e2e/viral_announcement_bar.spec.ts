import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_announcement_bar smoke', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_announcement_bar');
});

test.describe('Viral Announcement Bar Generator', () => {
  test('should load the widget, display live preview, and handle paywall', async ({ page }) => {
    await page.goto('/ui/viral-announcement-bar.html');

    // Verify title and page elements
    await expect(page).toHaveTitle(/Viral Announcement Bar Generator/);
    await expect(page.locator('h1')).toHaveText('Announcement Bar Generator');

    // Fill custom values
    await page.fill('#bar-text', 'New Feature Released!');
    await page.fill('#bar-link', 'https://example.com/feature');
    await page.selectOption('#bar-theme', 'blue');

    // Verify preview updates instantly
    await expect(page.locator('#preview-text')).toHaveText('New Feature Released!');
    const previewLink = page.locator('#preview-link');
    await expect(previewLink).toBeVisible();
    await expect(previewLink).toHaveAttribute('href', 'https://example.com/feature');

    // Check preview styling for the selected theme (blue is #0066FF)
    const previewBar = page.locator('#preview-bar');
    await expect(previewBar).toHaveCSS('background-color', 'rgb(0, 102, 255)');

    // Test branding soft paywall
    const removeBrandingCheckbox = page.locator('#remove-branding');

    // Clear pro status first
    await page.evaluate(() => { localStorage.setItem('has_pro', 'false'); window.dispatchEvent(new Event('storage')); });

    // Try to remove branding, modal should appear
    await removeBrandingCheckbox.locator('..').click();
    await expect(page.locator('#paywall-modal')).toHaveCSS('display', 'flex');

    // Keep branding
    await page.click('#close-modal-btn');
    await expect(page.locator('#paywall-modal')).not.toHaveCSS('display', 'flex');
    await expect(removeBrandingCheckbox).not.toBeChecked();

    // Verify branding is present in preview
    const previewPowered = page.locator('#preview-powered');
    await expect(previewPowered).toBeVisible();
    await expect(previewPowered).toHaveText('⚡ Powered by OHC');

    // Check the link URL contains the correct referral parameters
    const href = await previewPowered.getAttribute('href');
    expect(href).toContain('ref=e2e-tenant');
    expect(href).toContain('source=announcement_bar');

    // Upgrade to Pro in modal
    await removeBrandingCheckbox.locator('..').click();
    await page.click('#upgrade-btn');

    // Modal should disappear, toggle should be checked, and preview branding should hide
    await expect(page.locator('#paywall-modal')).not.toHaveCSS('display', 'flex');
    await expect(removeBrandingCheckbox).toBeChecked();
    await expect(previewPowered).not.toBeVisible();

    // Click Generate Embed Code
    const generateBtn = page.locator('#generate-btn');
    await generateBtn.click();

    // Result area should appear
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // Check embed code contents
    const embedCode = await page.inputValue('#embed-code');
    expect(embedCode).toContain('New Feature Released!');
    expect(embedCode).toContain('https://example.com/feature');
    expect(embedCode.toLowerCase()).toContain('#0066ff');
    expect(embedCode).not.toContain('⚡ Powered by OHC');
  });

  test('should generate embed code with branding if not pro', async ({ page }) => {
    await page.goto('/ui/viral-announcement-bar.html');

    // Click Generate Embed Code
    const generateBtn = page.locator('#generate-btn');
    await generateBtn.click();

    // Result area should appear
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // Check embed code contents
    const embedCode = await page.inputValue('#embed-code');
    expect(embedCode).toContain('⚡ Powered by OHC');
    expect(embedCode).toContain('ref=e2e-tenant');
  });

  test('should show responsive layout on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/ui/viral-announcement-bar.html');
    await page.waitForTimeout(100);

    await expect(page.locator('h1')).toHaveText('Announcement Bar Generator');

    const container = page.locator('.container');
    const box = await container.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);
  });
});
