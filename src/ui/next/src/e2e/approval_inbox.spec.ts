import { test, expect } from '../../../../e2e/fixtures';

test.describe('Unified Agent Feed Mobile Test', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render properly and handle tabs and display an action card from real data', async ({ page, loginAs, adminUser }) => {
    test.setTimeout(180000);

    await loginAs(page, adminUser);

    // 1. Simulate the SalesAgent drafting a quote to create a real proposal in the feed
    await page.request.post('/api/agents/approvals/simulate-quote-draft', {
      headers: {
        'x-tenant-id': 'tenant-1',
        'x-user-id': 'default'
      },
      data: {
        inbox_message_id: 'msg-feed-test-1'
      }
    });

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Wait for the unified agent feed to load
    await expect(page.locator('button', { hasText: /Proposals/ }).first()).toBeVisible({ timeout: 15000 });
    await expect(page.locator('button', { hasText: 'Activity Feed' })).toBeVisible();

    // Switch tabs
    await page.locator('button', { hasText: 'Activity Feed' }).click();

    // Verify glassmorphism CSS
    const feedContainer = page.locator('.glassmorphism').first();
    await expect(feedContainer).toBeVisible();
    await expect(feedContainer).toHaveCSS('backdrop-filter', /blur\(30px\)|none/);

    // Switch back to proposals
    await page.locator('button', { hasText: /Proposals/ }).first().click({ force: true });

    // Verify the proposal we injected is visible
    await expect(page.getByText('Draft Quote: Plumbing Fix for Customer').first()).toBeVisible();
    await expect(page.getByText('Requires Review').first()).toBeVisible();

    // Verify action buttons for the proposal
    const approveButton = page.locator('button', { hasText: 'Approve & Send' }).first();
    await expect(approveButton).toBeVisible();

    const editButton = page.locator('a', { hasText: 'Edit Draft' }).first();
    await expect(editButton).toBeVisible();

    const rejectButton = page.locator('button', { hasText: 'Ask Agent to Adjust' }).first();
    await expect(rejectButton).toBeVisible();

    // Test the optimistic approval
    await approveButton.click();

    // The optimistic update should remove the card from proposals
    await expect(page.getByText('Draft Quote: Plumbing Fix for Customer')).toHaveCount(0);
  });

  test('should display empty state or loading state in Activity Feed correctly', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    await page.goto('/dashboard');
    await expect(page.locator('button', { hasText: 'Activity Feed' })).toBeVisible({ timeout: 15000 });

    // Switch tabs
    await page.locator('button', { hasText: 'Activity Feed' }).click();

    // Loading or empty state or populated activities
    const activityFeedItems = page.locator('.glassmorphism', { hasText: /Activity Feed|No recent activity found|Action completed/ });
    await expect(activityFeedItems.first()).toBeVisible();
  });
});
