import { test, expect } from '@playwright/test';

test.describe('Abandoned Cart Growth Loop', () => {
    test('generate cart recovery endpoint returns correct formatted fallback when backend is unavailable', async ({ request }) => {
        const payload = {
            customer_name: 'Maya',
            cart_value: '$45.00'
        };

        const response = await request.post('http://localhost:3000/api/v1/growth/campaign/generate-cart', {
            data: payload
        });

        expect(response.ok()).toBeTruthy();

        const data = await response.json();
        expect(data).toHaveProperty('message');

        const msg = data.message;

        expect(msg).toContain('Hi Maya,');
        expect(msg).toContain('totaling $45.00');
        expect(msg).toContain('https://ohc.store/checkout/recover');
        expect(msg).toContain('⚡ Powered by OHC');
    });

    test('generate cart recovery endpoint returns generic fallback if payload is empty', async ({ request }) => {
        const response = await request.post('http://localhost:3000/api/v1/growth/campaign/generate-cart', {
            data: {}
        });

        expect(response.ok()).toBeTruthy();
        const data = await response.json();

        const msg = data.message;
        expect(msg).toContain('Hi there,');
        expect(msg).toContain('totaling $0.00');
        expect(msg).toContain('https://ohc.store/checkout/recover');
    });

    test('dashboard abandoned cart modal opens and displays drafted message', async ({ page }) => {
        await page.goto('http://localhost:3000/dashboard');

        // Wait for page hydration
        await page.waitForLoadState('networkidle');

        // The dashboard is hardcoded to show an abandoned cart for "Alex" with "$85.00"
        // Let's use that data for the E2E verification
        const recoverButton = page.getByRole('button', { name: 'Recover Cart' });
        await recoverButton.click();

        // Check if modal opens with the title "AI Cart Recovery"
        const modalTitle = page.getByRole('heading', { name: 'AI Cart Recovery' });
        await expect(modalTitle).toBeVisible();

        // Wait for the text area to be populated (this triggers the real fetch to generate-cart)
        const textArea = page.locator('textarea');
        await expect(textArea).toHaveValue(/Hi Alex,/);
        await expect(textArea).toHaveValue(/totaling \$85\.00/);
        await expect(textArea).toHaveValue(/⚡ Powered by OHC/);

        // Send campaign
        const sendButton = page.getByRole('button', { name: 'Send Campaign' });
        await sendButton.click();

        // Check success message appears
        const successMessage = page.getByRole('heading', { name: 'Campaign Sent Successfully!' });
        await expect(successMessage).toBeVisible();
    });
});
