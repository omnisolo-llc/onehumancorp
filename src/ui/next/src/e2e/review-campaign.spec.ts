import { test, expect } from '@playwright/test';

test.describe('Automated Review Campaign Growth Loop', () => {
    test('generate review endpoint returns correct formatted fallback when backend is unavailable', async ({ request }) => {
        const payload = {
            order_id: '12345',
            customer_name: 'Alice',
            product_name: 'Super Gadget'
        };

        const response = await request.post('http://localhost:3000/api/v1/growth/campaign/generate-review', {
            data: payload
        });

        expect(response.ok()).toBeTruthy();

        const data = await response.json();
        expect(data).toHaveProperty('message');

        const msg = data.message;
        // Verify it inserted the payload data
        expect(msg).toContain('Hi Alice,');
        expect(msg).toContain('Super Gadget');
        expect(msg).toContain('https://ohc.store/review/12345');

        // Ensure the referral growth loop is intact in the signature
        expect(msg).toContain('⚡ Powered by OHC');
    });

    test('generate review endpoint returns generic fallback if payload is empty', async ({ request }) => {
        const response = await request.post('http://localhost:3000/api/v1/growth/campaign/generate-review', {
            data: {}
        });

        expect(response.ok()).toBeTruthy();
        const data = await response.json();

        const msg = data.message;
        expect(msg).toContain('Hi Customer,');
        expect(msg).toContain('your order');
        expect(msg).toContain('https://ohc.store/review/recent');
    });
});

    test('review campaign page shows soft paywall when trying to send without pro', async ({ page }) => {
        await page.goto('http://localhost:3000/review-campaigns');

        // Ensure not on pro
        await page.evaluate(() => {
            localStorage.setItem('has_pro', 'false');
        });
        await page.reload();

        // Generate draft
        await page.click('button:has-text("Generate Email Draft")');

        // Click send
        await page.click('button:has-text("Send to Audience")');

        // Verify soft paywall opens
        await expect(page.locator('h2:has-text("Unlock Automated Campaigns")')).toBeVisible();

        // Verify Share on X button is present
        await expect(page.locator('button:has-text("Share on X to unlock 7 Days Free")')).toBeVisible();
    });
