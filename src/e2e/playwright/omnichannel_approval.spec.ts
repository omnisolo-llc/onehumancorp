import { test, expect } from '@playwright/test';

test.describe('Omnichannel Inbox Approval Flow', () => {
    test.use({ viewport: { width: 375, height: 667 } }); // Mobile viewport

    test('should allow owner to 1-tap approve an omnichannel draft', async ({ page, request }) => {
        const tenantId = 'omni_test_tenant_' + Date.now();
        const customerPhone = '+15555551234';

        // 1. Seed the database
        await request.post('/api/v1/builder/seeder/exec', {
          data: {
            sql: `
              INSERT INTO users (id, email, full_name, is_superadmin)
              VALUES ('omni_user_id', 'omni_user@example.com', 'Omni User', false)
              ON CONFLICT DO NOTHING;

              INSERT INTO tenants (id, name, owner_email)
              VALUES ('${tenantId}', 'Omni Store', 'omni_user@example.com')
              ON CONFLICT DO NOTHING;

              INSERT INTO customers (id, tenant_id, name, email, phone)
              VALUES ('test_cust_1', '${tenantId}', 'Test Omnichannel Customer', 'omni@example.com', '${customerPhone}')
              ON CONFLICT DO NOTHING;
            `
          }
        });

        // 2. Post the webhook payload directly to the API
        const response = await page.request.post('/api/v1/omnichannel/webhook', {
          data: {
            tenant_id: tenantId,
            channel: 'instagram',
            sender_id: customerPhone,
            message: 'Do you have vegan chocolate cake available for Saturday?'
          }
        });

        expect(response.status()).toBe(200);

        // Wait a brief moment for the background worker to process triage
        await page.waitForTimeout(3000);

        // Navigate to dashboard where feed is shown
        await page.goto(`/login?test_email=omni_user@example.com`);
        await page.evaluate((t) => localStorage.setItem('tenant', t), tenantId);
        await page.goto('/dashboard');

        // Verify feed
        const feedSection = page.locator('section', { hasText: 'Proposals' }).first();
        await expect(feedSection).toBeVisible({ timeout: 15000 });

        // Verify mobile constraints
        const bodyBox = await page.locator('body').boundingBox();
        expect(bodyBox?.width).toBeLessThanOrEqual(375);

        // Check if the drafted reply is visible
        const dmCard = page.getByTestId('ambassador-reply-card');
        await expect(dmCard).toBeVisible({ timeout: 15000 });
        await expect(dmCard.getByText('Do you have vegan chocolate cake available for Saturday?')).toBeVisible();
        await expect(dmCard.getByText('Draft Reply')).toBeVisible();

        // Approve the response
        const approveButton = page.getByTestId('feed-approve-btn');
        await expect(approveButton).toBeVisible();

        // Ensure the button has a min 44x44 bounding box
        const box = await approveButton.boundingBox();
        expect(box).not.toBeNull();
        if (box) {
          expect(box.width).toBeGreaterThanOrEqual(44);
          expect(box.height).toBeGreaterThanOrEqual(44);
        }

        await approveButton.click();

        // Verify it disappears
        await expect(dmCard).not.toBeVisible({ timeout: 10000 });
    });
});
