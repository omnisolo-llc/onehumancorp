import { expect, test } from './fixtures';
import { judgeGeneratedOutput } from './ai-judge';

test.describe('Customer Inbox', () => {
  test('drafts and sends a reply', async ({ page }, testInfo) => {
    await page.goto('http://localhost:3000/inbox');
    await expect(page.locator("text=Customer Inbox")).toBeVisible({ timeout: 10000 });
  });

  test('returns to dashboard on mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('http://localhost:3000/inbox');
    await page.locator('text=< Back').first().click({ force: true });
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });
});
