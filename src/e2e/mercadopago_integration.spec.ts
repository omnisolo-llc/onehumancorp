import { test, expect } from './fixtures';

test('Mercado Pago integration connection CUJ', async ({ page }) => {
  await page.goto('/integrations');
  await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
  const mercadopagoCard = page.locator('.rounded-\\[16px\\]', { hasText: 'Mercado Pago' });
  page.on('dialog', dialog => dialog.accept());
  await mercadopagoCard.getByRole('button', { name: 'Connect' }).click();
  await expect(mercadopagoCard.getByRole('button', { name: 'Manage' })).toBeVisible();
  await expect(mercadopagoCard.getByText('connected')).toBeVisible();
});
