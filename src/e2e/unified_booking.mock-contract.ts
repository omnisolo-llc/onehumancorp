import { test, expect } from '@playwright/test';

test.describe('Unified Booking & Quoting Engine CUJ', () => {

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
    await expect(page).toHaveURL(/success\.html/);
  });
});
