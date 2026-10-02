import { test, expect } from './fixtures';

test('simple test', async ({ anonymousPage }) => {
  await anonymousPage.goto('/login');
  await expect(anonymousPage.getByRole('heading', { name: 'Login' }).first()).toBeVisible();
});
