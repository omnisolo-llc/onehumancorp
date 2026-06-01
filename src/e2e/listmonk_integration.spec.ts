import { test, expect } from './fixtures';

test('Listmonk integration connection CUJ', async ({ page }) => {
  // Navigate to integrations page
  await page.goto('/integrations');

  // Verify Listmonk is present
  await expect(page.getByRole('heading', { name: 'Listmonk' })).toBeVisible();

  // Click Connect on Listmonk
  const listmonkCard = page.locator('.rounded-\\[16px\\]', { hasText: 'Listmonk' });

  // Accept the OAuth alert that will be triggered
  page.on('dialog', dialog => dialog.accept());

  await listmonkCard.getByRole('button', { name: 'Connect' }).click();

  // Verify status changes to Manage/Connected
  await expect(listmonkCard.getByRole('button', { name: 'Manage' })).toBeVisible();
  await expect(listmonkCard.getByText('connected')).toBeVisible();
});
