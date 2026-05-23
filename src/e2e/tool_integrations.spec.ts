import { test, expect } from './fixtures';

test.describe('Tool Integrations UI Premium Dashbaord', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Connect Tools' }).first()).toBeVisible();
  });

  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Connect Tools' })).toBeVisible();
    await expect(page.getByText('Seamlessly connect your favorite apps to streamline your business operations.')).toBeVisible();
  });

  test('displays ManyChat integration', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'ManyChat' })).toBeVisible();
  });

  test('displays Cal.com integration', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Cal.com' })).toBeVisible();
  });

  test('displays Resend integration', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Resend' })).toBeVisible();
  });

  test('displays Mercado Pago integration', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
  });

  test('displays EasyPost integration', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'EasyPost' })).toBeVisible();
  });

  test('displays Twilio integration', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio' })).toBeVisible();
  });

  test('displays Daily.co integration', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Daily.co' })).toBeVisible();
  });
});
