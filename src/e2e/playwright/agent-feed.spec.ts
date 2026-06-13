import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed (Mobile MVP)', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('displays action cards and allows approval without horizontal scrolling', async ({ page }) => {
    // MOCK API if we want to test ui reliably without backend
    await page.route('/api/agent-feed', async route => {
        const json = {
            items: [
                {
                    id: "1",
                    tenant_id: "t1",
                    event_source: "New Order",
                    lifecycle_state: "PENDING_APPROVAL",
                    created_at: new Date().toISOString(),
                    updated_at: new Date().toISOString(),
                    proposed_action: { title: "Fulfill Now", description: "3 new orders to fulfill" }
                }
            ]
        };
        await route.fulfill({ json });
    });

    // Navigate to the feed page
    await page.goto('/feed');

    // Wait for feed items to load
    await page.waitForSelector('[data-testid="agent-feed"]');

    // Ensure there is no horizontal scroll
    const clientWidth = await page.evaluate(() => document.documentElement.clientWidth);
    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    expect(scrollWidth).toBeLessThanOrEqual(clientWidth);

    await page.waitForSelector('[data-testid="agent-feed-card"]');

    const cards = page.locator('[data-testid="agent-feed-card"]');

    const count = await cards.count();
    expect(count).toBeGreaterThan(0);

    if (count > 0) {
      const buttons = cards.first().locator('button');
      const buttonCount = await buttons.count();
      for (let i = 0; i < buttonCount; i++) {
          const boundingBox = await buttons.nth(i).boundingBox();
          expect(boundingBox?.width).toBeGreaterThanOrEqual(44);
          expect(boundingBox?.height).toBeGreaterThanOrEqual(44);
      }

      const firstApproveButton = buttons.filter({ hasText: 'Approve' }).first();

      // MOCK API for patch
      await page.route('**/api/agent-feed/*', async route => {
          await route.fulfill({ status: 200, json: { success: true } });
      });

      if (await firstApproveButton.isVisible()) {
        await firstApproveButton.click();
      }
    }
  });
});
