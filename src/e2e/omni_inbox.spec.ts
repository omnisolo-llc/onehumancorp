import { test, expect } from './fixtures';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
  test('displays the database-backed inbox experience', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    await expect(page.getByText('Message Queue')).toBeVisible();
    await expect(page.getByText('Conversation Detail')).toBeVisible();
    await expect(page.getByText('Loaded from `/api/ui/inbox/messages`')).toBeVisible();

  });

  test('displays a drafted message and allows sending', async ({ page, request }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'demo@onehumancorp.com');
    await page.fill('input[type="password"]', 'demo123');
    await page.click('button:has-text("Log In")');
    await page.waitForURL('/dashboard');

    await page.goto('/inbox');
    await expect(page.getByText('Message Queue')).toBeVisible();

    await expect(page.getByText('Do you do vegan cakes?')).toBeVisible();
    await expect(page.getByText('Yes, we do vegan cakes!')).toBeVisible();

    await page.click('button:has-text("Approve & Send Draft")');
    await expect(page.getByText('sent').first()).toBeVisible();
  });
});
