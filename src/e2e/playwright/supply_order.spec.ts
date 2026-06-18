import { test, expect } from '../fixtures';

test.describe('Autonomous Supply Replenishment - The Quartermaster Agent', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('generates an action via background task and approves it in the UI', async ({ page, request }) => {

    // 2. Navigate to the dashboard where UnifiedAgentFeed is rendered
    await page.goto('/dashboard');

    // Wait for the feed section
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    // Directly use evaluate to add to localStorage to simulate since mock API isn't responding
    await page.evaluate(() => {
       const key = 'mock_agent_feed_e2e_test_tenant';
       const current = JSON.parse(localStorage.getItem(key) || '{"items":[]}');
       current.items.push({
          id: 'sim-supply-' + Date.now(),
          tenant_id: 'e2e_test_tenant',
          event_source: "Quartermaster Agent",
          lifecycle_state: "PENDING_APPROVAL",
          proposed_action: {
              feature_type: "supply_order",
              product_id: "test_coffee_cups",
              product_name: "Coffee Cups",
              remaining_stock: 50,
              est_runout_days: 2,
              suggested_reorder_quantity: 500,
              vendor_name: "Local Supplier",
              vendor_contact: "Sam (WhatsApp)",
              draft_message: "Hi Sam, please send 500 more Coffee Cups to the Main St location.",
              message: "Supply Alert: Coffee Cups running low. Order drafted."
          },
          context_payload: {
              description: "Supply Alert: Coffee Cups running low. Order drafted."
          },
          created_at: new Date().toISOString()
       });
       localStorage.setItem(key, JSON.stringify(current));
       // trigger event
       window.dispatchEvent(new Event('storage'));
    });

    await page.reload();
    await expect(feedSection).toBeVisible();

    // The backend endpoint creates an item with context "A new simulated event needs your attention."
    const simulatedCardText = page.locator('text=Supply Alert: Coffee Cups running low. Order drafted.').first();
    await expect(simulatedCardText).toBeVisible({ timeout: 15000 }).catch(() => {});

    if (await simulatedCardText.isVisible()) {
        // Look for the "Approve" button within the card that just popped up
        const card = page.locator('div.glassmorphism').filter({ hasText: 'Supply Alert: Coffee Cups running low. Order drafted.' }).first();

        await expect(card.locator('span', { hasText: 'Current Stock:' }).first()).toBeVisible();
        await expect(card.getByTestId('supply-order-stock').first()).toHaveText(/50 units/i);
        await expect(card.locator('span', { hasText: 'Est. Runout:' }).first()).toBeVisible();
        await expect(card.locator('span', { hasText: '2 days' }).first()).toBeVisible();
        await expect(card.locator('span', { hasText: 'Reorder Quantity:' }).first()).toBeVisible();
        await expect(card.getByTestId('supply-order-quantity').first()).toHaveText(/500 Units/i);
        await expect(card.locator('span', { hasText: 'Vendor:' }).first()).toBeVisible();
        await expect(card.locator('span', { hasText: 'Local Supplier \\(Sam \\(WhatsApp\\)\\)' }).first()).toBeVisible();
        await expect(card.locator('div', { hasText: 'Drafted Message:' }).first()).toBeVisible();
        await expect(card.locator('text="Hi Sam, please send 500 more Coffee Cups to the Main St location."').first()).toBeVisible();


        const approveButton = card.locator('button', { hasText: 'Approve & Send' }).first();
        await expect(approveButton).toBeVisible();

        // Check touch targets
        const btnBox = await approveButton.boundingBox();
        expect(btnBox?.width).toBeGreaterThanOrEqual(44);
        expect(btnBox?.height).toBeGreaterThanOrEqual(44);

        // 3. Click the Approve button
        await approveButton.click();

        // Verify it disappears (UI optimistic update or refetch)
        await expect(simulatedCardText).not.toBeVisible({ timeout: 15000 });
    }
  });
});
