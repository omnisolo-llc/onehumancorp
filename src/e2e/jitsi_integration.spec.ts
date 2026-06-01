import { test, expect } from './fixtures';

test('Jitsi Meet integration connection CUJ', async ({ page }) => {
  await page.goto('/integrations');
  await expect(page.getByRole('heading', { name: 'Jitsi Meet' })).toBeVisible();
  const jitsiCard = page.locator('.rounded-\\[16px\\]', { hasText: 'Jitsi Meet' });
  page.on('dialog', dialog => dialog.accept());
  await jitsiCard.getByRole('button', { name: 'Connect' }).click();
  await expect(jitsiCard.getByRole('button', { name: 'Manage' })).toBeVisible();
  await expect(jitsiCard.getByText('connected')).toBeVisible();
});
