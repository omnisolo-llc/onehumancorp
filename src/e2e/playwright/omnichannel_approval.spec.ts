import { test, expect } from '@playwright/test';

test.describe('Omnichannel Inbox Approval Flow', () => {
    test.use({ viewport: { width: 375, height: 667 } }); // Mobile viewport

    test('should allow owner to 1-tap approve an omnichannel draft', async ({ page, request }) => {
        // 1. Simulate an incoming webhook
        const tenantId = 'tenant_omni_test';
        const webhookPayload = {
            tenant_id: tenantId,
            source: 'instagram',
            identifier: 'sarah_bakes',
            message: 'Do you have vegan chocolate cake available for Saturday?'
        };

        // Assume API is running on localhost:8080 or test environment endpoint
        // For the sake of E2E, we might need a test-specific endpoint setup.
        // We will just verify the UI flow assuming data is seeded or mocked in the E2E setup.

        // This is a placeholder test showing the flow requested.
        await page.goto('/feed');

        // Wait for Action Required card to appear
        const actionCard = page.locator('text=1 New Message from Sarah (Insta DM)');
        await expect(actionCard).toBeVisible({ timeout: 10000 });

        // Tap to open unified view
        await actionCard.click();

        // Verify context and drafted reply
        await expect(page.locator('text=Sarah bought a vegan cake')).toBeVisible();
        await expect(page.locator('text=Hi Sarah! Yes, we still make')).toBeVisible();

        // 1-Tap Approve
        const approveButton = page.locator('button:has-text("Approve & Send")');
        await expect(approveButton).toBeVisible();
        await approveButton.click();

        // Verify success
        await expect(page.locator('text=Message Sent')).toBeVisible();
    });
});
