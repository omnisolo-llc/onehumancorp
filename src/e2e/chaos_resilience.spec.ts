import { test, expect } from '@playwright/test';

test.describe('Chaos Resilience & ML Timeout Validation', () => {
    test('UI reflects PAUSED state when AI agent times out (60s simulation)', async ({ page, request }) => {
        test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
        // Mock the backend to inject a PAUSED state directly into the UI. This tests that the UI component gracefully renders
        // the state and its styling correctly when an agent times out and pauses.
        await page.route('**/api/v1/orchestration/tasks', async route => {
            const json = {
                tasks: [
                    {
                        id: 'mock-paused-task',
                        department: 'operations',
                        status: 'PAUSED',
                        proposed_content: 'System is paused. Please manually check inventory.',
                        updated_at: new Date().toISOString()
                    }
                ]
            };
            await route.fulfill({ json });
        });

        // Navigation already handled by loginAs in fixtures for '/dashboard'
        await page.goto('/dashboard');

        // Navigate to Inbox or Operations to check agent status
        await page.click('text=Operations');

        // Ensure that the translucent glass UI applies to failure states explicitly
        const pausedCard = page.locator('.agent-paused-card').first();
        await expect(pausedCard).toBeVisible();
        await expect(pausedCard).toHaveCSS('backdrop-filter', /blur\(30px\)/);
        await expect(pausedCard).toContainText('System is paused');
    });
});
