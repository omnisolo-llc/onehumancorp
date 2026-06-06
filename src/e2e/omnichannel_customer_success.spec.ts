import { expect, test } from './fixtures';

test.describe('Omnichannel Customer Success Agent', () => {
  test('A user logs in, sees the drafted message card on the mobile-sized feed, taps "Approve," and the system dispatches the message back', async ({ page, request }) => {
    // Required to trigger mobile layout
    await page.setViewportSize({ width: 375, height: 812 });

    const tenantId = 'team-default';
    const messageContent = 'Hi Maya, do you still have vegan cake available?';

    // 1. Simulate incoming message webhook
    const webhookResponse = await request.post('/api/agents/webhook', {
      data: {
        tenant_id: tenantId,
        source: 'instagram_dm',
        message: messageContent
      }
    });

    expect(webhookResponse.ok()).toBeTruthy();

    // 2. Go to the dashboard
    await page.goto('/dashboard');

    // Wait for the agent worker to pick up and process the task (background polling)
    await page.waitForTimeout(4000);

    // 3. Navigate to Team page
    await page.goto('/team');
    await expect(page.getByRole('heading', { name: 'Your Team' })).toBeVisible();

    // 4. Click on the Customer Success department card
    await page.getByText('Customer Success').click();

    // 5. Check for the drafted reply in the inbox
    await expect(page.getByText('Customer Inquiry')).toBeVisible();
    await expect(page.getByText(messageContent)).toBeVisible();
    await expect(page.getByText('AI Draft')).toBeVisible();

    // 6. Click Approve button
    const approveButton = page.getByRole('button', { name: 'Approve' });
    await expect(approveButton.first()).toBeVisible();
    await approveButton.first().click();

    // 7. Wait and verify it was removed from pending
    await page.waitForTimeout(2000);
    await expect(page.getByText(messageContent)).toBeHidden();
  });
});
