import { test, expect } from '@playwright/test';

test.describe('The Promoter Agent CUJ', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Owner simulates product creation and schedules generated social posts', async ({ page }) => {
    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // 2. Go to the feed page
    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed')).toBeVisible();

    // 3. Click the Simulate Promoter Draft button
    const simBtn = page.getByTestId('simulate-promoter-btn');
    if (await simBtn.isVisible()) {
      await simBtn.click();
    }

    // 4. Verify the Promoter action card appears
    const feedCard = page.getByTestId('agent-feed-card').first();
    await expect(feedCard).toBeVisible({ timeout: 15000 });

    // 5. Verify the card contains the specific UI elements for Promoter
    await expect(feedCard).toContainText('New Product: New Collection');
    await expect(feedCard).toContainText('Generated Marketing Posts');
    await expect(feedCard).toContainText('Instagram');
    await expect(feedCard).toContainText('TikTok');

    // 6. Test Edit functionality
    const editBtn = feedCard.getByTestId('feed-edit-btn');
    await expect(editBtn).toBeVisible();
    await editBtn.click();

    const textarea = page.getByTestId('feed-edit-input');
    await expect(textarea).toBeVisible();
    await textarea.fill('Testing edit functionality');

    const saveBtn = page.getByTestId('feed-save-edit-btn');
    await saveBtn.click();
    await expect(textarea).not.toBeVisible();

    // 7. Test Approve & Schedule functionality
    const approveBtn = feedCard.getByTestId('feed-approve-btn');
    await expect(approveBtn).toBeVisible();
    await expect(approveBtn).toContainText('Approve & Schedule');

    await approveBtn.click();

    // 8. Verify the card disappears from the feed
    await expect(feedCard).not.toBeVisible({ timeout: 10000 });
  });
});
