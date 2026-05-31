import { test, expect } from '@playwright/test';

test.describe('Embedded Capital Engine', () => {
  test('Maya (Baker) can view and accept a pre-approved capital offer in her dashboard', async ({ page, request }) => {
    // 1. Setup mock data: Maya's tenant id is 'tenant_1' based on fixtures
    const tenantId = 'tenant_1';

    // Simulate a large booking to trigger the finance agent via our API (or just inject an offer directly into DB)
    // For the E2E, we'll intercept the frontend's call to /api/v1/capital/offers/my-store and return a mock offer
    await page.route(`**/api/v1/capital/offers/${tenantId}`, async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([{
          id: 'offer-123',
          tenant_id: tenantId,
          merchant_id: tenantId,
          amount: 600.0,
          flat_fee: 45.0,
          repayment_percentage: 0.10,
          status: 'active'
        }])
      });
    });

    // Mock the accept endpoint
    await page.route(`**/api/v1/capital/offers/offer-123/accept`, async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          advance: {
            id: 'adv-456',
            offer_id: 'offer-123',
            total_owed: 645.0,
            total_repaid: 0.0,
            status: 'active'
          }
        })
      });
    });

    // Login logic (using simple local storage injection if possible, or navigate via standard flow)
    await page.goto('/login');
    await page.fill('input[type="email"]', 'maya@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');

    // Make sure we set the tenant id in local storage (some tests might do this implicitly)
    await page.evaluate((tid) => {
      localStorage.setItem('tenant', tid);
    }, tenantId);

    // 2. Navigate to Dashboard
    await page.goto('/dashboard');

    // 3. Verify the "Smart Offer" card appears natively below the order details / in dashboard
    await expect(page.locator('text=Smart Capital Offer')).toBeVisible();
    await expect(page.locator('text=Pre-approved')).toBeVisible();
    // Accept $600.00
    await expect(page.locator('button', { hasText: 'Accept $600.00' })).toBeVisible();

    // 4. Maya taps "Accept $600.00"
    await page.click('button:hasText("Accept $600.00")');

    // 5. Verify immediate availability (celebratory green toast/message appears)
    await expect(page.locator('text=Funds added to your OHC Wallet.')).toBeVisible();
  });
});
