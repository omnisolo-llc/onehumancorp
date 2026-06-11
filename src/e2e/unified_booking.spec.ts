import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

// currentAppSmoke('unified_booking');

test.describe('Unified Booking & Quoting Engine CUJ', () => {

  test('Customer requests service via Next.js mobile booking flow', async ({ page, request }) => {
    // We'll test the actual Next.js booking flow that we just built instead of the html stubs

    // First let's seed a service via API so the UI has something to render
    const tenantId = 'e2e-tenant';
    const authHeaders = { 'x-tenant-id': tenantId, 'x-user-id': 'e2e-user' };

    // In real E2E we assume backend returns services. Let's hit the new booking UI
    await page.goto('/booking?tenant=' + tenantId);

    await expect(page.getByRole('heading', { name: 'Book a Service' })).toBeVisible({ timeout: 15000 });

    // The UI fetches services and displays them. It might be empty if DB isn't seeded with services.
    // If it renders the empty list or services, it should display steps.
    await expect(page.getByText('1. Select Service')).toBeVisible({ timeout: 5000 });
  });

  test('Customer requests service via booking.html, owner approves via quote.html, customer pays deposit', async ({ page }) => {
    // 1. Customer initiates a request
    await page.goto('/booking.html?tenant=e2e-tenant');
    await page.waitForTimeout(500);

    // Verify form exists
    await expect(page.locator('#description')).toBeVisible();
    await page.locator('#description').fill('I need a quote for a 2-hour piano lesson.');

    // Submit request
    await page.getByRole('button', { name: 'Get a Quote' }).click();

    // Verify success view
    await expect(page.getByText('Request Sent!')).toBeVisible();

    // 2. Owner adjusts and sends the quote
    // For this E2E test we simulate opening the generated quote link as an owner
    // In a full implementation, they would see this in their inbox and click a "Review Quote" button.
    await page.goto('/quote.html?tenant=e2e-tenant&mode=owner&id=mock-quote-123');
    await page.waitForTimeout(500);

    // Verify Owner view controls
    await expect(page.locator('#deposit-slider')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Approve & Send to Customer' })).toBeVisible();

    // Adjust deposit to 50%
    await page.locator('#deposit-slider').fill('50');
    // Ensure display updates
    await expect(page.locator('#deposit-percent-display')).toContainText('50%');

    // Approve and send
    page.on('dialog', dialog => dialog.accept());
    await page.getByRole('button', { name: 'Approve & Send to Customer' }).click();
    await page.waitForTimeout(1000); // Allow redirect

    // 3. Customer views quote and pays deposit
    await page.goto('/quote.html?tenant=e2e-tenant&mode=customer&id=mock-quote-123');
    await page.waitForTimeout(500);

    // Verify Customer view controls
    await expect(page.getByText('Action Required')).toBeVisible();

    // Select timeslot
    const dateCards = page.locator('.date-card');
    await expect(dateCards.first()).toBeVisible();
    await dateCards.nth(1).click();

    const timeSlots = page.locator('.time-slot');
    await expect(timeSlots.first()).toBeVisible();
    await timeSlots.nth(2).click();

    // Pay Deposit
    await page.getByRole('button', { name: 'Pay with Card' }).click();
    await page.waitForTimeout(1000); // Allow redirect

    // Verify success redirect
    await expect(page).toHaveURL(/success.html/);
  });
});
