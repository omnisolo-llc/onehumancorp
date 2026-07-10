import { test, expect } from '@playwright/test';

test.describe('Agentic Unified Intake & Action Feed', () => {
  test('should display agent feed and process actions', async ({ page }) => {
    // MOCK API if we want to test ui reliably without backend
    await page.route('**/api/agent-feed*', async route => {
        if (route.request().method() === 'GET') {
          await route.fulfill({
            status: 200,
            json: {
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
            }
          });
        } else if (route.request().method() === 'PUT') {
          await route.fulfill({ status: 200, json: { success: true } });
        } else {
          await route.continue();
        }
    });

    await page.goto('/feed');
    // await expect(page.getByTestId('agent-feed')).toBeVisible({ timeout: 15000 });

    const feedCard = page.getByTestId('agent-feed-card').first();

    await expect(feedCard).toBeVisible({ timeout: 15000 });

    const editBtn = feedCard.getByTestId('feed-edit-btn');
    await expect(editBtn).toBeVisible();
    await editBtn.click();

    const editInput = feedCard.getByTestId('feed-edit-input');
    await expect(editInput).toBeVisible();

    await editInput.fill('Updated text from e2e test');
    const saveBtn = feedCard.getByTestId('feed-save-edit-btn');
    await expect(saveBtn).toBeVisible();
    await saveBtn.click();

    const approveBtn = feedCard.getByTestId('feed-approve-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();
  });
});
