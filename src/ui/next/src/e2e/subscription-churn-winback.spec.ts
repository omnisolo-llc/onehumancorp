import { expect, test } from '@playwright/test';

test.describe('Agentic Subscription Retention & Churn Prediction System CUJ', () => {
  test('Owner sees drafted win-back offer for at-risk subscriber in Agent Feed and approves it', async ({ page, loginAs, adminUser, request }) => {
    await loginAs(page, adminUser);

    // 1. Simulate the Churn Prediction Worker having found an at-risk subscriber
    // and drafting a win-back message via The Ambassador.
    await request.post('/api/agents/approvals/simulate-subscription-churn-winback', {
      headers: {
        'x-tenant-id': adminUser.tenantId || 'e2e-tenant'
      }
    });

    // Wait for insertion
    await page.waitForTimeout(2000);

    // 2. Navigate to Dashboard (Agent Feed)
    await page.goto('/dashboard');

    // 3. Verify the Action Card is present and visible
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    const feedSection = page.locator('#unified-agent-feed-section').first();
    await expect(feedSection).toBeVisible();

    // Verify the context and intent
    const card = feedSection.locator('.glassmorphism', { hasText: /at-risk subscriber|winback|Alex/i }).first();
    await expect(card).toBeVisible({ timeout: 15000 });

    await expect(card).toContainText('Health Score: 30');
    await expect(card).toContainText("Hi Alex, we noticed you haven't booked a lesson in a few weeks");

    // 4. Owner approves the drafted message
    const approveButton = card.locator('button', { hasText: 'Approve' }).first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // 5. Verify the card moves out of the pending list
    // (In offline queue/optimistic UI it will disappear right away)
    await expect(card).not.toBeVisible();

    // Switch to Activity Feed to see the approved action
    const activityFeedTab = page.locator('button', { hasText: 'Activity Feed' }).first();
    await activityFeedTab.click();

    // It should show up as APPROVED
    const activityItem = page.locator('.glassmorphism', { hasText: /APPROVED/i }).first();
    await expect(activityItem).toBeVisible({ timeout: 10000 });
  });
});
