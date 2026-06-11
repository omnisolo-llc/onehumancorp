import { test, expect } from './fixtures';

test.describe('Viral Tauri Booking Growth Loop', () => {
  test('customer booking flow shows Powered by OHC and referral widgets', async ({ page }) => {
    // 1. Customer visits booking intake form (using Tauri app URL structure)
    // We use the Next.js proxy to the Rust server to load the HTML files
    await page.goto('/api/ui/booking.html?tenant=e2e-tenant');
    await page.waitForLoadState('networkidle');

    const bookingFooterLink = page.locator('#ohc-viral-link');
    await expect(bookingFooterLink).toBeVisible();
    await expect(bookingFooterLink).toHaveAttribute('href', /source=booking_viral_loop/);
    await expect(bookingFooterLink).toContainText('Powered by OHC');

    // 2. Customer visits a drafted quote
    await page.goto('/api/ui/quote.html?mode=customer&tenant=e2e-tenant&id=test-quote-123');
    await page.waitForLoadState('networkidle');

    const quoteFooterLink = page.locator('#ohc-viral-link');
    await expect(quoteFooterLink).toBeVisible();
    await expect(quoteFooterLink).toHaveAttribute('href', /source=quote_viral_loop/);
    await expect(quoteFooterLink).toContainText('Powered by OHC');

    // 3. Customer sees the viral referral widget after payment success
    await page.goto('/api/ui/success.html?type=booking_deposit&tenant=e2e-tenant');
    await page.waitForLoadState('networkidle');

    await expect(page.locator('#success-title')).toContainText('Payment Successful!');
    await expect(page.getByRole('heading', { name: 'Give 20%, Get 10%' })).toBeVisible();

    const linkInput = page.locator('#viral-link-input');
    await expect(linkInput).toBeVisible();
    await expect(linkInput).toHaveValue(/source=success_viral_loop/);

    const copyBtn = page.locator('#btn-copy-viral');
    await copyBtn.click();
    await expect(copyBtn).toContainText('Copied!');

    const waBtn = page.locator('#btn-wa-viral');
    await expect(waBtn).toBeVisible();

    const successFooterLink = page.locator('#ohc-viral-link-footer');
    await expect(successFooterLink).toBeVisible();
    await expect(successFooterLink).toHaveAttribute('href', /source=success_viral_loop/);
    await expect(successFooterLink).toContainText('Powered by OHC');
  });
});
