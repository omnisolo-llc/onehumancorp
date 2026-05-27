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
    await expect(page.getByRole('heading', { name: 'Meta Cloud API' })).toBeVisible();
    await expect(page.getByText('Unified Inbox for IG, FB, and WhatsApp.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Google Calendar API' })).toBeVisible();
    await expect(page.getByText('Two-way synchronization for appointments.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'EasyPost' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByText('Real-time checkout rates and one-click PDF labels.')).toBeVisible();
    await expect(page.getByText('Accept local payment methods like Pix in LATAM.')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Mailchimp Marketing API' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Zoom API' })).toBeVisible();
    await expect(page.getByText('Automatic synchronization of customer lists.')).toBeVisible();
    await expect(page.getByText('Auto-generated unique meeting links for bookings.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio Programmable SMS' })).toBeVisible();
    await expect(page.getByText('Automated SMS confirmations and reminders.')).toBeVisible();
  });

  test('can connect Meta Cloud API', async ({ page }) => {
    const connectButton = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Meta Cloud API' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting meta_cloud_api via OAuth...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Google Calendar API', async ({ page }) => {
    const connectButton = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Google Calendar API' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting google_calendar_api via OAuth...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Mailchimp Marketing API and Mercado Pago', async ({ page }) => {
    const listmonkBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Mailchimp Marketing API' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await listmonkBtn.click();

    const mercadoBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Mercado Pago' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await mercadoBtn.click();
  });

  test('can connect EasyPost, Twilio Programmable SMS, and Zoom API', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const easypostBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'EasyPost' }).getByRole('button', { name: 'Connect' });
    await easypostBtn.click();
    const twBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Twilio Programmable SMS' }).getByRole('button', { name: 'Connect' });
    await twBtn.click();
    const jitsiBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Zoom API' }).getByRole('button', { name: 'Connect' });
    await jitsiBtn.click();
  });
});
