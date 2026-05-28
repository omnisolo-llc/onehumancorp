import { test, expect } from './fixtures';

test.describe('Tool Integrations UI Premium Dashbaord', () => {
  test.beforeEach(async ({ page }) => {
    // Dismiss the upgrade modal if it appears
    page.on('dialog', dialog => dialog.accept());
    await page.goto('/dashboard');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible();
  });

  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible();
    await expect(page.getByText('Supercharge your workflow by connecting your favorite tools.')).toBeVisible();
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

  test('can connect Social Media Accounts', async ({ page }) => {
    const connectButton = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Ayrshare' }).getByRole('button', { name: 'Connect' });

    await connectButton.click();
  });

  test('can connect Customer Booking', async ({ page }) => {
    const connectButton = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Cal.com' }).getByRole('button', { name: 'Connect' });

    await connectButton.click();
  });

  test('can connect Customer Emails and Local Payments', async ({ page }) => {
    const emailBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Listmonk' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await emailBtn.click();

    const paymentBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Mercado Pago' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await paymentBtn.click();
  });

  test('can connect Shipping, Text Notifications, and Online Meetings', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const shippingBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'EasyPost' }).getByRole('button', { name: 'Connect' });
    await shippingBtn.click();
    const smsBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Twilio' }).getByRole('button', { name: 'Connect' });
    await smsBtn.click();
    const meetingBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Jitsi Meet' }).getByRole('button', { name: 'Connect' });
    await meetingBtn.click();
  });
});
