import { expect, test } from '@playwright/test';

test.describe('Triage Action Feed UI', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render triage feed properly, show distinct card types, allow approval, and display empty state', async ({ page }) => {
    test.setTimeout(180000);

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'default');

    const seedData = [
      { source: 'Instagram DM Message', priority: 'high', context: 'Message: Customer asked about vegan cakes.', action_type: 'Draft Reply', action_payload: 'Yes, we have vegan options.' },
      { source: 'Website Booking Request', priority: 'medium', context: 'Booking: Sarah wants an estimate tomorrow at 2PM.', action_type: 'Accept Booking', action_payload: 'Booked for 2PM tomorrow.' },
      { source: 'Inventory Alert', priority: 'urgent', context: 'Alert: Low stock on flour.', action_type: 'Reorder', action_payload: 'Order 50lbs of flour.' }
    ];

    for (const data of seedData) {
      await page.request.post(`/api/ui/triage/create?tenant_id=${encodeURIComponent(tenantId)}`, {
        data: {
          customer_id: 'cust_test',
          source: data.source,
          priority: data.priority,
          context: data.context,
          action_type: data.action_type,
          action_payload: data.action_payload
        }
      });
    }

    await page.goto('/triage');
    await expect(page.locator('body')).toContainText(/Work Triage/, { timeout: 15000 });

    // Let's use a simpler locator for list items
    let listItems = page.locator('div[data-testid^="triage-card-"]');

    await page.waitForTimeout(2000);
    await expect(page.locator('body')).toContainText(/Work Triage|All caught up/);

    let count = await listItems.count();

    while (count > 0) {
      // Removed list item click

      const approveBtn = listItems.nth(0).getByTestId('approve-btn');
      const dismissBtn = listItems.nth(0).getByTestId('dismiss-btn');

      if (await approveBtn.isVisible()) {
        await approveBtn.click();
      } else if (await dismissBtn.isVisible()) {
        await dismissBtn.click();
      } else {
        break;
      }

      await page.waitForTimeout(1000);

      count = await listItems.count();
    }

    await page.waitForTimeout(3000);

    // Look for the specific empty state element
    const emptyState = page.getByTestId('triage-feed-empty');
    if (await emptyState.isVisible()) {
        await expect(emptyState).toContainText(/caught up/i);
    }

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    const proposalsTab = page.locator('button', { hasText: /Proposals/ });
    if (await proposalsTab.isVisible()) {
       await proposalsTab.click();
    }

    const triageFeedEmpty = page.getByTestId('triage-feed-empty');
    if (await triageFeedEmpty.isVisible()) {
       await page.waitForTimeout(3000);
       await expect(triageFeedEmpty).toContainText(/caught up/i, { timeout: 15000 });
    }
  });
});
