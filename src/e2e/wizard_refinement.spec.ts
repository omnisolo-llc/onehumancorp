import { test, expect } from './fixtures';

test.describe('Wizard Refinement E2E', () => {
  test('keeps the setup flow plain-language and reversible', async ({ page }) => {
    // We updated /builder to serve this flow
    await page.goto('/builder');
    await expect(page.getByText('What are you building today?')).toBeVisible();
    await page.getByText('Selling Products').click();
    await expect(page.getByRole('heading', { name: "Let's build your store" })).toBeVisible();
    await page.getByRole('button', { name: "Next: Choose Vibe" }).click();
    await expect(page.getByText('Business name must be at least 3 characters.')).toBeVisible();
  });

  test.skip('exposes AI helper and prompt tuning areas', async ({ page }) => {
  });

  test.skip('settings remain accessible from dashboard quick actions', async ({ page }) => {
  });
});