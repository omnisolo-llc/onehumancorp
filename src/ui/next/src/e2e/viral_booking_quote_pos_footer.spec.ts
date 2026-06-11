import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Viral Footer Loop in Booking, Quote, and POS', () => {

  test('Booking request page contains a working "Powered by OHC" referral link', async ({ page }) => {
    // Navigate to the static booking page with a dummy tenant ID
    await page.goto('/booking.html?tenant=test-tenant-123');

    // Wait for the booking form to load
    await expect(page.locator('#booking-form')).toBeVisible();

    // Verify the "Powered by OHC" footer is visible
    const poweredByLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
    await expect(poweredByLink).toBeVisible();

    // Verify the link has the correct referral structure
    const href = await poweredByLink.getAttribute('href');
    expect(href).toContain('test-tenant-123');
    expect(href).toContain('source=booking_footer');
  });

  test('Quote page contains a working "Powered by OHC" referral link', async ({ page }) => {
    // Navigate to the static quote page with a dummy tenant ID
    await page.goto('/quote.html?id=1029&tenant=test-tenant-456&mode=customer');

    // Wait for the quote details to load
    await expect(page.locator('#quote-details-card')).toBeVisible();

    // Verify the "Powered by OHC" footer is visible
    const poweredByLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
    await expect(poweredByLink).toBeVisible();

    // Verify the link has the correct referral structure
    const href = await poweredByLink.getAttribute('href');
    expect(href).toContain('test-tenant-456');
    expect(href).toContain('source=quote_footer');
  });

  test('POS receipt page contains a working "Powered by OHC" referral link', async ({ page }) => {
    // Set a mock tenant ID in localStorage to test dynamic insertion
    await page.addInitScript(() => {
        window.localStorage.setItem('tenant_id', 'pos-test-tenant-789');
    });

    // Navigate to the static POS page
    await page.goto('/pos.html');

    // Wait for the POS view to load
    await expect(page.locator('#pos-view')).toBeVisible();

    // Simulate entering an amount and charging to get to the receipt screen
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '00', exact: true }).click();
    await page.locator('#charge-btn').click();

    // Simulate customer tap
    await page.locator('#simulate-tap-btn').click();

    // Wait for the receipt screen to be visible
    await expect(page.locator('#receipt-screen')).toBeVisible();

    // Verify the "Powered by OHC" footer is visible
    const poweredByLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
    await expect(poweredByLink).toBeVisible();

    // Verify the link has the correct referral structure based on the mocked localStorage
    const href = await poweredByLink.getAttribute('href');
    expect(href).toContain('pos-test-tenant-789');
    expect(href).toContain('source=pos_footer');
  });

});
