import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed Mobile-First UX', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test.beforeEach(async ({ page }) => {
    // Mock the agent-feed API to provide consistent data for testing
    await page.route('/api/agent-feed*', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          items: [
            {
              id: 'test-ops-1',
              tenant_id: 'default',
              event_source: 'inventory_agent',
              lifecycle_state: 'PENDING_APPROVAL',
              proposed_action: { message: 'Low stock on Flour: 5kg left.' },
              context_payload: { product: 'Flour', remaining: '5kg' },
              created_at: new Date().toISOString(),
              updated_at: new Date().toISOString(),
            },
            {
              id: 'test-cs-1',
              tenant_id: 'default',
              event_source: 'cs_agent',
              lifecycle_state: 'PENDING_APPROVAL',
              proposed_action: { message: 'Drafted reply to Sarah about cake delivery.' },
              context_payload: { customer: 'Sarah', subject: 'Delivery' },
              created_at: new Date().toISOString(),
              updated_at: new Date().toISOString(),
            },
          ],
        }),
      });
    });

    await page.goto('/dashboard');
  });

  test('should display Unified Agent Feed at the top on mobile', async ({ page }) => {
    // The feed should be visible
    const feed = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feed).toBeVisible();

    // Check for 375px constraints (no horizontal scroll)
    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    expect(scrollWidth).toBe(375);
  });

  test('should have 44x44px touch targets for action buttons', async ({ page }) => {
    const approveBtn = page.locator('[data-testid="action-card-test-ops-1"] button[aria-label="Approve"]');
    const editBtn = page.locator('[data-testid="action-card-test-ops-1"] button[aria-label="Edit"]');
    const discardBtn = page.locator('[data-testid="action-card-test-ops-1"] button[aria-label="Discard"]');

    const approveBox = await approveBtn.boundingBox();
    const editBox = await editBtn.boundingBox();
    const discardBox = await discardBtn.boundingBox();

    expect(approveBox?.height).toBeGreaterThanOrEqual(44);
    expect(approveBox?.width).toBeGreaterThanOrEqual(44);
    expect(editBox?.height).toBeGreaterThanOrEqual(44);
    expect(discardBox?.height).toBeGreaterThanOrEqual(44);
  });

  test('should expand card when Edit is clicked', async ({ page }) => {
    const card = page.locator('[data-testid="action-card-test-ops-1"]');
    const editBtn = card.locator('button[aria-label="Edit"]');

    await expect(card.locator('text=Context & Details')).not.toBeVisible();
    await editBtn.click();
    await expect(card.locator('text=Context & Details')).toBeVisible();
    await expect(card.locator('text=Product')).toBeVisible();
    // Use first() to avoid strict mode violation if "Flour" appears in title and context
    await expect(card.locator('text=Flour').first()).toBeVisible();
  });

  test('should handle Approve action', async ({ page }) => {
    // Mock the approval API
    await page.route('/api/agents/approvals/test-ops-1', async (route) => {
      await route.fulfill({ status: 200, body: JSON.stringify({ success: true }) });
    });

    const card = page.locator('[data-testid="action-card-test-ops-1"]');
    const approveBtn = card.locator('button[aria-label="Approve"]');

    await approveBtn.click();
    // Card should be removed from the list (optimistic update)
    await expect(card).not.toBeVisible();
  });
});
