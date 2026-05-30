import { test, expect } from '@playwright/test';

test.describe('Team Approvals and Budget Alert E2E', () => {
  test('displays Budget Alert Toast when AI budget is low', async ({ page }) => {
    // Mock the approvals endpoint to return a low AI budget
    await page.route('/api/agents/approvals', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          pending_approvals: [
            {
              id: 'req-1',
              tenant_id: 'test-tenant',
              department: 'customer_success',
              description: 'Draft email for review | Payload: {"feature_type": "ambassador_reply", "original_message": "Do you do vegan cakes?", "generated_response": "Yes, we do vegan cakes!"}',
              status: 'PENDING_APPROVAL',
              action_risk: 'HIGH'
            }
          ],
          next_cursor: null,
          ai_budget: 5 // Low budget to trigger the toast
        }),
      });
    });

    await page.goto('/team');

    // Verify the budget alert toast is visible with the expected text
    await expect(page.locator('text=Your agents have been busy!')).toBeVisible();
    await expect(page.locator('text=You are at 90% of your AI budget.')).toBeVisible();
    await expect(page.locator('text=Upgrade Plan')).toBeVisible();
  });

  test('displays correctly formatted draft in Customer Success inbox', async ({ page }) => {
    // Mock the approvals endpoint to return a pending Customer Success draft
    await page.route('/api/agents/approvals', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          pending_approvals: [
            {
              id: 'req-1',
              tenant_id: 'test-tenant',
              department: 'customer_success',
              description: 'Draft email for review | Payload: {"feature_type": "ambassador_reply", "original_message": "Do you do vegan cakes?", "generated_response": "Yes, we do vegan cakes!"}',
              status: 'PENDING_APPROVAL',
              action_risk: 'HIGH'
            }
          ],
          next_cursor: null,
          ai_budget: 100 // High budget to avoid the toast
        }),
      });
    });

    await page.goto('/team');

    // Navigate to Customer Success Inbox
    await page.click('text=The Ambassador');

    // Verify the draft is correctly formatted
    await expect(page.locator('text=The Ambassador drafted an email.')).toBeVisible();
    await expect(page.locator('text=Do you do vegan cakes?')).toBeVisible();
    await expect(page.locator('text=Yes, we do vegan cakes!')).toBeVisible();

    // Verify the Approve and Edit buttons are present
    await expect(page.locator('button:has-text("Approve")')).toBeVisible();
    await expect(page.locator('button:has-text("Edit")')).toBeVisible();
  });
});
