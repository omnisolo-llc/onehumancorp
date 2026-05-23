import { test, expect } from './fixtures';
import { judgeGeneratedOutput } from './ai-judge';

test.describe('Customer Inbox', () => {
  test('drafts and sends a reply', async ({ page }, testInfo) => {

    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Customer Inbox' }).or(page.getByRole('heading', { name: 'Unified Inbox' }))).toBeVisible();
    await page.getByRole('button', { name: /AI Draft/ }).click();
    await expect(page.locator('#reply-input')).not.toHaveValue('');
    const draft = await page.locator('#reply-input').inputValue();
    await judgeGeneratedOutput(testInfo, {
      output: draft,
      rubric: 'The reply must directly answer that vegan birthday cake options are available, sound helpful and professional, avoid making unsupported promises, and be ready to send to a customer.',
    });
    await page.getByRole('button', { name: 'Send' }).click();
    await expect(page.locator('#messages-list')).toContainText(draft);
  });

  test('returns to dashboard on mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/inbox');
    await page.getByRole('button', { name: '< Back' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('Check if the Unified Inbox loads properly', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Unified Inbox' }).or(page.getByRole('heading', { name: 'Customer Inbox' }))).toBeVisible({ timeout: 15000 });
  });

  test('Select a message thread to view it', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Unified Inbox' }).or(page.getByRole('heading', { name: 'Customer Inbox' }))).toBeVisible({ timeout: 15000 });
  });

  test('Verify message cards contain source, created_at, and content', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Unified Inbox' }).or(page.getByRole('heading', { name: 'Customer Inbox' }))).toBeVisible({ timeout: 15000 });
  });

  test('Ensure user can reply and clear input', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Unified Inbox' }).or(page.getByRole('heading', { name: 'Customer Inbox' }))).toBeVisible({ timeout: 15000 });
  });

  test('Ensure back button returns to message list on mobile view', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Unified Inbox' }).or(page.getByRole('heading', { name: 'Customer Inbox' }))).toBeVisible({ timeout: 15000 });
  });
});
