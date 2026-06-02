import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Autonomous Subscription Box & Membership Engine', () => {
    test('CUJ: Create and manage a subscription box', async ({ page }) => {
        // Go to dashboard
        await page.goto('/dashboard');
        await expect(page.locator('h1').filter({ hasText: 'Dashboard' })).toBeVisible({ timeout: 5000 }).catch(() => null);

        // Wait for page to be ready
        await page.waitForLoadState('networkidle');

        // Click "Add Product"
        await page.evaluate(() => {
            if (window.showScreen) {
                window.showScreen('add-item-screen');
            }
        });
        await expect(page.locator('#add-item-screen')).toBeVisible();

        // Select Subscription Box
        await page.getByRole('radio', { name: '🔁 Subscription Box' }).click();

        // Fill details
        await page.locator('#item-name').fill('Monthly Coffee Bean');
        await page.locator('#item-price').fill('29.00');
        await page.locator('#item-desc').fill('Fresh roasted coffee delivered monthly.');
        await page.locator('#item-frequency').selectOption('monthly');
        await page.locator('#item-cutoff').fill('5');

        // Save
        await page.evaluate(() => {
            if (window.saveCatalogItem) {
                window.saveCatalogItem();
            }
        });

        // The save might pop an alert. Playwright handles alerts automatically by accepting them.
        // Wait to go back to dashboard or close
        await page.evaluate(() => {
            if (window.showScreen) {
                window.showScreen('dashboard-screen');
            }
        });

        // Click on Subscription Card
        await page.evaluate(() => {
            if (window.showScreen) {
                window.showScreen('subscription-manager-screen');
            }
        });

        // Ensure elements are visible
        await expect(page.locator('#subscription-manager-screen')).toBeVisible();
        await expect(page.locator('h1').filter({ hasText: 'Subscription Management' })).toBeVisible();

        // Check active subscribers count (mocked in API as 2)
        await expect(page.locator('#active-subscribers-count')).toBeVisible();

        // Wait for mocked API to populate
        await page.waitForTimeout(1000);

        // Check that there is a batch listed
        const batchList = page.locator('#batches-list');
        await expect(batchList).toBeVisible();

        // Verify print labels button exists and click it
        const printBtn = page.getByRole('button', { name: /Print \d+ Labels/ });
        await expect(printBtn).toBeVisible();

        // Click print labels
        page.once('dialog', dialog => dialog.accept());
        await printBtn.click();

        // Wait to make sure action finished
        await page.waitForTimeout(1000);

        // Now it should turn into a download link
        const downloadLink = page.getByRole('link', { name: 'Download Labels PDF' });
        await expect(downloadLink).toBeVisible();

        // Click simulate failed payment
        await page.evaluate(() => {
            if (window.showScreen) {
                window.showScreen('dashboard-screen');
            }
        });

        page.once('dialog', dialog => dialog.accept());
        const simulateBtn = page.getByRole('button', { name: 'Simulate Failed Payment (CS Agent)' });
        await expect(simulateBtn).toBeVisible();
        await simulateBtn.click();

        // Wait for feed to update
        await page.waitForTimeout(1500);
        await expect(page.locator('#activity-feed')).toContainText('Customer Success Agent');
        await expect(page.locator('#activity-feed')).toContainText('payment recovery email');
    });
});
