import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Agent Action Feed', () => {
  test('Action card transitions from PENDING_APPROVAL to APPROVED', async ({ page }) => {
    // Navigate to the action feed
    await page.goto('/action-feed');

    // Check if the "Agent Action Feed" header exists
    await expect(page.locator('h1', { hasText: 'Agent Action Feed' })).toBeVisible();

    // In a real DB with seed data there will be cards, but E2E might have empty state initially
    // We'll simulate creating an action card via API first
    const tenantId = 'e2e-tenant';
    const response = await page.request.post(`/api/v1/action-feed/${tenantId}/cards`, {
      data: {
        tenant_id: tenantId,
        agent_id: 'Marketing',
        trigger_event: 'System Check',
        context_summary: 'This is an E2E test action card',
        proposed_action: { "action": "Run E2E Validation" }
      }
    });

    expect(response.status()).toBe(201);

    // Refresh to see the new card
    await page.reload();

    // Verify card is visible
    const cardContext = page.locator('h3', { hasText: 'System Check' }).first();
    await expect(cardContext).toBeVisible();

    // Get the card container to find the approve button
    const card = cardContext.locator('..').locator('..');
    const approveButton = card.locator('button', { hasText: 'Approve & Execute' });

    // Tap approve
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // The card should disappear after being approved
    await expect(cardContext).not.toBeVisible();
  });
});
