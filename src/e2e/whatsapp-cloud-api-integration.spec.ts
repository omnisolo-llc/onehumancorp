import { test, expect } from './fixtures';

test.describe('WhatsApp Cloud API Integration', () => {
    test('user can link their WhatsApp Cloud API account', async ({ page }) => {
        // Login and navigate to Integrations
        await page.goto('/login');
        await page.fill('input[name="email"]', 'admin@example.com');
        await page.fill('input[name="password"]', 'password123');
        await page.click('button[type="submit"]');
        await page.waitForURL('/dashboard');

        await page.goto('/integrations');

        // Verify the Integration card exists
        const integrationCard = page.locator('h3', { hasText: 'WhatsApp Cloud API' });
        await expect(integrationCard).toBeVisible();

        // Click Connect
        const connectBtn = page.locator('div').filter({ has: integrationCard }).locator('button', { hasText: 'Connect' });
        await expect(connectBtn).toBeVisible();
        await connectBtn.click();

        // Verify modal appears
        const modalHeading = page.locator('h2', { hasText: 'Connect WhatsApp Cloud API' });
        await expect(modalHeading).toBeVisible();

        // Click to Connect with Meta
        const connectWithMetaBtn = page.locator('button', { hasText: 'Connect with Meta' });
        await connectWithMetaBtn.click();

        // Check if the success toast appears or status updates
        await expect(page.locator('text=WhatsApp Cloud API connected.')).toBeVisible();
    });
});
