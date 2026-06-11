import { test, expect } from '@playwright/test';

test.describe('Proactive Context-Aware Task Suggestions', () => {
    test.beforeEach(async ({ page, request }) => {
        // Set up standard tenant user session. We can use a simple mock token or default setup.
        // E2E test rule: start from the home page after user login.

        // Let's create an explicit tenant context and trigger the backend
        const tenant_id = "test-tenant-123";

        // Use the test endpoint to trigger the job queue
        const res = await request.post('/api/test/trigger-proactive-analysis', {
            data: { tenant_id }
        });

        expect(res.ok()).toBeTruthy();

        // Set tenant context for frontend
        await page.addInitScript(() => {
            localStorage.setItem("tenant_id", "test-tenant-123");
            localStorage.setItem("user_id", "default");
        });

        await page.goto('/dashboard');
    });

    test('should display proactive suggestion in feed and allow approval', async ({ page }) => {
        // Since the backend processes the job asynchronously (polling every 5 seconds),
        // we might need to wait for the feed to update or reload the page.
        // We'll give it a few seconds for the worker to process the job.
        await page.waitForTimeout(6000);

        // Reload to get the fresh feed from DB
        await page.reload();

        // 1. Look for the Priority Action card
        const priorityActionBadge = page.locator('span.uppercase', { hasText: 'Priority Action' });
        await expect(priorityActionBadge).toBeVisible();

        const insightTitle = page.locator('h3', { hasText: 'Needs Attention Today' });
        await expect(insightTitle).toBeVisible();

        const insightMessage = page.locator('div', { hasText: 'You have 2 estimates pending from yesterday. Tap to review drafted follow-up messages.' }).first();
        await expect(insightMessage).toBeVisible();

        // 2. Click the Approve button
        const approveBtn = page.locator('button', { hasText: 'Approve' }).first();
        await expect(approveBtn).toBeVisible();

        // Click and verify the card goes away
        await approveBtn.click();

        // The item should be filtered out by the optimistic UI update
        await expect(priorityActionBadge).not.toBeVisible();
        await expect(insightTitle).not.toBeVisible();
    });
});
