import { test, expect } from '@playwright/test';

test.describe('Quote Engine', () => {
  // Use a predictable tenant for the E2E test if possible, or setup a new one

  test('AI generating draft -> Owner editing & approving -> Customer accepting', async ({ page, request }) => {
    // 1. Simulate AI drafting a quote
    const res = await request.post('/api/agents/approvals/simulate-quote-draft', {
        headers: { 'x-tenant-id': 'test-e2e-quotes' }
    });
    // This creates a quote draft and adds it to the agent feed.
    // wait for it
    await page.waitForTimeout(2000);

    // Navigate to feed
    await page.goto('/feed');

    // We should see an "Edit Quote" button
    const editBtn = page.locator('[data-testid="feed-edit-quote-btn"]').first();
    await editBtn.waitFor({ state: 'visible', timeout: 15000 });

    // Extract the URL before clicking
    const editUrl = await editBtn.evaluate((el: any) => el.onclick?.toString() || window.location.href);

    await editBtn.click();

    // On edit page
    await page.waitForSelector('text=Edit Quote Draft');

    // Check initial values
    const descInput = page.locator('[data-testid="quote-item-desc-0"]');
    await expect(descInput).toBeVisible();

    // Update quantity
    const qtyInput = page.locator('[data-testid="quote-item-qty-0"]');
    await qtyInput.fill('2');

    // Save
    const saveBtn = page.locator('[data-testid="save-quote-btn"]');
    await saveBtn.click();

    // Wait for redirect to feed
    await page.waitForURL('**/feed');

    // Now extract Quote ID from URL we went to earlier to simulate the customer view
    // Since Playwright evaluate might not get the exact URL from onClick we can intercept the request or just find the quote id

    // Easiest is to go to the customer quote page by listing quotes from API
    const feedRes = await request.get('/api/agent-feed', { headers: { 'x-tenant-id': 'test-e2e-quotes' }});
    const feedData = await feedRes.json();
    const quoteId = feedData.items[0]?.context_payload?.quote_id;

    if (quoteId) {
        await page.goto(`/quoting?id=${quoteId}`);
        await page.waitForSelector('text=Project Proposal');

        // Check if price reflects quantity = 2. Initially it was 250, so 500
        await expect(page.locator('text=$500.00')).toBeVisible();

        // Click Accept
        await page.click('button:has-text("Accept Proposal")');

        // Wait for success
        await expect(page.locator('text=Proposal Accepted')).toBeVisible();
    }
  });
});
