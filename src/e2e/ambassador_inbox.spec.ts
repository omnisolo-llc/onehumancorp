import { test, expect } from './fixtures';

/**
 * Persona: Maya the Baker
 * Business: Maya's Custom Cakes
 * CUJ: Managing customer inquiries across channels with AI assistance.
 */
test.describe('Ambassador Inbox CUJ', () => {
  test('Maya can manage an Instagram inquiry using AI Ambassador', async ({ page }) => {
    // 1. Start from Dashboard (handled by loginAs in fixture)
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // 2. Navigate to Inbox via Sidebar/Nav
    // Based on the UI, we might have a link or we can go directly to /inbox
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    // 3. Verify the AI Ambassador status is visible and "Active" by default
    const aiStatus = page.getByText('AI Ambassador: Active');
    await expect(aiStatus).toBeVisible();

    // 4. Select a conversation from the list (e.g., Sarah J. via Instagram)
    const conversationCard = page.getByRole('button', { name: /Sarah J./i });
    await expect(conversationCard).toBeVisible();
    await conversationCard.click();

    // 5. Verify the thread view opens with customer message
    await expect(page.getByText('Do you have vegan options for birthday cakes?')).toBeVisible();

    // 6. Verify the AI Ambassador has drafted a response (indicated by "✨ Ambassador")
    await expect(page.getByText('✨ Ambassador')).toBeVisible();
    await expect(page.getByText('Yes, we have several vegan birthday cake options available!')).toBeVisible();

    // 7. Approve the AI draft
    const approveButton = page.getByRole('button', { name: 'Approve' });
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // 8. Verify the message is now sent (the "Approve" button should disappear, replaced by "Me" or similar role signature)
    await expect(approveButton).not.toBeVisible();
    await expect(page.getByText('Me •')).toBeVisible();

    // 9. Maya sends a manual follow-up message
    const textarea = page.getByPlaceholder('Type a message...');
    await textarea.fill('I can also do gluten-free if you need!');
    await page.keyboard.press('Enter');

    // 10. Verify manual message appears in the thread
    await expect(page.getByText('I can also do gluten-free if you need!')).toBeVisible();

    // 11. Test AI Toggle functionality
    await page.getByRole('button', { name: 'Toggle' }).click();
    await expect(page.getByText('AI Ambassador: Paused')).toBeVisible();
  });

  test('Mobile Responsiveness: Maya manages inbox on her iPhone (375px)', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/inbox');

    // In mobile view, only the conversation list should be visible initially
    const conversationCard = page.getByRole('button', { name: /Sarah J./i });
    await expect(conversationCard).toBeVisible();

    // Click a conversation
    await conversationCard.click();

    // The thread view should now be visible, and the list should be hidden (md:hidden logic)
    await expect(page.getByText('Do you have vegan options for birthday cakes?')).toBeVisible();

    // There should be a back button to return to the list
    const backButton = page.locator('button').filter({ has: page.locator('svg') }).first(); // The chevron-left we added
    await backButton.click();

    // List should be visible again
    await expect(conversationCard).toBeVisible();
  });

  test('Maya can toggle AI Ambassador status', async ({ page }) => {
    await page.goto('/inbox');

    const statusText = page.getByText(/AI Ambassador:/);
    await expect(statusText).toContainText('Active');

    await page.getByRole('button', { name: 'Toggle' }).click();
    await expect(statusText).toContainText('Paused');

    await page.getByRole('button', { name: 'Toggle' }).click();
    await expect(statusText).toContainText('Active');
  });

  test('Maya can switch between multiple conversations', async ({ page }) => {
    await page.goto('/inbox');

    // Select first conversation
    await page.getByRole('button', { name: /Sarah J./i }).click();
    await expect(page.getByRole('heading', { name: 'Sarah J.' })).toBeVisible();
    await expect(page.getByText('Do you have vegan options for birthday cakes?')).toBeVisible();

    // Select second conversation
    await page.getByRole('button', { name: /15550102030/i }).click();
    await expect(page.getByRole('heading', { name: '15550102030' })).toBeVisible();
    await expect(page.getByText('When will my order be shipped?')).toBeVisible();
  });

  test('Maya can search for a specific conversation', async ({ page }) => {
    await page.goto('/inbox');

    const searchInput = page.getByPlaceholder('Search conversations...');
    await searchInput.fill('Alex');

    // Only Alex should be visible
    await expect(page.getByRole('button', { name: /Alex Miller/i })).toBeVisible();
    await expect(page.getByRole('button', { name: /Sarah J./i })).not.toBeVisible();
  });

  test('Maya can edit an AI draft before sending', async ({ page }) => {
    await page.goto('/inbox');
    await page.getByRole('button', { name: /Sarah J./i }).click();

    const editButton = page.getByRole('button', { name: 'Edit' });
    await editButton.click();

    const textarea = page.locator('textarea').filter({ hasText: /Yes, we have several/ });
    await textarea.fill('Actually, we are sold out of vegan cakes today.');

    await page.getByRole('button', { name: 'Save & Send' }).click();

    await expect(page.getByText('Actually, we are sold out of vegan cakes today.')).toBeVisible();
    await expect(page.getByText('Me •')).toBeVisible();
  });
});
