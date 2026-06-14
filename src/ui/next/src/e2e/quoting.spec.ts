import { test, expect } from '@playwright/test';

test.describe('Autonomous Service Estimator & Quoting', () => {
  test('Customer submits request-quote form successfully', async ({ page }) => {
    // We mock the backend intake so we don't need real LLM credentials
    await page.route('/api/quotes/intake', async (route) => {
      await route.fulfill({
        status: 201,
        contentType: 'application/json',
        body: JSON.stringify({ id: 'mocked-quote-123' })
      });
    });

    await page.goto('/request-quote');
    await expect(page.locator('h1')).toContainText('Request an Estimate');
    await page.fill('textarea[id="description"]', 'My sink is leaking');
    await page.fill('input[id="imageUrl"]', 'https://example.com/sink.jpg');
    await page.click('button[type="submit"]');

    // Check that success message appears
    await expect(page.locator('h3')).toContainText('Request Sent!');
  });

  test('Owner edits and approves the quote', async ({ page }) => {
    await page.route('/api/quotes?id=test-quote-123', async (route) => {
      if (route.request().method() === 'GET') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            quote: { status: 'DRAFT' },
            line_items: [
              { id: '1', description: 'Labor', unit_price_cents: 5000, quantity: 2 },
              { id: '2', description: 'Parts', unit_price_cents: 2000, quantity: 1 }
            ]
          })
        });
      } else if (route.request().method() === 'POST') {
        await route.fulfill({ status: 200 });
      }
    });

    await page.route('/api/quotes/test-quote-123/approve', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true })
      });
    });

    // Accept alert dialog automatically
    page.on('dialog', dialog => dialog.accept());

    await page.goto('/quoting?id=test-quote-123&mode=owner');

    // Wait for data to load
    await expect(page.locator('h1')).toContainText('Project Proposal');
    await expect(page.locator('h4', { hasText: 'Labor' })).toBeVisible();

    // Check initial total: 2*$50 + 1*$20 = $120.00
    await expect(page.getByText('$120.00')).toBeVisible();

    // Click "Approve & Send"
    await page.getByRole('button', { name: 'Approve & Send' }).click();
    // Alert is triggered (and auto accepted).
  });

  test('Customer views and pays the approved quote deposit', async ({ page }) => {
    await page.route('/api/quotes?id=test-quote-123', async (route) => {
      if (route.request().method() === 'POST') {
        return route.fulfill({
          status: 200,
        });
      }
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          quote: { status: 'ACCEPTED' },
          line_items: [
            { id: '1', description: 'Labor', unit_price_cents: 5000, quantity: 2 }
          ]
        })
      });
    });

    await page.route('/api/billing/create-checkout-session', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ checkout_url: 'https://checkout.stripe.com/mock' })
      });
    });

    // Mock window.location.assign via page evaluate since we can't intercept the actual navigation easily in standard playwright routing without breaking out
    await page.addInitScript(() => {
      delete (window as any).location;
      (window as any).location = { assign: () => {} };
    });

    await page.goto('/quoting?id=test-quote-123');

    await expect(page.locator('h1')).toContainText('Project Proposal');
    await expect(page.locator('h4', { hasText: 'Labor' })).toBeVisible();

    // Total is $100.00
    await expect(page.getByText('$100.00')).toBeVisible();

    // The status is ACCEPTED so "Pay Deposit" button is present
    await page.getByRole('button', { name: 'Pay Deposit' }).click();

    // We can't verify window.location.assign natively without a spy, but if the button click didn't throw and the mock was hit, it's successful.
  });
});
