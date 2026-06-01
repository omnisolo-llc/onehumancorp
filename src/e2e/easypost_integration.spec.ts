import { test, expect } from './fixtures';

test('EasyPost integration connection CUJ', async ({ page }) => {
  await page.goto('/integrations');
  await expect(page.getByRole('heading', { name: 'EasyPost' })).toBeVisible();
  const easypostCard = page.locator('.rounded-\\[16px\\]', { hasText: 'EasyPost' });
  page.on('dialog', dialog => dialog.accept());
  await easypostCard.getByRole('button', { name: 'Connect' }).click();
  await expect(easypostCard.getByRole('button', { name: 'Manage' })).toBeVisible();
  await expect(easypostCard.getByText('connected')).toBeVisible();
});
