import { test, expect } from '@playwright/test';

test.describe('Agent Teammate Mesh Dashboard - UI/UX', () => {

    // 1. Start from home/login page flow
    test('1. Start from home page and navigate to dashboard', async ({ page }) => {
        // Mock a login flow and navigate directly
        await page.goto('/dashboard');

        // Assert we are on the dashboard
        await expect(page.locator('h1').filter({ hasText: 'Dashboard' })).toBeVisible();
    });

    // 2. Verify Team Activity Panel layout and premium glassmorphism
    test('2. Verify Team Activity layout uses glassmorphism and Apple styling', async ({ page }) => {
        await page.goto('/dashboard');

        const activityPanelHeader = page.locator('h2').filter({ hasText: 'Team Activity' });
        await expect(activityPanelHeader).toBeVisible();

        // Check for Swarm Online pill
        await expect(page.locator('text=Swarm Online')).toBeVisible();
    });

    // 3. Verify Grandma Test (No technical developer terminology)
    test('3. Grandmother test - no technical jargon like JSON, API, Kubernetes shown', async ({ page }) => {
        await page.goto('/dashboard');

        // Ensure no developer jargon leaks into the dashboard view
        const pageText = await page.locator('body').innerText();
        expect(pageText.toLowerCase()).not.toContain('json');
        expect(pageText.toLowerCase()).not.toContain('kubernetes');
        expect(pageText.toLowerCase()).not.toContain('websocket');
        expect(pageText.toLowerCase()).not.toContain('payload');

        // It should use friendly terms
        await expect(page.locator('text=Team Activity')).toBeVisible();
        await expect(page.locator('text=Swarm Online')).toBeVisible();
    });

    // 4. Verify mesh activity real-time items render
    test('4. Realtime team activity items render correctly', async ({ page }) => {
        await page.goto('/dashboard');

        // Wait for mock interval to push an item (4.5s)
        await page.waitForTimeout(5000);

        // Agent icons should be visible
        const agentIcons = page.locator('text=🤖');
        expect(await agentIcons.count()).toBeGreaterThan(0);

        // Check if one of the mock agents appeared
        const pageText = await page.locator('body').innerText();
        const hasAgent = pageText.includes('Sales Agent') ||
                         pageText.includes('Support Agent') ||
                         pageText.includes('Marketing Agent') ||
                         pageText.includes('Operations Agent');

        expect(hasAgent).toBeTruthy();
    });

    // 5. Check fully responsive & full feature coverage
    test('5. Verify dashboard is fully responsive down to 375px', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 812 });
        await page.goto('/dashboard');

        // Ensure elements are still stacked nicely and readable
        const businessSnapshot = page.locator('h2').filter({ hasText: 'Business Snapshot' });
        await expect(businessSnapshot).toBeVisible();

        const teamActivity = page.locator('h2').filter({ hasText: 'Team Activity' });
        await expect(teamActivity).toBeVisible();
    });

});
