import { test, expect } from './fixtures';

test.describe('🛡️ Sentry: AI Agent Resilience', () => {
    test('Verify PAUSED state notification when AI service is unavailable', async ({ adminPage: page }) => {
        await page.route('**/api/ai/reason', async route => {
            await route.fulfill({ status: 503, body: JSON.stringify({ error: "Service Unavailable" }) });
        });
        await page.goto('/dashboard/tasks');
        await expect(page.getByText('AI Agent Paused')).toBeVisible();
    });
});
