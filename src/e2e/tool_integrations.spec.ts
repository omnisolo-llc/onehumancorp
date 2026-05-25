import { test, expect } from './fixtures';

test.describe('Tool Integrations UI Premium Dashbaord', () => {
  test.beforeEach(async ({ page }) => {
    // Dismiss the upgrade modal if it appears
    page.on('dialog', dialog => dialog.accept());
    await page.goto('/');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Connect Tools' }).first()).toBeVisible();
  });

  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Connect Tools' })).toBeVisible();
    await expect(page.getByText('Seamlessly connect your favorite apps to streamline your business operations.')).toBeVisible();
  });

  test('displays manychat integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Manychat' })).toBeVisible();
    await expect(page.getByText('Unified inbox for Instagram, Messenger, and WhatsApp.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Cal.com' })).toBeVisible();
    await expect(page.getByText('Zero-Config Booking & Calendar Sync.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Shippo' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByText('Automated Label Generation and real-time shipping rates.')).toBeVisible();
    await expect(page.getByText('Accept credit cards and local payment methods in Latin America.')).toBeVisible();
  });

  test('displays email automation and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Mailchimp Automations' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Zoom' })).toBeVisible();
    await expect(page.getByText('Trigger email campaigns based on customer purchase behavior.')).toBeVisible();
    await expect(page.getByText('Auto-Generated Meeting Links for online services.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio' })).toBeVisible();
    await expect(page.getByText('Reliable SMS alerts for new orders and customer notifications.')).toBeVisible();
  });

  test('can connect Manychat', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Manychat' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Manychat...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Cal.com', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Cal.com' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Cal.com...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Mailchimp Automations and Mercado Pago', async ({ page }) => {
    const resendBtn = page.locator('div.card.glass').filter({ hasText: 'Mailchimp Automations' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await resendBtn.click();

    const mercadoBtn = page.locator('div.card.glass').filter({ hasText: 'Mercado Pago' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await mercadoBtn.click();
  });

  test('can connect Shippo, Twilio, and Zoom', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const shippoBtn = page.locator('div.card.glass').filter({ hasText: 'Shippo' }).getByRole('button', { name: 'Connect' });
    await shippoBtn.click();
    const twBtn = page.locator('div.card.glass').filter({ hasText: 'Twilio' }).getByRole('button', { name: 'Connect' });
    await twBtn.click();
    const zoomBtn = page.locator('div.card.glass').filter({ hasText: 'Zoom' }).getByRole('button', { name: 'Connect' });
    await zoomBtn.click();
  });
});
