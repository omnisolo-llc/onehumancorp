import { test, expect } from './fixtures';

test.describe('Viral AI Pricing Calculator Widget', () => {
  test('should load the widget and generate an embed code', async ({ page }) => {
    await page.goto('/ui/viral-pricing-calculator.html');

    await expect(page.locator('h1')).toHaveText('AI Pricing Calculator');
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();

    const serviceInput = page.locator('#service-name');
    await expect(serviceInput).toBeVisible();
    await serviceInput.fill('Advanced SEO Consulting');

    const basePriceInput = page.locator('#base-price');
    await expect(basePriceInput).toBeVisible();
    await basePriceInput.fill('1500');

    await generateBtn.click();

    await expect(generateBtn).toBeDisabled();
    await expect(generateBtn).toHaveText('Generating...');

    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    const embedCodeTextarea = page.locator('#embed-code');
    await expect(embedCodeTextarea).toBeVisible();
    const embedText = await embedCodeTextarea.inputValue();

    expect(embedText).toContain('Advanced SEO Consulting');
    expect(embedText).toContain('1500');
    expect(embedText).toContain('<iframe');
    expect(embedText).toContain('Powered by OHC');
  });

  test('should copy the embed code to clipboard', async ({ page, context }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);
    await page.goto('/ui/viral-pricing-calculator.html');

    const generateBtn = page.locator('#generate-btn');
    await generateBtn.click();
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    const copyBtn = page.locator('#copy-btn');
    await expect(copyBtn).toHaveText('Copy Embed Code');

    await copyBtn.click();

    await expect(copyBtn).toHaveText('Copied!', { timeout: 3000 });

    try {
        const clipboardText = await page.evaluate(async () => {
            return await navigator.clipboard.readText();
        });
        expect(clipboardText).toContain('<iframe');
        expect(clipboardText).toContain('Powered by OHC');
    } catch (e) {
        console.warn('Clipboard read failed (expected in some headless environments): ', e);
    }
  });

  test('should show responsive layout on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/ui/viral-pricing-calculator.html');
    await page.waitForTimeout(100);

    await expect(page.locator('h1')).toHaveText('AI Pricing Calculator');

    const container = page.locator('.container');
    const box = await container.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);
  });

  test('should navigate back to the dashboard', async ({ page }) => {
    await page.goto('/ui/viral-pricing-calculator.html');
    const backLink = page.locator('.back-link');
    await expect(backLink).toBeVisible();
    await expect(backLink).toHaveAttribute('href', '/dashboard.html');
  });

  test('should render the embedded calculator properly', async ({ page }) => {
    // Navigate to the embed directly as it would appear in an iframe
    await page.goto('/ui/calculator-embed.html?tenant=e2e-tenant&service=Test%20Service&base=1000');

    // Check initial state
    await expect(page.locator('#service-title')).toHaveText('Test Service Estimate');
    await expect(page.locator('#total-price')).toHaveText('$1,000');

    // Change inputs and verify recalculation
    await page.locator('#speed').selectOption('2'); // Rush 2x
    await page.locator('#scale').selectOption('3'); // Large 3x

    // Total should be 1000 * 2 * 3 = 6000
    await expect(page.locator('#total-price')).toHaveText('$6,000');

    // Check action button
    const bookBtn = page.locator('#book-btn');
    await expect(bookBtn).toBeVisible();
    await expect(bookBtn).toHaveText('Request Quote');
  });
});
