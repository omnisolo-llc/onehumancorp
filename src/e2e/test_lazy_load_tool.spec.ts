import { test, expect } from '@playwright/test';
import { clearAndSeedDatabase, createTestUser } from './db_utils';

test.describe('Lazy Load Tool UI Verification', () => {
  let testUser: any;

  test.beforeAll(async () => {
    await clearAndSeedDatabase();
    testUser = await createTestUser();
  });

  test('Agent should cleanly handle lazy loading invalid tool', async ({ page }) => {
    // Navigate to application
    await page.goto('/');

    // Login
    await page.fill('input[type="email"]', testUser.email);
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign in")');
    await page.waitForURL('/dashboard');

    // Go to Assistant
    await page.click('text="Assistant"');

    // Simulate user prompting agent to use a non-existent tool,
    // requiring the agent to attempt lazy loading it.
    await page.fill('textarea[placeholder*="Ask anything..."]', 'Please lazy load the tool "NonExistentToolThatTriggersLlmRecoverableError"');
    await page.click('button:has-text("Send")');

    // Verify the agent gracefully handles it (by catching the LlmRecoverable error internally
    // and informing the user about the failure, rather than crashing or showing a 500)
    await expect(page.locator('text="The following tools are not available"')).toBeVisible({ timeout: 15000 });
  });
});
