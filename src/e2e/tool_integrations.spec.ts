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

  test('displays social media integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Meta Graph API' })).toBeVisible();
    await expect(page.getByText('Unified Native Social Media Inbox for Instagram, Facebook, and WhatsApp.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Calendly' })).toBeVisible();
    await expect(page.getByText('Zero-Config Booking & Calendar Sync.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Shippo' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByText('Automated Label Generation and real-time shipping rates.')).toBeVisible();
    await expect(page.getByText('Accept credit cards and local payment methods in Latin America.')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Mailchimp' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Zoom' })).toBeVisible();
    await expect(page.getByText('Keep marketing lists in sync automatically.')).toBeVisible();
    await expect(page.getByText('Auto-Generated Meeting Links for online services.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio SMS' })).toBeVisible();
    await expect(page.getByText('Abstracted developer SMS notifications for the unified inbox.')).toBeVisible();
  });

  test('displays whatsapp integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'WhatsApp Business API' })).toBeVisible();
    await expect(page.getByText('Manage WhatsApp messages in the unified inbox.')).toBeVisible();
  });

  test('can connect Meta Graph API', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Meta Graph API' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Meta Graph API...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Calendly', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Calendly' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Calendly...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Mailchimp and Mercado Pago', async ({ page }) => {
    const mailchimpBtn = page.locator('div.card.glass').filter({ hasText: 'Mailchimp' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await mailchimpBtn.click();

    const mercadoBtn = page.locator('div.card.glass').filter({ hasText: 'Mercado Pago' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await mercadoBtn.click();
  });

  test('can connect Shippo, Twilio SMS, and Zoom', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const shippoBtn = page.locator('div.card.glass').filter({ hasText: 'Shippo' }).getByRole('button', { name: 'Connect' });
    await shippoBtn.click();
    const twBtn = page.locator('div.card.glass').filter({ hasText: 'Twilio SMS' }).getByRole('button', { name: 'Connect' });
    await twBtn.click();
    const zoomBtn = page.locator('div.card.glass').filter({ hasText: 'Zoom' }).getByRole('button', { name: 'Connect' });
    await zoomBtn.click();
  });

  test('can connect WhatsApp Business API', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const waBtn = page.locator('div.card.glass').filter({ hasText: 'WhatsApp Business API' }).getByRole('button', { name: 'Connect' });
    await waBtn.click();
  });
});
