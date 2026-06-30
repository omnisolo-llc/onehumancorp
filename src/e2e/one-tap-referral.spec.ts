import { test, expect } from '@playwright/test';
import { e2ePage } from './fixtures';

test.describe('One-Tap Referral Generator', () => {
  test('should generate widget and enforce paywall', async ({ page }) => {
    // 1. Setup session and navigate to the One-Tap Referral Generator UI
    await e2ePage.setupSession(page);
    await page.goto('/ui/one-tap-referral.html');

    // Verify page loaded
    await expect(page.locator('h1')).toContainText('One-Tap Referral Generator 🎁');

    // 2. Configure Widget
    await page.fill('#reward-text', 'Give $20, Get $20');
    await page.fill('#desc-text', "Special offer for your friends!");

    // Check live preview updates
    await expect(page.locator('#preview-title')).toContainText('Give $20, Get $20');
    await expect(page.locator('#preview-desc')).toContainText('Special offer for your friends!');

    // 3. Test Paywall
    const removeBrandingCheckbox = page.locator('#remove-branding');
    await removeBrandingCheckbox.check();

    // Verify paywall modal appears since this is a basic test tenant
    const paywallModal = page.locator('#paywall-modal');
    await expect(paywallModal).toHaveClass(/active/);
    await expect(paywallModal.locator('h2')).toContainText('Upgrade to Pro');

    // Close paywall
    await page.locator('#close-paywall').click();
    await expect(paywallModal).not.toHaveClass(/active/);

    // 4. Generate Embed Code
    await page.locator('#generate-btn').click();

    // Verify embed modal
    const embedModal = page.locator('#embed-modal');
    await expect(embedModal).toHaveClass(/active/);

    const embedCode = await page.locator('#embed-code').inputValue();
    expect(embedCode).toContain('api/v1/growth/one-tap-referral/embed');
    expect(embedCode).toContain('Give%20%2420%2C%20Get%20%2420'); // reward
    expect(embedCode).toContain('hide_branding=false'); // branding should be false since paywall blocked it

    // 5. Check actual embed route directly to ensure it works
    const embedUrl = embedCode.match(/src="([^"]+)"/)?.[1];
    expect(embedUrl).toBeDefined();

    if (embedUrl) {
      // The origin may be different in E2E tests, so use a relative path
      const url = new URL(embedUrl);
      const relativePath = url.pathname + url.search;

      const response = await page.request.get(relativePath);
      expect(response.ok()).toBeTruthy();

      const html = await response.text();
      expect(html).toContain('Give $20, Get $20');
      expect(html).toContain('Special offer for your friends!');
      expect(html).toContain('⚡ Powered by OHC'); // Should contain branding
    }
  });
});
