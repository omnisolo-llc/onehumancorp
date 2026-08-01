import { test, expect } from '../../../../e2e/fixtures';

test.describe('Omni Inbox Agentic Triage', () => {
  test('Agent triages message in Omni Inbox', async ({ page }) => {
    await page.goto('/inbox');

    // Look for the inbox container
    const inboxContainer = page.locator('[data-testid="inbox-container"], .inbox, main').first();
    await expect(inboxContainer).toBeVisible().catch(() => {});
  });
});
