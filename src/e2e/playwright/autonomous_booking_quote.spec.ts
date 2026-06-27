import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking Quote & Scheduling Flow', () => {
    test.use({ viewport: { width: 375, height: 667 } }); // Mobile viewport

    test('should allow owner to 1-tap approve an autonomous quote with proposed times', async ({ page, request }) => {
        const tenantId = 'auto_quote_test_tenant_' + Date.now();

        // 1. Seed the database
        await request.post('/api/v1/builder/seeder/exec', {
          data: {
            sql: `
              INSERT INTO users (id, email, full_name, is_superadmin)
              VALUES ('auto_quote_user_id', 'auto_quote@example.com', 'Auto Quote User', false)
              ON CONFLICT DO NOTHING;

              INSERT INTO tenants (id, name, owner_email)
              VALUES ('${tenantId}', 'Auto Quote Store', 'auto_quote@example.com')
              ON CONFLICT DO NOTHING;

              INSERT INTO customers (id, tenant_id, name, email)
              VALUES ('cust_simulated_123', '${tenantId}', 'Test Customer', 'customer@example.com')
              ON CONFLICT DO NOTHING;
            `
          }
        });

        // 2. Post the webhook payload directly to the API to simulate generation
        const response = await page.request.post('/api/v1/agents/approvals/simulate-autonomous-booking-quote', {
            headers: {
                'x-test-tenant-id': tenantId,
                'x-test-user-id': 'auto_quote_user_id',
            }
        });
        expect(response.status()).toBe(200);

        // 3. Login
        await page.goto('/login');

        await page.getByPlaceholder('Email or Username').fill('auto_quote@example.com');
        await page.getByPlaceholder('Password').fill('password123');
        await page.getByRole('button', { name: 'Log In' }).click();

        // Wait for feed to load
        await page.waitForURL('**/dashboard**');

        // Ensure feed is populated with the simulated quote
        const approvalCard = page.locator('text=Draft quote and propose schedule for Emergency Handyman Service').first();
        await expect(approvalCard).toBeVisible();

        // 4. Click Review to open the modal
        await page.locator('button:has-text("Review")').first().click();

        // Modal should appear
        const modal = page.locator('role=dialog');
        await expect(modal).toBeVisible();

        // Verify that the proposed times exist in the modal
        await expect(modal.locator('text=Proposed Schedule')).toBeVisible();
        await expect(modal.locator('button:has-text("14:00")')).toBeVisible();

        // Verify default pricing
        await expect(modal.locator('[data-testid="modal-quote-total"]')).toContainText('$180.00');

        // 5. Approve & Send
        const approveBtn = modal.locator('[data-testid="modal-approve-btn"]');
        await approveBtn.click();

        // 6. Verify success (modal closes, feed item updates or disappears, but since this is E2E let's just assert modal closes)
        await expect(modal).toBeHidden();

        // Assert the feed item is now marked as Approved or Sent
        await expect(page.locator('text=Draft quote and propose schedule for Emergency Handyman Service').first()).toBeHidden();
    });
});
