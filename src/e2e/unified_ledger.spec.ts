import { test, expect } from '@playwright/test';

test.describe('Unified Ledger & Multi-Currency Settlement Engine', () => {

    test('Ledger balance and statements are visible on the dashboard via FinancialOverviewCard', async ({ page }) => {
        await page.goto('/login');
        await page.fill('input[name="email"]', 'starter@example.com');
        await page.fill('input[name="password"]', 'password123');
        await page.click('button[type="submit"]');
        await page.waitForURL('/dashboard');

        // Check if the new Financial Overview card is displayed
        const financialsCard = page.locator('text=Financial Overview');
        await expect(financialsCard).toBeVisible();

        // Assert Money In, Money Out and Tax to Save are present
        await expect(page.locator('text=Money In')).toBeVisible();
        await expect(page.locator('text=Money Out')).toBeVisible();
        await expect(page.locator('text=Tax to Save')).toBeVisible();
    });

    test('Can simulate Snap Receipt from the FAB', async ({ page }) => {
        await page.goto('/login');
        await page.fill('input[name="email"]', 'starter@example.com');
        await page.fill('input[name="password"]', 'password123');
        await page.click('button[type="submit"]');
        await page.waitForURL('/dashboard');

        // Open FAB
        await page.click('button:has-text("+")');

        // Setup file chooser for the 'Snap Receipt' input
        const fileChooserPromise = page.waitForEvent('filechooser');
        await page.click('text=📸 Snap Receipt');
        const fileChooser = await fileChooserPromise;

        // Mock a receipt file upload
        await fileChooser.setFiles({
            name: 'receipt.jpg',
            mimeType: 'image/jpeg',
            buffer: Buffer.from('fake-image-content')
        });

        // The alert triggers after successful upload
        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain('Receipt uploaded successfully. AI is processing it.');
            await dialog.accept();
        });

        // Wait a brief moment for the upload to complete
        await page.waitForTimeout(1000);
    });

    test('Agent Accountant can answer balance queries', async ({ page }) => {
        await page.goto('/agent/chat');
        await page.fill('textarea', 'What is my current ledger balance?');
        await page.click('button[aria-label="Send message"]');

        await expect(page.locator('text=1500.00')).toBeVisible();
        await expect(page.locator('text=USD')).toBeVisible();
    });
});
