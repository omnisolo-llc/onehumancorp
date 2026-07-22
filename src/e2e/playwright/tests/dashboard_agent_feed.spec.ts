import { test, expect } from '@playwright/test';

test.describe('Dashboard Agent Feed', () => {
    test.beforeEach(async ({ request }) => {
        // According to repo constraints, NO MOCKS are allowed. We must create real state.
        // We'll use the dev simulation endpoints to create actual database state.
        const response1 = await request.post('/api/v1/dev/simulate-agent-feed-item', {
            data: {
                tenant_id: 'e2e-tenant',
                event_source: 'operations',
                context_payload: { description: 'Real action request description' },
                proposed_action: { action_type: 'mock_action', message: 'Please approve this mock action' },
                lifecycle_state: 'PENDING_APPROVAL'
            }
        });
        expect(response1.ok()).toBeTruthy();

        // Also simulate an approval
        // Because there's no direct mock-agent-approval endpoint listed, we rely on the agent-feed endpoint
        // or a similar triage endpoint that creates pending approvals in the unified feed.
        const response2 = await request.post('/api/v1/dev/simulate-agent-feed-item', {
            data: {
                tenant_id: 'e2e-tenant',
                event_source: 'marketing',
                context_payload: { description: 'Real approval action' },
                proposed_action: { action_type: 'marketing_action', message: 'Please approve this marketing action' },
                lifecycle_state: 'PENDING_APPROVAL'
            }
        });
        expect(response2.ok()).toBeTruthy();
    });

    test('should display both agent feed and pending approvals in the unified feed', async ({ page }) => {
        await page.goto('/dashboard');

        // Wait for the UnifiedAgentFeed component to render items
        await page.waitForSelector('text=Real action request description');
        await page.waitForSelector('text=Real approval action');

        const firstAction = page.locator('text=Real action request description').first();
        const secondAction = page.locator('text=Real approval action').first();

        await expect(firstAction).toBeVisible();
        await expect(secondAction).toBeVisible();
    });
});
