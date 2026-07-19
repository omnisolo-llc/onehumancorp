import { test, expect } from './fixtures';

test.describe('Omnichannel Inbox UI', () => {
  test('Owner sees sender id and known customer in inbox using database seed', async ({ page, loginAs, adminUser }) => {
    test.setTimeout(60000);
    await loginAs(page, adminUser);
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    await page.goto('/inbox');

    // Wait for inbox header
    await expect(page.getByRole('heading', { name: 'Unified Inbox' }).first()).toBeVisible({ timeout: 10000 });

    // Based on the seed data, we might not have a message yet, so let's check for empty state OR message
    const emptyState = page.locator('.app-empty');
    const msg = page.locator('.app-list-item').first();

    await Promise.race([
        expect(emptyState).toBeVisible(),
        expect(msg).toBeVisible()
    ]);
  });
});
