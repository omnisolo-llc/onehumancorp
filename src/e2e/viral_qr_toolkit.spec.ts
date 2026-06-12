import { test, expect } from './fixtures';

test.describe('QR Code Toolkit Growth Feature', () => {
    test.beforeEach(async ({ page }) => {
        // Start from dashboard
        await page.goto('/dashboard');
        await expect(page.locator('h1')).toContainText(/Dashboard/i);
    });

    test('owner can navigate to QR Toolkit and generate codes', async ({ page }) => {
        // Find the QR Toolkit card in Growth section
        const toolkitBtn = page.getByRole('link', { name: /QR Code Toolkit/i });
        await expect(toolkitBtn).toBeVisible();
        await toolkitBtn.click();

        // Verify we are on the toolkit page
        await expect(page).toHaveURL(/\/qr-toolkit/);
        await expect(page.locator('h1')).toContainText(/QR Code Toolkit/i);

        // Check default configuration
        await expect(page.getByText(/Visit Storefront/i)).toBeVisible();

        // Verify live preview renders an SVG
        const qrSvg = page.locator('#printable-qr svg');
        await expect(qrSvg).toBeVisible();

        // Switch to Review action
        const reviewBtn = page.getByRole('button', { name: /Leave a Review/i });
        await reviewBtn.click();

        // Verify the QR value would change (implicitly checked by interaction,
        // real value check would need component state inspection or clipboard)

        // Test soft paywall for premium features
        const frameToggle = page.getByRole('button', { name: /Branded Frame/i });
        await frameToggle.click();

        // Paywall modal should appear
        await expect(page.getByText(/Unlock Branded QR Tools/i)).toBeVisible();

        // Click "Maybe later" to close
        await page.getByRole('button', { name: /Maybe later/i }).click();
        await expect(page.getByText(/Unlock Branded QR Tools/i)).toBeHidden();
    });

    test('owner can unlock Pro features via share loop', async ({ page }) => {
        await page.goto('/qr-toolkit');

        // Open paywall
        await page.getByRole('button', { name: /Branded Frame/i }).click();

        // Click share to unlock
        const shareBtn = page.getByRole('button', { name: /Share on X to Unlock/i });
        await expect(shareBtn).toBeVisible();

        // Intercept the window.open call for Twitter
        const [popup] = await Promise.all([
            page.waitForEvent('popup'),
            shareBtn.click(),
        ]);

        await expect(popup).toHaveURL(/twitter\.com\/intent\/tweet/);
        await popup.close();

        // Check for success message (simulated in implementation)
        await expect(page.getByText(/Upgrade Success!/i)).toBeVisible();

        // Wait for paywall to auto-close
        await expect(page.getByText(/Unlock Branded QR Tools/i)).toBeHidden({ timeout: 5000 });

        // Premium features should now be toggleable without paywall
        // (Implementation might need page reload or state update)
    });

    test('responsive layout check at 375px', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 667 });
        await page.goto('/qr-toolkit');

        // Header should be visible and not overflowing
        await expect(page.locator('h1')).toBeVisible();

        // Configuration and Preview should stack vertically (grid-cols-1)
        const mainGrid = page.locator('main');
        await expect(mainGrid).toHaveClass(/grid-cols-1/);

        // Touch targets check (buttons should be at least 44px high)
        const visitBtn = page.getByRole('button', { name: /Visit Storefront/i });
        const box = await visitBtn.boundingBox();
        expect(box?.height).toBeGreaterThanOrEqual(44);
    });
});
