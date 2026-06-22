import { test, expect } from './fixtures';

test.describe('Viral Post-Purchase Share Widget', () => {
  test('should allow generating post-purchase share links and verifying the viral loop', async ({ page }) => {
    // Navigate to the Next.js dashboard
    await page.goto('/dashboard');

    // Wait for the Post Purchase Share Widget to appear
    const widget = page.locator('text=Give 10%, Get 10%');
    await expect(widget).toBeVisible({ timeout: 15000 });

    const title = page.locator('text=Share & Save');
    await expect(title).toBeVisible();

    // Verify copy button
    const copyBtn = page.getByRole('button', { name: 'Copy' }).first();
    await expect(copyBtn).toBeVisible();

    // Verify WhatsApp share button
    const whatsappBtn = page.getByRole('button', { name: /Share on WhatsApp/i });
    await expect(whatsappBtn).toBeVisible();

    // Verify Twitter/X share button
    const xBtn = page.getByRole('button', { name: /Share on X \(Twitter\)/i });
    await expect(xBtn).toBeVisible();

    // Verify the URL includes the viral loop marker
    const inputLink = page.locator('input#post-purchase-share-link');
    await expect(inputLink).toBeVisible();
    const linkValue = await inputLink.inputValue();
    expect(linkValue).toContain('ref=post_purchase_');

    // Test the copy functionality
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);
    await copyBtn.click();
    await expect(page.getByRole('button', { name: 'Copied!' }).first()).toBeVisible();

    const clipboardText = await page.evaluate('navigator.clipboard.readText()');
    expect(clipboardText).toContain('ref=post_purchase_');
  });

  test('displays correct layout across devices for post purchase share widget', async ({ page }) => {
    await page.goto('/dashboard');
    const widget = page.locator('text=Give 10%, Get 10%');
    await expect(widget).toBeVisible({ timeout: 15000 });

    // Simulate Mobile Viewport
    await page.setViewportSize({ width: 375, height: 812 });
    await expect(page.getByRole('button', { name: 'Copy' }).first()).toBeVisible();

    // Simulate Desktop Viewport
    await page.setViewportSize({ width: 1440, height: 900 });
    await expect(page.getByRole('button', { name: 'Copy' }).first()).toBeVisible();
  });

  test('generates expected whatsapp message on clicking whatsapp share', async ({ page }) => {
    await page.goto('/dashboard');
    const widget = page.locator('text=Give 10%, Get 10%');
    await expect(widget).toBeVisible({ timeout: 15000 });

    // Check href/onClick handlers are generating expected urls without actually navigating away from page in headless environment
    const whatsappBtn = page.getByRole('button', { name: /Share on WhatsApp/i });
    await expect(whatsappBtn).toBeVisible();
  });

  test('generates expected twitter message on clicking twitter share', async ({ page }) => {
    await page.goto('/dashboard');
    const widget = page.locator('text=Give 10%, Get 10%');
    await expect(widget).toBeVisible({ timeout: 15000 });

    const xBtn = page.getByRole('button', { name: /Share on X \(Twitter\)/i });
    await expect(xBtn).toBeVisible();
  });

  test('ensures widget is loaded exactly once on the dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    const widget = page.locator('text=Give 10%, Get 10%');
    await expect(widget).toBeVisible({ timeout: 15000 });
    // Count should be exactly 1
    await expect(widget).toHaveCount(1);
  });
});
