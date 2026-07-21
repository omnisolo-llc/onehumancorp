import { test as base, expect } from './fixtures';

const test = base.extend({
  page: async ({ page }, use) => {
    // Mobile viewport
    await page.setViewportSize({ width: 375, height: 812 });
    await use(page);
  }
});

test.describe('Unified Action Feed e2e', () => {
  test('Unified Feed - Tap to approve user journey', async ({ adminUser, loginAs, page, request }) => {
    // Setup test data directly using the API
    const feedItemPayload = {
      tenant_id: adminUser.tenantId,
      event_source: "Instagram DM",
      context_payload: {
        msg: "Can I get a custom cake next Tuesday?",
      },
      proposed_action: {
        action_type: "Draft Reply",
        draft_reply: "Hi! Yes, we have availability next Tuesday. The base price for a custom cake is $150. Would you like to proceed with a $50 deposit?"
      },
      lifecycle_state: "PENDING"
    };

    const res = await request.post('/api/v1/agent-feed', {
      data: feedItemPayload
    });
    expect(res.ok()).toBeTruthy();

    await loginAs(page, adminUser);

    // We expect to land on unified feed or can navigate there
    await page.goto('/dashboard');
    await expect(page.getByText("Today's Triage Center")).toBeVisible({ timeout: 15000 });

    // Wait for the feed item to load
    await expect(page.getByText('Can I get a custom cake next Tuesday?')).toBeVisible({ timeout: 15000 });

    // Assert the draft is visible
    await expect(page.getByText('Would you like to proceed with a $50 deposit?')).toBeVisible();

// 3. Swipe to approve
    const cardToSwipe = page.locator('section[aria-label="Unified Agent Feed"] > div:last-child > div').first();
    if (await cardToSwipe.isVisible()) {
        const box = await cardToSwipe.boundingBox();
        if (box) {
           await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
           await page.mouse.down();
           await page.mouse.move(box.x + box.width / 2 + 150, box.y + box.height / 2, { steps: 5 });
           await page.mouse.up();
        }
    }

    // The item should disappear from the list (or show a success message)
    await expect(page.getByText('Can I get a custom cake next Tuesday?')).not.toBeVisible({ timeout: 5000 });
    // await expect(page.getByText('All caught up!')).toBeVisible();
  });

  test('Unified Feed - Tap to reject user journey', async ({ adminUser, loginAs, page, request }) => {
    // Setup test data directly using the API
    const feedItemPayload = {
      tenant_id: adminUser.tenantId,
      event_source: "Operations",
      context_payload: {
        msg: "Staffing alert: Only 1 person scheduled for closing shift.",
      },
      proposed_action: {
        action_type: "Draft Request",
        draft_reply: "Hey team, we need one more person for the closing shift tonight. Anyone available?"
      },
      lifecycle_state: "PENDING"
    };

    const res = await request.post('/api/v1/agent-feed', {
      data: feedItemPayload
    });
    expect(res.ok()).toBeTruthy();

    await loginAs(page, adminUser);

    // We expect to land on unified feed or can navigate there
    await page.goto('/dashboard');
    await expect(page.getByText("Today's Triage Center")).toBeVisible({ timeout: 15000 });

    // Wait for the feed item to load
    await expect(page.getByText('Staffing alert: Only 1 person scheduled for closing shift.')).toBeVisible({ timeout: 15000 });

// 3. Swipe to reject
    const cardToSwipe = page.locator('section[aria-label="Unified Agent Feed"] > div:last-child > div').first();
    if (await cardToSwipe.isVisible()) {
        const box = await cardToSwipe.boundingBox();
        if (box) {
           await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
           await page.mouse.down();
           await page.mouse.move(box.x + box.width / 2 - 150, box.y + box.height / 2, { steps: 5 });
           await page.mouse.up();
        }
    }

    // The item should disappear from the list (or show a success message)
    await expect(page.getByText('Staffing alert: Only 1 person scheduled for closing shift.')).not.toBeVisible({ timeout: 5000 });
    // await expect(page.getByText('All caught up!')).toBeVisible();
  });
});
