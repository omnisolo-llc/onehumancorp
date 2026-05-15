import { test, expect } from '@playwright/test';

test.describe('KAIROS Swarm Observability Panel', () => {

    test.beforeEach(async ({ page }) => {
        // Mock the pushState to prevent SecurityError
        await page.addInitScript(() => {
            const originalPushState = history.pushState;
            history.pushState = function(state, title, url) {
                try {
                    return originalPushState.apply(this, [state, title, url]);
                } catch (e) {
                    console.log('Caught pushState error:', e);
                }
            };
        });
    });

    test('should render the KAIROS UI glassmorphism style on body', async ({ page }) => {
        const response = await page.goto('/', { timeout: 60000 });

        // Simulate login to get to dashboard
        await page.evaluate(() => { window.showScreen('dashboard-screen'); });

        await expect(page.locator('#dashboard-screen')).toBeVisible();

        const bodyFilter = await page.evaluate(() => {
            return window.getComputedStyle(document.body).backdropFilter;
        });

        expect(bodyFilter).toContain('blur(20px)');
    });

    test('should display Swarm Observability Panel on Dashboard', async ({ page }) => {
        await page.goto('/', { timeout: 60000 });
        await page.evaluate(() => { window.showScreen('dashboard-screen'); });

        const panelTitle = page.locator('h2:has-text("Swarm Observability Panel")');
        await expect(panelTitle).toBeVisible();
    });

    test('should show Support Agent reply count', async ({ page }) => {
        await page.goto('/', { timeout: 60000 });
        await page.evaluate(() => { window.showScreen('dashboard-screen'); });

        const supportActivity = page.locator('p:has-text("✅ Your Support Agent replied to 3 customers")');
        await expect(supportActivity).toBeVisible();
    });

    test('should show Order Manager stock update', async ({ page }) => {
        await page.goto('/', { timeout: 60000 });
        await page.evaluate(() => { window.showScreen('dashboard-screen'); });

        const orderActivity = page.locator('p:has-text("📦 Order Manager updated stock for 12 items")');
        await expect(orderActivity).toBeVisible();
    });

    test('should ensure Swarm Observability Panel has glassmorphism class', async ({ page }) => {
        await page.goto('/', { timeout: 60000 });
        await page.evaluate(() => { window.showScreen('dashboard-screen'); });

        const panel = page.locator('h2:has-text("Swarm Observability Panel")').locator('..');
        await expect(panel).toHaveClass(/glass/);
    });
});
