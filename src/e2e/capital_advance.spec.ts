import { test, expect } from '@playwright/test';

test.describe('Capital Advance User Journey', () => {
  test('should display capital offer, allow adjusting the amount, and accept the advance', async ({ page }) => {
    // Navigate to the app (assuming dev server runs on localhost:3000)
    await page.goto('http://localhost:3000/');

    // Click "Dashboard" or go directly to the dashboard logic
    // We can evaluate scripts or use the UI to navigate
    await page.evaluate(() => {
        if (typeof showScreen === 'function') {
            showScreen('dashboard-screen');
        }
    });

    // We wait for the capital advance container to be populated
    const container = page.locator('#capital-advance-container');
    await expect(container).toBeVisible();

    // Verify offer is shown
    await expect(page.locator('text=You\'re approved for a $1500 advance')).toBeVisible();

    // The slider default is 1500
    const display = page.locator('#capital-display');
    await expect(display).toHaveText('$1500');

    const slider = page.locator('#capital-slider');
    await slider.fill('1000');
    // For input[type=range], fill or evaluate might be needed to trigger input
    await slider.evaluate((node: HTMLInputElement) => {
        node.value = '1000';
        node.dispatchEvent(new Event('input', { bubbles: true }));
    });

    await expect(display).toHaveText('$1000');
    await expect(page.locator('text=until $1000 is repaid')).toBeVisible();

    // Accept the offer
    const acceptBtn = page.locator('#capital-accept-btn');
    await acceptBtn.click();

    // Wait for the success animation
    await expect(page.locator('text=🎉 Funds Added!')).toBeVisible();

    // The page reloads after 2 seconds, wait for it
    await page.waitForTimeout(2500);

    // Verify we are now seeing the Active Advance view instead of the offer
    await expect(page.locator('text=Active Advance: $1000')).toBeVisible();
    await expect(page.locator('text=Remaining Balance: $1000')).toBeVisible();
  });
});
