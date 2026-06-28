import { test, expect } from '@playwright/test';

test.describe('Agentic Unified Intake & Action Feed', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display agent feed and process actions', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email').fill('admin@ohc.local');
    await page.getByPlaceholder('Password').fill('admin');
    await page.getByRole('button', { name: 'Sign In' }).click();
    await page.waitForURL('**/dashboard**');

    await page.goto('/feed');

    // Let's create an ambassador draft to test
    await page.getByTestId('simulate-ambassador-btn').click();

    const feedCard = page.getByTestId('agent-feed-card').filter({ hasText: 'CUSTOMER MESSAGE' }).first();
    await expect(feedCard).toBeVisible({ timeout: 10000 });

    // Validate 375px rendering / UI layout by clicking buttons
    const editBtn = feedCard.getByTestId('feed-edit-btn').first();
    await expect(editBtn).toBeVisible();
    await editBtn.click();

    const editInput = feedCard.getByTestId('feed-edit-input');
    await expect(editInput).toBeVisible();
    await editInput.fill('Updated text from e2e test with new text');

    const saveBtn = feedCard.getByTestId('feed-save-edit-btn');
    await expect(saveBtn).toBeVisible();
    await saveBtn.click();

    const approveBtn = feedCard.getByTestId('feed-approve-btn').first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Card should disappear after successful approve
    await expect(feedCard).not.toBeVisible({ timeout: 5000 });
  });
});
