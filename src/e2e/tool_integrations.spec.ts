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
    await expect(page.getByRole('heading', { name: 'Meta' })).toBeVisible();
    await expect(page.getByText('Unified Social Media Inbox Integration')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Google Calendar' })).toBeVisible();
    await expect(page.getByText('Native Google Calendar Sync')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Shippo' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByText('Native Shipping Rate Calculation and Label Generation')).toBeVisible();
    await expect(page.getByText('Native Integration of Local Payment Methods')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'SendGrid' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Zoom' })).toBeVisible();
    await expect(page.getByText('Native Email Campaign Manager')).toBeVisible();
    await expect(page.getByText('Native Zoom Link Generation')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio' })).toBeVisible();
    await expect(page.getByText('Native SMS Order Notifications')).toBeVisible();
  });

  test('can connect Meta', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Meta' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Meta...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Google Calendar', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Google Calendar' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Google Calendar...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect SendGrid and Mercado Pago', async ({ page }) => {
    const listmonkBtn = page.locator('div.card.glass').filter({ hasText: 'SendGrid' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await listmonkBtn.click();

    const mercadoBtn = page.locator('div.card.glass').filter({ hasText: 'Mercado Pago' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await mercadoBtn.click();
  });

  test('can connect Shippo, Twilio, and Jitsi Meet', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const easypostBtn = page.locator('div.card.glass').filter({ hasText: 'Shippo' }).getByRole('button', { name: 'Connect' });
    await easypostBtn.click();
    const twBtn = page.locator('div.card.glass').filter({ hasText: 'Twilio' }).getByRole('button', { name: 'Connect' });
    await twBtn.click();
    const jitsiBtn = page.locator('div.card.glass').filter({ hasText: 'Zoom' }).getByRole('button', { name: 'Connect' });
    await jitsiBtn.click();
  });
});
