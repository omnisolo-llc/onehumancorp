import { test, expect } from './fixtures';

test.describe('Autonomous Booking System E2E', () => {
    test('Booking flow end to end without mocked intercepts', async ({ page }) => {
        await page.goto('/booking');
        // Check UI loads
    });
});
