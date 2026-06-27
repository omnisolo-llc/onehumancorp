import { test, expect } from '@playwright/test';
import { e2eTenantContext } from './fixtures';

test.describe('Autonomous Staff Scheduling & Task Orchestration', () => {
  const STAFF_PHONE = '+15550101010'; // Matching the seed data

  test('Manager can approve a shift reassignment from Twilio SMS call-out', async ({ page, request }) => {

    // Simulate the incoming Twilio Webhook from a staff member
    const webhookResponse = await request.post('/api/v1/webhooks/twilio', {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      data: `From=${encodeURIComponent(STAFF_PHONE)}&To=${encodeURIComponent('+15550109999')}&Body=${encodeURIComponent("I'm sick and can't make my shift tomorrow.")}`
    });

    expect(webhookResponse.ok()).toBeTruthy();

    // The Triage Worker will parse this, determine it's a shift callout, and add a PENDING_APPROVAL action to the agent feed.
    // Wait for async processing
    await page.waitForTimeout(4000);

    // Login and navigate to the agent feed
    await page.goto('/login');
    await page.fill('input[name="email"]', 'starter@example.com');
    await page.fill('input[name="password"]', 'password123');
    await page.click('button[type="submit"]');
    await page.waitForURL('**/dashboard');

    // Make sure we are on the dashboard viewing the Agent Feed
    const agentFeed = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(agentFeed).toBeVisible({ timeout: 15000 });

    // Ensure the action card appears
    // We expect the AI to draft a reassignment action based on our prompt changes.
    const approveBtn = page.getByTestId('approve-shift-btn');
    await expect(approveBtn).toBeVisible({ timeout: 15000 });

    const aiProposalText = page.getByText('AI Proposal', { exact: false });
    await expect(aiProposalText).toBeVisible();

    // Click "Approve & Notify"
    await approveBtn.click();

    // Verify it disappears (or updates status)
    await expect(approveBtn).not.toBeVisible({ timeout: 5000 });

    // Also check that it might appear in the Activity Feed as APPROVED
    await page.getByText('Activity Feed').click();
    await expect(page.getByText('APPROVED', { exact: true }).first()).toBeVisible({ timeout: 5000 });
  });
});
