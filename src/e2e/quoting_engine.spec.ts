import { test, expect } from '@playwright/test';

test.describe('Dynamic Pricing & Instant Quotes Engine', () => {

  test.use({ viewport: { width: 375, height: 812 } }); // Mobile viewport requirement

  test('Customer requests a quote with instant dynamic pricing without network requests', async ({ page }) => {
    // 1. Visit the booking page
    await page.goto('/booking?service_id=cake-123&tenant=test-tenant');

    // Wait for initial network requests to complete (fetching rules)
    await page.waitForLoadState('networkidle');

    // Check base price
    const estimatedPriceLocator = page.getByTestId('estimated-price');
    await expect(estimatedPriceLocator).toContainText('$50.00');

    // Start tracking network requests to ensure pricing updates locally
    let networkRequestsMade = 0;
    page.on('request', (req) => {
      // Ignore background analytics/telemetry if any, but catch api calls
      if (req.url().includes('/api/v1/')) {
        networkRequestsMade++;
      }
    });

    // 2. Toggle options and verify instant pricing
    // We assume the default mock rules if API isn't live: Rush fee (Tomorrow) +20%, Vegan +$5.00, Delivery +$15.00

    // Toggle "Vegan" (Flat $5.00)
    await page.getByLabel('Vegan').check();
    await expect(estimatedPriceLocator).toContainText('$55.00');

    // Toggle "Delivery" (Flat $15.00)
    await page.getByLabel('Delivery').check();
    await expect(estimatedPriceLocator).toContainText('$70.00');

    // Toggle "Rush fee (Tomorrow)" (Percentage 20% of base $50 = $10)
    await page.getByLabel('Rush fee (Tomorrow)').check();
    await expect(estimatedPriceLocator).toContainText('$80.00');

    // Verify no new network requests were made during these interactions
    expect(networkRequestsMade).toBe(0);

    // 3. Submit request
    await page.fill('input[placeholder="First Last"]', 'Test Customer');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="date"]', '2025-10-10');
    await page.getByRole('button', { name: '10:30 AM' }).click();

    // Submit
    await page.getByRole('button', { name: 'Request Final Quote' }).click();

    // Verify success state
    await expect(page.getByText('Request Sent!')).toBeVisible();
  });

  test('Owner dashboard triage feed has Action Required quote card and can approve it', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Verify Glassmorphism card exists
    await expect(page.getByText('New Quote Request')).toBeVisible();
    await expect(page.getByText('From Maya (Custom Cake Order)')).toBeVisible();

    // Navigate to quoting approval page
    await page.getByRole('link', { name: 'Review & Approve' }).click();

    // Verify Quoting page
    await expect(page).toHaveURL(/\/quoting\?id=test-quote-123/);
    await expect(page.getByText('Review Draft Quote')).toBeVisible();
  });

});
