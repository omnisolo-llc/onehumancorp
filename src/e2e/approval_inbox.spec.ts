import { test, expect } from './fixtures';

test('AI Team Dashboard and Approval Inbox', async ({ page, request }) => {
  // Mock the API for testing the UI specifically
  await page.route('/api/agents/approvals', async (route) => {
    const json = {
      pending_approvals: [
        {
          id: '0',
          tenant_id: 'mock-tenant',
          department: 'CustomerSuccess',
          description: "3 customers haven't reviewed their orders. Request reviews?",
          status: 'Pending',
          action_risk: 'Low'
        },
        {
          id: 'e2e-approval-mock-1',
          tenant_id: 'mock-tenant',
          department: 'CustomerSuccess',
          description: 'Draft email for review: Maya ordered a vegan cake',
          status: 'Pending',
          action_risk: 'High'
        },
        {
          id: 'e2e-approval-mock-2',
          tenant_id: 'mock-tenant',
          department: 'Marketing',
          description: 'Draft Instagram Post: New vegan cakes available!',
          status: 'Pending',
          action_risk: 'Low'
        }
      ]
    };
    await route.fulfill({ json });
  });

  await page.route('/api/agents/approvals/*', async (route) => {
    await route.fulfill({ json: { success: true } });
  });

  // 1. User opens the app, authenticates and navigates to the Team Dashboard
  await page.goto('/');

  // Login via UI (from global-setup login structure)
  // Assuming the user is already logged in via global-setup.ts
  await page.goto('/team');

  // Assert Team Dashboard elements (375px mobile-first)
  await expect(page.locator('text=The Ambassador')).toBeVisible();
  await expect(page.locator('text=The Promoter')).toBeVisible();

  // "The Ambassador" has pending approvals indicator (e.g., a badge)
  const ambassadorCard = page.locator('text=The Ambassador').locator('..');
  await expect(ambassadorCard.locator('text=1 item awaiting approval')).toBeVisible();

  // 2. User taps "The Ambassador" department
  await ambassadorCard.click();

  // Verify approval inbox view for The Ambassador
  await expect(page.locator('text=Draft email for review: Maya ordered a vegan cake')).toBeVisible();
  await expect(page.locator("text=3 customers haven't reviewed their orders. Request reviews?")).toBeVisible();

  // 3. User approves the action (Swipe right / Approve button)
  const approveBtn = page.locator('button', { hasText: 'Approve' }).first();
  await approveBtn.click();

  // Wait for the action to be processed (mocking the UI removal)
  await expect(page.locator("text=3 customers haven't reviewed their orders. Request reviews?")).not.toBeVisible();
});
