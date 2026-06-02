import { test, expect } from '@playwright/test';

test.describe('Google Business Sync E2E', () => {
    test.beforeEach(async ({ page }) => {
        // Mock API responses
        await page.route('**/api/v1/auth/login', route => {
            route.fulfill({
                status: 200,
                json: { token: 'mock-token', tenant_id: 'test-tenant' }
            });
        });

        await page.route('**/api/v1/business/settings', route => {
            route.fulfill({
                status: 200,
                json: {
                    name: "Carlos Handyman",
                    category: "repair",
                    stage: "operating"
                }
            });
        });

        await page.goto('/');

        // Wait for page load and skip setup if needed
        try {
            const skipButton = await page.getByRole('button', { name: /Skip/i });
            if (await skipButton.isVisible({ timeout: 2000 })) {
                await skipButton.click();
            }
        } catch (e) {}
    });

    test('should allow connecting Google Business and approving reviews', async ({ page }) => {
        // Find the Local Visibility card
        await expect(page.getByText('Local Visibility', { exact: false })).toBeVisible();

        // Connect to Google Business
        const connectBtn = page.getByRole('button', { name: 'Connect Google Business' });
        await expect(connectBtn).toBeVisible();
        await connectBtn.click();

        // Verify state change after clicking connect
        await expect(page.getByText('Connecting...')).toBeVisible();

        // Should eventually show Synced status
        await expect(page.getByText('🟢 Synced with Google Maps')).toBeVisible({ timeout: 5000 });

        // The reviews container should now be visible
        await expect(page.getByText('3 New Reviews to Approve')).toBeVisible();

        // Check the mock review is there
        await expect(page.getByText('John Doe')).toBeVisible();
        await expect(page.getByText('"Great plumbing service, fixed it quickly!"')).toBeVisible();
        await expect(page.getByText("AI Draft: 'Hi John, thanks for trusting us with your plumbing repair! We are glad it was fixed quickly. - Carlos'")).toBeVisible();

        // Approve and Reply
        const approveBtn = page.getByRole('button', { name: 'Approve & Reply' });
        await approveBtn.click();

        // Check the publishing state
        await expect(page.getByText('Publishing...')).toBeVisible();

        // Verify success message
        await expect(page.getByText('Reply published to Google Maps!')).toBeVisible({ timeout: 5000 });
    });
});
