import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';
import * as path from 'path';

test.describe('Multimodal Quote E2E (Issue #28020)', () => {
  test('Customer requests quote with image, Agent drafts Estimate, Owner approves, Customer checks out', async ({ browser, page }) => {
    // 1. Customer Context
    const customerContext = await browser.newContext();
    const customerPage = await customerContext.newPage();

    await customerPage.goto('/instant-quote.html?tenant=e2e-tenant');

    await customerPage.waitForSelector('#notes');
    await customerPage.fill('#notes', 'My sink is leaking under the cabinet. I need an estimate for repair.');

    const fileChooserPromise = customerPage.waitForEvent('filechooser');
    await customerPage.click('#image-upload-input');
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles(path.join(__dirname, 'test-image.jpg'));

    await customerPage.click('#submit-btn');
    await customerPage.waitForSelector('#success-view', { state: 'visible' });
    await customerContext.close();

    // 2. Owner Context (using default page)
    await page.goto('/quote.html');

    // Check if the quote is in the UI
    await expect(page.locator('text=My sink is leaking under the cabinet')).toBeVisible({ timeout: 10000 });

    // Owner clicks Approve & Send
    await page.click('#approve-quote-btn');

    // Wait for success message
    await expect(page.locator('text=Estimate approved and sent to customer')).toBeVisible();

    // 3. Customer checks out
    // In our E2E environment we just verify the checkout URL generation logic
    const checkoutUrl = await page.getAttribute('#checkout-link', 'href');
    expect(checkoutUrl).toContain('checkout');
  });
});
