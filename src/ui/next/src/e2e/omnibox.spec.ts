import { test, expect } from '../../../e2e/fixtures';

test('Global Search / Omnibox should open with Cmd+K and search', async ({ adminPage }) => {
  const page = adminPage;
  await page.goto('/dashboard');
  await page.keyboard.press('Meta+k');
  const input = page.getByPlaceholder('Search customers, orders, messages...');
  await expect(input).toBeVisible();
  await input.fill('Ava');
  await page.waitForTimeout(2000);

  await expect(page.getByText('Ava Customer')).toBeVisible();

  await page.keyboard.press('Escape');
  await expect(input).not.toBeVisible();
});
