import { test, expect } from '@playwright/test';
import { memberPage as pageFixture } from './fixtures';
import { currentAppSmoke } from './smoke_helpers';

currentAppSmoke('viral_offer_widget');

test.describe('Viral Offer Embed Widget', () => {
  test('verify offer widget flow and viral branding', async ({ page }) => {
    // 1. Log in
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');

    // 2. Navigate to offer widget
    await page.goto('/offer-widget');
    await expect(page.locator('h1')).toContainText('Embeddable Offer Widget');

    // 3. Verify Viral Loop is visible in the actual iframe preview
    const iframe = page.locator('iframe[title="Offer Widget Preview"]');
    await expect(iframe).toBeVisible();
    const iframeFrame = iframe.contentFrame();
    await expect(iframeFrame.locator('text=⚡ Powered by OHC')).toBeVisible();

    // 4. Fill form
    await page.fill('input[value="Special Offer"]', 'Summer Sale');
    await page.fill('textarea', 'Get 50% off all cakes.');
    await page.fill('input[value="Get Offer"]', 'Claim Now');

    // Verify it updates in the iframe
    await expect(iframeFrame.locator('text=Summer Sale')).toBeVisible();
    await expect(iframeFrame.locator('text=Get 50% off all cakes.')).toBeVisible();
    await expect(iframeFrame.locator('text=Claim Now')).toBeVisible();

    // Verify remove branding behavior
    await page.getByLabel(/Remove "Powered by OHC" branding/).check();
    await expect(iframeFrame.locator('text=⚡ Powered by OHC')).toBeHidden();

    await page.getByLabel(/Remove "Powered by OHC" branding/).uncheck();
    await expect(iframeFrame.locator('text=⚡ Powered by OHC')).toBeVisible();

    // 5. Open Modal
    await page.click('button:has-text("Get Embed Code")');

    // 6. Verify Modal Contents
    await expect(page.locator('h2')).toContainText('Your Embed Code');

    // 7. Verify the Embed Code has the expected URL structure and parameters
    const codeValue = await page.inputValue('textarea');
    expect(codeValue).toContain('<iframe');
    expect(codeValue).toContain('Summer%20Sale');
    expect(codeValue).toContain('theme=light');
    expect(codeValue).toContain('/embed/offer?');
  });
});
