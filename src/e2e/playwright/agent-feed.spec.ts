import { test, expect } from '@playwright/test';
import path from 'path';

test.describe('Unified Agent Feed (Mobile MVP)', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('displays action cards and allows approval without horizontal scrolling', async ({ page }) => {
    await page.goto(`file://${path.resolve('src/ui/tauri/src/ui/dashboard.html')}`);

    await page.waitForSelector('section[aria-label="Unified Agent Feed"]');
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    await page.evaluate(() => {
        if (window.loadTriage) {
          window.agentFeedItems = [
              {
                  id: "test-feed-1",
                  event_source: "instagram_dm",
                  priority: "High",
                  context_payload: {
                      customer_message: "Do you make custom vegan cakes?",
                      draft_reply: "Yes we do! Here is a booking link: https://ohc.page/book"
                  }
              }
          ];
          window.renderTriageItems(window.agentFeedItems);
        }
    });

    // Wait for feed items to load
    await page.waitForSelector('.triage-item');

    // Ensure there is no horizontal scroll
    const clientWidth = await page.evaluate(() => document.documentElement.clientWidth);
    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    expect(scrollWidth).toBeLessThanOrEqual(clientWidth);

    const cards = page.locator('.triage-item');

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

      const firstApproveButton = buttons.filter({ hasText: 'Approve & Send' }).first();

      if (await firstApproveButton.isVisible()) {
        // mock handleTriageAction instead of API since it's a file:// protocol test
        await page.evaluate(() => {
           window.handleTriageAction = function(id, state) {
              const itemEl = document.getElementById(`triage-${id}`);
              if (itemEl) itemEl.style.display = 'none';
           }
        });
        await firstApproveButton.click();
        await expect(cards.first()).not.toBeVisible();
      }
    }
  });
});
