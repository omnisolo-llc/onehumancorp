import { test, expect } from './fixtures';

test.describe('Agentic Unified Intake & Action Feed', () => {
  test('should display agent feed and process actions', async ({ page }) => {
    // Rely on real backend and e2e-seed.sql for data.
    // The test fixture logs us in automatically.
    await page.goto('/feed');

    const feedContainer = page.getByTestId('agent-feed');
    await expect(feedContainer).toBeVisible({ timeout: 15000 });

    const feedCard = page.getByTestId('agent-feed-card').first();
    await expect(feedCard).toBeVisible({ timeout: 15000 });

    // Look for a card that has an edit button we can test.
    // Some cards might not have an edit button (like incident_resolution), so we filter.
    const editableCard = page.getByTestId('agent-feed-card').filter({ has: page.getByTestId('feed-edit-btn') }).first();

    if (await editableCard.isVisible()) {
      const editBtn = editableCard.getByTestId('feed-edit-btn');
      await expect(editBtn).toBeVisible();
      await editBtn.click();

      const editInput = editableCard.getByTestId('feed-edit-input');
      await expect(editInput).toBeVisible();

      await editInput.fill('Updated text from e2e test');
      const saveBtn = editableCard.getByTestId('feed-save-edit-btn');
      await expect(saveBtn).toBeVisible();
      await saveBtn.click();

      const approveBtn = editableCard.getByTestId('feed-approve-btn');
      await expect(approveBtn).toBeVisible();
      await approveBtn.click();

      await expect(editableCard).not.toBeVisible({ timeout: 15000 });
    }
  });
});
