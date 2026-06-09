import { test, expect } from './fixtures';

test.describe('Invisible Offline-to-Online QR Commerce Bridge', () => {
    test('merchant can generate a contextual QR code for an imported product', async ({ page }) => {
        // Assume test runner handles login and initial navigation via fixtures if necessary.
        // We will directly test the products page route.
        await page.goto('/products');

        // Verify the page title
        const pageTitle = page.locator('h1', { hasText: 'Products' });
        await expect(pageTitle).toBeVisible();

        // Check for the "Imported Products" section
        const sectionTitle = page.locator('div.app-panel-title', { hasText: 'Imported Products' });
        await expect(sectionTitle).toBeVisible();

        // Click the first "Generate QR Code" button
        const generateButton = page.locator('button', { hasText: 'Generate QR Code' }).first();
        await expect(generateButton).toBeVisible();
        await generateButton.click();

        // Verify the QR Code Modal is opened
        const modalTitle = page.locator('h2', { hasText: 'Checkout QR Code' });
        await expect(modalTitle).toBeVisible();

        // The text should mention the product, e.g., "Chocolate Cake"
        const modalBody = page.locator('p', { hasText: 'Print or display this code. Customers can scan it to instantly buy' });
        await expect(modalBody).toBeVisible();
        await expect(modalBody).toContainText('Chocolate Cake');

        // Check that the image is rendered with the correct source URL
        const qrImage = page.locator('img[alt="QR Code for Chocolate Cake"]');
        await expect(qrImage).toBeVisible();
        const imgSrc = await qrImage.getAttribute('src');
        expect(imgSrc).toContain('api.qrserver.com');
        expect(imgSrc).toContain(encodeURIComponent(encodeURIComponent('Chocolate Cake')));
        expect(imgSrc).toContain('checkout');

        // Verify Save / Print button
        const saveButton = page.locator('button', { hasText: 'Save / Print' });
        await expect(saveButton).toBeVisible();

        // Verify "Powered by OHC" watermark
        const poweredBy = page.locator('p', { hasText: 'Powered by OHC' });
        await expect(poweredBy).toBeVisible();

        // Close the modal
        const closeBtn = page.locator('button').filter({ has: page.locator('svg') }).first(); // Assuming the close btn has SVG and is first button inside modal
        // A more robust selector for the close button
        await page.locator('.fixed button').first().click();

        // Modal should be gone
        await expect(modalTitle).toBeHidden();
    });
});
