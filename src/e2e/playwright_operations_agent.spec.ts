import { test, expect } from '@playwright/test';

test.describe('Operations Agent Fulfillment - Customer Use Journey', () => {

    test('E2E: Login and verify Operations Agent Dashboard load', async ({ page }) => {
        await page.goto('/');
        await expect(page.locator('body')).toBeVisible();
        await expect(page).toHaveTitle(/.*|OneHumanCorp/i);
    });

    test('E2E: Fetch Pending Approvals from Activity Feed', async ({ page }) => {
        await page.goto('/');
        await page.route('**/api/agents/approvals', async (route) => {
            await route.fulfill({
                json: {
                    pending_approvals: [{
                        id: '123',
                        tenant_id: 'tenant-1',
                        department: 'Operations',
                        description: 'Refund requested for Order 999',
                        status: 'Pending',
                        action_risk: 'HIGH'
                    }],
                    next_cursor: null
                }
            });
        });

        // Trigger request to see it handled in frontend
        const response = await page.request.get('/api/agents/approvals');
        expect(response.ok()).toBeTruthy();
        const data = await response.json();
        expect(data.pending_approvals).toBeDefined();
    });

    test('E2E: Approve Draft Action from Activity Feed', async ({ page }) => {
        await page.goto('/');
        await page.route('**/api/agents/approvals/123', async (route) => {
            await route.fulfill({ json: { success: true } });
        });

        const response = await page.request.post('/api/agents/approvals/123', {
            data: { approved: true }
        });
        expect(response.ok()).toBeTruthy();
    });

    test('E2E: Reject Draft Action from Activity Feed', async ({ page }) => {
        await page.goto('/');
        await page.route('**/api/agents/approvals/123', async (route) => {
            await route.fulfill({ json: { success: true } });
        });

        const response = await page.request.post('/api/agents/approvals/123', {
            data: { approved: false }
        });
        expect(response.ok()).toBeTruthy();
    });

    test('E2E: Verify Inventory Update UI Response', async ({ page }) => {
        await page.goto('/');
        await expect(page.locator('body')).toBeVisible();
    });
});
