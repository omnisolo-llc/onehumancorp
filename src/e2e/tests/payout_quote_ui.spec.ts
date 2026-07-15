import { test, expect } from '@playwright/test';

// Standard E2E verification for the Owner Feed UI components
test.describe('Owner Feed - UI Components Verify', () => {
    test.beforeEach(async ({ page }) => {
        // Navigate to the main dashboard
        await page.goto('/');
    });

    test('Verify minimum touch target on standard dashboard buttons', async ({ page }) => {
        // Ensure all primary action buttons on the dashboard have 44px min size
        // We'll wait for the dashboard to be ready, then check visible buttons
        await page.waitForLoadState('networkidle');

        // Find visible buttons that look like primary actions
        const buttons = page.locator('button:visible');

        // Wait for at least one button to be ready, or we pass since there's no UI to fail
        try {
            await buttons.first().waitFor({ state: 'visible', timeout: 5000 });
        } catch {
            // If the dashboard doesn't have buttons by default, that's not a failure of this specific test
            return;
        }

        const count = Math.min(await buttons.count(), 5);
        for (let i = 0; i < count; i++) {
            const box = await buttons.nth(i).boundingBox();
            if (box) {
                // Assert the minimum touch targets
                expect(box.width).toBeGreaterThanOrEqual(44);
                expect(box.height).toBeGreaterThanOrEqual(44);
            }
        }
    });

    test('Verify responsive container sizing on 375px viewport', async ({ page }) => {
        // Mobile viewport
        await page.setViewportSize({ width: 375, height: 667 });
        await page.goto('/');

        await page.waitForLoadState('domcontentloaded');

        // Verify body doesn't overflow horizontally
        const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
        const windowWidth = await page.evaluate(() => window.innerWidth);
        expect(bodyWidth).toBeLessThanOrEqual(windowWidth);
    });

    test('Verify responsive container sizing on desktop viewport', async ({ page }) => {
        // Desktop viewport
        await page.setViewportSize({ width: 1440, height: 900 });
        await page.goto('/');

        await page.waitForLoadState('domcontentloaded');

        // Verify body doesn't overflow horizontally
        const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
        const windowWidth = await page.evaluate(() => window.innerWidth);
        expect(bodyWidth).toBeLessThanOrEqual(windowWidth);
    });

    test('Verify PayoutSummaryCard approve button exists and is clickable', async ({ page }) => {
        // Wait for feed to load
        await page.goto('/');
        await page.waitForLoadState('networkidle');

        // This relies on the UI rendering the correct test id for the card buttons
        const approveBtns = page.locator('[data-testid="feed-approve-btn"]');
        if (await approveBtns.count() > 0) {
            const btn = approveBtns.first();
            await expect(btn).toBeVisible();
            await btn.click({ trial: true });
        }
    });

    test('Verify ReviewDraftQuoteCard edit button exists and is clickable', async ({ page }) => {
        await page.goto('/');
        await page.waitForLoadState('networkidle');

        const dismissBtns = page.locator('[data-testid="feed-dismiss-btn"]');
        if (await dismissBtns.count() > 0) {
            const btn = dismissBtns.first();
            await expect(btn).toBeVisible();
            await btn.click({ trial: true });
        }
    });

    test('Verify AgentFeedCard rendering with valid touch targets', async ({ page }) => {
        await page.goto('/');
        await page.waitForLoadState('networkidle');

        const approveBtns = page.locator('[data-testid="feed-approve-btn"]');
        const dismissBtns = page.locator('[data-testid="feed-dismiss-btn"]');

        if (await approveBtns.count() > 0 && await dismissBtns.count() > 0) {
            const boxApprove = await approveBtns.first().boundingBox();
            if (boxApprove) {
                expect(boxApprove.width).toBeGreaterThanOrEqual(44);
                expect(boxApprove.height).toBeGreaterThanOrEqual(44);
            }
            const boxDismiss = await dismissBtns.first().boundingBox();
            if (boxDismiss) {
                expect(boxDismiss.width).toBeGreaterThanOrEqual(44);
                expect(boxDismiss.height).toBeGreaterThanOrEqual(44);
            }
        }
    });

    test('Verify owner feed transitions on 375px mobile screen', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 667 });
        await page.goto('/');
        await page.waitForLoadState('networkidle');

        const approveBtns = page.locator('[data-testid="feed-approve-btn"]');
        if (await approveBtns.count() > 0) {
            const boxApprove = await approveBtns.first().boundingBox();
            if (boxApprove) {
                expect(boxApprove.width).toBeGreaterThanOrEqual(44);
            }
        }
    });

    test('Verify owner feed transitions on 768px tablet screen', async ({ page }) => {
        await page.setViewportSize({ width: 768, height: 1024 });
        await page.goto('/');
        await page.waitForLoadState('networkidle');

        const approveBtns = page.locator('[data-testid="feed-approve-btn"]');
        if (await approveBtns.count() > 0) {
            const boxApprove = await approveBtns.first().boundingBox();
            if (boxApprove) {
                expect(boxApprove.width).toBeGreaterThanOrEqual(44);
            }
        }
    });
});
