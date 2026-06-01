import { test, expect } from '@playwright/test';

test.describe('Offline Marketing / QR Code Growth Loop', () => {
    test.beforeEach(async ({ page }) => {
        // Evaluate localstorage to simulate logged-in user without Pro tier
        await page.addInitScript(() => {
            window.localStorage.setItem('tenant_id', 'test-tenant');
            window.localStorage.setItem('business_name', 'Test Business');
            window.localStorage.setItem('has_pro', 'false');
        });
    });

    test('Business owner can navigate to QR code generator and trigger Pro paywall', async ({ page }) => {
        // 1. Owner starts at the dashboard
        await page.goto('http://localhost:3000/dashboard');

        // 2. Owner finds the new Offline to Online Growth section and clicks to create flyer
        const createFlyerBtn = page.locator('button', { hasText: 'Create QR Flyer' });
        await expect(createFlyerBtn).toBeVisible();
        await createFlyerBtn.click();

        // 3. System routes to the QR Code page
        await expect(page).toHaveURL(/.*\/qr-code/);

        // 4. Verify the UI renders correctly with default values
        await expect(page.locator('h1', { hasText: 'Offline Marketing 📱' })).toBeVisible();
        await expect(page.locator('h1', { hasText: 'Test Business' })).toBeVisible();
        await expect(page.locator('p', { hasText: 'Scan to Order' })).toBeVisible();
        await expect(page.locator('p', { hasText: 'ohc.store/test-tenant' })).toBeVisible();

        // 5. Check if the "PRO" tags are visible
        const proTags = page.locator('span', { hasText: 'PRO' });
        await expect(proTags).toHaveCount(2); // One for Color, One for Logo

        // 6. Owner tries to change the color of the QR code
        // The first button is the black (free) color, second is a custom color
        const colorButtons = page.locator('button', { has: page.locator('xpath=ancestor::div[contains(@class, "flex gap-3")]') }).nth(1);
        await colorButtons.click();

        // 7. System displays the Soft Paywall modal since they are on the free plan
        const modal = page.locator('div[role="dialog"]').or(page.locator('h2', { hasText: 'Custom Branding' }));
        await expect(modal).toBeVisible();
        await expect(page.locator('p', { hasText: 'Customizing QR code colors and adding your logo is a Pro feature.' })).toBeVisible();

        // 8. Owner dismisses the paywall and keeps the free version
        const keepFreeBtn = page.locator('button', { hasText: 'Keep Free Version' });
        await keepFreeBtn.click();

        // 9. Modal closes
        await expect(modal).toBeHidden();

        // 10. Owner verifies download buttons exist for the free flyer
        await expect(page.locator('button', { hasText: 'Download Flyer (PDF)' })).toBeVisible();
        await expect(page.locator('button', { hasText: 'Download QR Image (PNG)' })).toBeVisible();
    });

    test('Pro user can change QR code color', async ({ page }) => {
        // Evaluate localstorage to simulate logged-in user WITH Pro tier
        await page.addInitScript(() => {
            window.localStorage.setItem('tenant_id', 'test-tenant-pro');
            window.localStorage.setItem('business_name', 'Pro Business');
            window.localStorage.setItem('has_pro', 'true');
        });

        await page.goto('http://localhost:3000/qr-code');

        // Verify the "PRO" tags are hidden
        const proTags = page.locator('span', { hasText: 'PRO' });
        await expect(proTags).toHaveCount(0);

        // Owner tries to change the color of the QR code
        const blueColorBtn = page.locator('button', { has: page.locator('xpath=ancestor::div[contains(@class, "flex gap-3")]') }).nth(1);
        await blueColorBtn.click();

        // System does NOT display the Soft Paywall modal
        const modal = page.locator('h2', { hasText: 'Custom Branding' });
        await expect(modal).toBeHidden();

        // The QR code wrapper border should change to the selected color (#4F46E5)
        const qrWrapper = page.locator('.p-4.bg-white.rounded-3xl.shadow-lg');
        await expect(qrWrapper).toHaveCSS('border-color', 'rgb(79, 70, 229)'); // #4F46E5 converted to rgb
    });
});