import { test, expect } from '@playwright/test';

test.describe('Ambassador Agent Workflow', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('simulate ambassador draft, verify in feed, edit, and approve', async ({ page }) => {
    // We will just hit the real UI here but it needs the backend running.
    // If the backend is mocked, we can do it via routes.
    // Assuming backend is running because of `bazel test //src/e2e:playwright` wrapper.

    // 1. Navigate to the agent feed
    await page.goto('/feed');

    // Ensure we are caught up initially, or just click the button right away
    // 2. Trigger the simulation
    await page.getByTestId('simulate-ambassador-btn').click();

    // 3. Verify the action card appears
    const feedCard = page.getByTestId('agent-feed-card').first();
    await expect(feedCard).toBeVisible({ timeout: 10000 });

    // Verify specific Ambassador UI elements
    await expect(feedCard).toContainText('CUSTOMER MESSAGE');
    await expect(feedCard).toContainText('New Message from @customer');
    await expect(feedCard).toContainText('Do you have vegan chocolate cake available for Saturday?');
    await expect(feedCard).toContainText('Agent Draft');
    await expect(feedCard).toContainText('Yes we do! We have 3 left for this Saturday');
    await expect(feedCard).toContainText('Returning Customer (2 past orders).');

    // 4. Click 'Edit'
    const editBtn = feedCard.getByTestId('feed-edit-btn');
    await expect(editBtn).toContainText('Edit');
    await editBtn.click();

    // Verify textarea appears
    const textarea = page.getByTestId('feed-edit-input');
    await expect(textarea).toBeVisible();

    // Modify the text
    await textarea.fill('Yes we do! We have 3 left for this Saturday. I can set aside one for you.');

    const saveBtn = page.getByTestId('feed-save-edit-btn');
    await saveBtn.click();

    // Wait for the textarea to be hidden
    await expect(textarea).not.toBeVisible();
    await expect(feedCard).toContainText('Yes we do! We have 3 left for this Saturday. I can set aside one for you.');

    // 5. Click 'Approve & Send Draft'
    const approveBtn = feedCard.getByTestId('feed-approve-btn');
    await expect(approveBtn).toContainText('✨ Approve & Send Draft');
    await approveBtn.click();

  });
});
