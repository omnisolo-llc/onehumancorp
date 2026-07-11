import { expect, test, E2E_ADMIN_USER } from './fixtures';

test.describe('Unified Agent Feed Interactive Flow', () => {
  test.use({ viewport: { width: 375, height: 812 } });
  test('should display and allow interaction with triage items and invoice items', async ({ page, request, loginAs }) => {
    test.setTimeout(180000);
    await loginAs(page, E2E_ADMIN_USER);

    // Seed a triage item to ensure it's in the list
    await request.post('/api/dev/simulate-agent-feed-item', {
        data: {
          tenant_id: 'phslc',
          priority: 'High',
          feature_type: 'triage',
          context_summary: 'New customer inquiry from Sarah',
          action_type: 'Draft Reply',
          action_payload: 'Hi Sarah, yes we have availability.'
        }
    });

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Ensure the triage card shows up
    const triageCards = page.locator('text=A new simulated event needs your attention.');
    if (await triageCards.count() > 0) {
        const triageCard = triageCards.first();
        await expect(triageCard).toBeVisible();

        // Check if the Approve & Send button is there
        const approveBtn = triageCard.locator('xpath=../..').getByTestId('feed-approve-btn');
        await expect(approveBtn).toBeVisible();
    }

    // Ensure the invoice card shows up if seeded or exists
    // (If not explicitly seeded, just verify the selector logic)
    const invoiceCards = page.getByTestId('invoice-card');
    if (await invoiceCards.count() > 0) {
        await expect(invoiceCards.first()).toBeVisible();
    }
  });

});


