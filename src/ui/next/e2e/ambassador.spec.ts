import { test, expect } from '@playwright/test';
import { clearDatabase, login } from './fixtures/utils';

test.describe('The Ambassador Agent Flow', () => {
  test.beforeEach(async () => {
    await clearDatabase();
  });

  test('receives instagram DM, generates draft, and allows owner to approve', async ({ page, request }) => {
    const tenantId = 'ambassador-tenant-123';

    // 1. Simulate an incoming message via the mock API (simulate webhook)
    const mockRes = await request.post(`/api/dev/mock-ambassador?tenant_id=${tenantId}`, {
      data: {
        source: 'instagram',
        sender_id: 'maya_baker',
        message: 'Do you have vegan options for birthday cakes?'
      }
    });
    expect(mockRes.ok()).toBeTruthy();
    const data = await mockRes.json();
    expect(data.success).toBe(true);

    // 2. Login as the owner
    await login(page, tenantId, 'owner_user');

    // 3. Navigate to the agent feed (dashboard)
    await page.goto('/dashboard');

    // Wait for feed to load and display the new ambassador draft action card
    await page.waitForSelector('[data-testid="ambassador-draft-card"]');

    // Check if the card has the customer message
    await expect(page.getByText('Customer: Do you have vegan options for birthday cakes?')).toBeVisible();

    // Check if the draft reply exists
    await expect(page.getByText('Draft:')).toBeVisible();

    // 4. Owner approves the draft reply
    await page.click('[data-testid="approve-ambassador-draft"]');

    // 5. The card should disappear after approval (optimistic update or refetch)
    await expect(page.getByTestId('ambassador-draft-card')).toHaveCount(0);
  });
});
