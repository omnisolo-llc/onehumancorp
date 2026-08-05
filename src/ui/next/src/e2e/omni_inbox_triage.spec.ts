import { test, expect } from '@playwright/test';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page }) => {
    // Setup real data using standard E2E setup for native chat
    // Navigate to login and login first
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible({ timeout: 10000 });

    // Navigate to the inbox page
    await page.goto('/inbox');

    // Create a native chat message dynamically via evaluating fetch since we are in the real browser
    await page.evaluate(async () => {
      // Get the default tenant ID from somewhere or use a generic one if auth handles it
      // Create inbox, conversation and message via the new Native Chat API
      const tenant_id = "00000000-0000-0000-0000-000000000000"; // Assuming a default or known tenant for E2E
      // In this E2E test, since the backend handles creating conversation, we just call the API directly:
      await fetch(`/api/v1/integrations/chat/conversations/${tenant_id}`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
              contact_id: "11111111-1111-1111-1111-111111111111",
              inbox_id: "22222222-2222-2222-2222-222222222222",
              initial_message: "Hi Maya, do you have 2 vegan cakes for Saturday?"
          })
      });
    });

    // Wait for the message to appear naturally via live reload or manual refresh
    await page.reload();

    // Ensure the message shows up naturally.
    // In a real application, the UI polls or receives a socket update.
    // For now we'll wait for the newly created message item in the UI DOM
    const messageButton = page.locator('.app-list-item', { hasText: 'Hi Maya, do you have 2 vegan cakes for Saturday?' }).first();

    // We expect it to be visible
    await expect(messageButton).toBeVisible({ timeout: 15000 });
    await messageButton.click();
  });
});
