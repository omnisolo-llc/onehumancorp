import { test, expect } from './fixtures';

test.describe('Tool Integrations - Real Workflows', () => {
  test.beforeEach(async ({ page }) => {
    // Dismiss the upgrade modal if it appears
    page.on('dialog', dialog => dialog.accept());
  });

  test('Mercado Pago integration in checkout', async ({ page }) => {
    await page.goto('/checkout');
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();

    // Select LATAM region to show Mercado Pago
    await page.locator('select').selectOption('MX');
    await expect(page.getByText('we recommend Mercado Pago')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Pay with Mercado Pago' })).toBeVisible();
  });

  test('EasyPost and Twilio integration in dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Order Fulfillment' })).toBeVisible();

    // Check EasyPost print label button
    await expect(page.getByRole('button', { name: 'Print Label (EasyPost)' })).toBeVisible();

    // Check Twilio SMS notifications toggle
    await expect(page.getByText('Twilio SMS Notifications')).toBeVisible();
  });

  test('Listmonk integration in dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Email Marketing' })).toBeVisible();

    // Check Listmonk send email button
    await expect(page.getByRole('button', { name: 'Send via Listmonk' })).toBeVisible();
  });

  test('Cal.com and Jitsi Meet integration in calendar', async ({ page }) => {
    await page.goto('/calendar');
    await expect(page.getByRole('heading', { name: 'Calendar & Bookings' })).toBeVisible();

    // Check Cal.com sync button
    await expect(page.getByRole('button', { name: 'Sync Cal.com' })).toBeVisible();

    // Check Jitsi Meet links for online appointments
    await expect(page.getByText('🎥 Join Jitsi Meet')).toBeVisible();
  });

  test('Ayrshare integration in unified inbox', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Unified Inbox' })).toBeVisible();

    // Check Ayrshare connect button
    await expect(page.getByRole('button', { name: 'Link Socials (Ayrshare)' })).toBeVisible();
  });
});
