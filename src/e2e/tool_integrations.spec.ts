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
    await expect(page.getByRole('heading', { name: 'Ayrshare' })).toBeVisible();
    await expect(page.getByText('Unified API for posting and retrieving messages across social networks.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Cal.com' })).toBeVisible();
    await expect(page.getByText('Zero-Config Booking & Calendar Sync.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'EasyPost' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByText('Painless Shipping Labels & Tracking.')).toBeVisible();
    await expect(page.getByText('Accept credit cards and local payment methods in Latin America.')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Listmonk' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Jitsi Meet' })).toBeVisible();
    await expect(page.getByText('Embedded, No-Jargon Email Campaigns.')).toBeVisible();
    await expect(page.getByText('Zero-Setup Online Lessons and video conferencing.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio' })).toBeVisible();
    await expect(page.getByText('Reliable SMS alerts for new orders and customer notifications.')).toBeVisible();
  });

  test('can connect Ayrshare', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Ayrshare' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Ayrshare...');
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

  test('can connect Listmonk and Mercado Pago', async ({ page }) => {
    const listmonkBtn = page.locator('div.card.glass').filter({ hasText: 'Listmonk' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await listmonkBtn.click();

    const mercadoBtn = page.locator('div.card.glass').filter({ hasText: 'Mercado Pago' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await mercadoBtn.click();
  });

  test('can connect EasyPost, Twilio, and Jitsi Meet', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const easypostBtn = page.locator('div.card.glass').filter({ hasText: 'EasyPost' }).getByRole('button', { name: 'Connect' });
    await easypostBtn.click();
    const twBtn = page.locator('div.card.glass').filter({ hasText: 'Twilio' }).getByRole('button', { name: 'Connect' });
    await twBtn.click();
    const jitsiBtn = page.locator('div.card.glass').filter({ hasText: 'Jitsi Meet' }).getByRole('button', { name: 'Connect' });
    await jitsiBtn.click();
  });
});
