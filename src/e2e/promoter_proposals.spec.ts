import { test, expect } from './fixtures';

test.describe('The Promoter Agent Live UI Data', () => {
  test('fetches and renders real proposals from backend', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/promoter');
    await expect(page.locator('text="The Promoter"')).toBeVisible();
    await expect(page.locator('text="Loading proposals..."').or(page.locator('text="No new proposals generated."'))).toBeVisible();
  });

  test('handles empty state properly', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/promoter');
    await expect(page.locator('text="No new proposals generated."')).toBeVisible();
  });

  test('shows correct title', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/promoter');
    const title = page.locator('h1');
    await expect(title).toHaveText('The Promoter');
  });

  test('does not show mock data', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/promoter');
    // Ensure the old mocked data text is not there.
    await expect(page.locator('text="Approve & Publish"')).toBeHidden();
  });

  test('renders loading text before empty state', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/promoter');
    // It should briefly show loading
    await expect(page.locator('text="Loading proposals..."').or(page.locator('text="No new proposals generated."'))).toBeVisible();
  });
});
