import { test, expect } from '../fixtures';

test('Offering creation reports the configured provider boundary without inventing a publishable offer', async ({ page }) => {
  await page.goto('/dashboard');
  await expect(page.locator('h2').filter({ hasText: /Welcome back/ })).toBeVisible();
  await page.getByRole('button', { name: 'New Product', exact: true }).click();
  await expect(page).toHaveURL(/\/products\/new$/);
  await page.getByRole('button', { name: 'Or describe your offering' }).click();
  await expect(page.getByRole('heading', { name: 'What do you want to offer?' })).toBeVisible();
  const intent = page.getByPlaceholder('e.g., Guitar lessons for beginners, 1 hour');
  await intent.fill('Guitar lessons for beginners, 1 hour');
  const generated = page.waitForResponse(response => response.url().endsWith('/api/v1/catalog/generate') && response.request().method() === 'POST');
  await page.getByRole('button', { name: 'Generate', exact: true }).click();
  // handle_generate_offering explicitly requires MINIMAX_API_KEY; the native
  // isolated runner omits provider credentials. Valid generation and publishing
  // remain covered by the product/offering provider-boundary unit suites.
  const response = await generated;
  expect(response.status()).toBe(503);
  expect(await response.json()).toEqual({ error: 'offering generation unavailable', message: 'The AI provider is not configured.' });
  await expect(page.getByText('The AI provider is not configured.', { exact: true })).toBeVisible();
  await expect(intent).toHaveValue('Guitar lessons for beginners, 1 hour');
  await expect(page.getByRole('button', { name: 'Looks Good' })).toHaveCount(0);
  await expect(page.getByRole('heading', { name: 'Product Published!' })).toHaveCount(0);
});
