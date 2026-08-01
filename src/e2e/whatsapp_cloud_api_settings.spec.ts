import { test, expect } from './fixtures';

test.describe('WhatsApp Cloud API Settings', () => {
    test('User can see instructions for getting API tokens', async ({ page }) => {
        await page.goto('/settings/integrations');
        // Wait for page
    });
});
