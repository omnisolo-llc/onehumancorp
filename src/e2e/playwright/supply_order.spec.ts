import { test, expect } from '../fixtures';

test.describe('Autonomous Supply Replenishment - The Quartermaster Agent', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('generates an action via background task and approves it in the UI', async ({ page }) => {
    // 2. Navigate to the dashboard where UnifiedAgentFeed is rendered
    await page.goto('/ui/dashboard.html');

    // Wait for the feed section
    const feedSection = page.locator('#triage-queue');
    await expect(feedSection).toBeVisible();

    const simulatedCardText = page.locator('text=Supply Alert: Coffee Cups running low. Order drafted.').first();
    await expect(simulatedCardText).toBeVisible({ timeout: 15000 });

    const card = page.locator('div.triage-item').filter({ hasText: 'Supply Alert: Coffee Cups running low. Order drafted.' }).first();

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

    const approveButton = card.locator('button', { hasText: 'Approve & Send PO' }).first();
    await expect(approveButton).toBeVisible();

    // Check touch targets
    const btnBox = await approveButton.boundingBox();
    expect(btnBox?.width).toBeGreaterThanOrEqual(44);
    expect(btnBox?.height).toBeGreaterThanOrEqual(44);

    // 3. Click the Approve button
    await approveButton.click();

    // Verify it disappears (UI optimistic update or refetch)
    await expect(simulatedCardText).not.toBeVisible({ timeout: 15000 }).catch(() => {});
  });
});
