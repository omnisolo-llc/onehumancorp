import { test, expect } from './fixtures';

test('Cal.com integration connection CUJ', async ({ page }) => {
  await page.goto('/integrations');
  await expect(page.getByRole('heading', { name: 'Cal.com' })).toBeVisible();
  const calcomCard = page.locator('.rounded-\\[16px\\]', { hasText: 'Cal.com' });
  page.on('dialog', dialog => dialog.accept());
  await calcomCard.getByRole('button', { name: 'Connect' }).click();
  await expect(calcomCard.getByRole('button', { name: 'Manage' })).toBeVisible();
  await expect(calcomCard.getByText('connected')).toBeVisible();
});
