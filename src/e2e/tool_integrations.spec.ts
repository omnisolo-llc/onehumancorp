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
    await expect(page.getByRole('heading', { name: 'Connect Tools' }).first()).toBeVisible();
    await expect(page.getByText('Seamlessly connect your favorite apps to streamline your business operations.')).toBeVisible();
  });

  test('displays social media integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Meta Graph API' })).toBeVisible();
    await expect(page.getByText('Unified Instagram & WhatsApp Inbox.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect Meta Graph API' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Google Calendar' })).toBeVisible();
    await expect(page.getByText('Seamless Booking & Conflict Resolution.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Shippo' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByText('Automated Label Generation & Tracking.')).toBeVisible();
    await expect(page.getByText('Accept local payment methods in LATAM.')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Resend' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Zoom' })).toBeVisible();
    await expect(page.getByText('AI-Driven Customer Campaigns.')).toBeVisible();
    await expect(page.getByText('Automated Online Lesson Links.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio' })).toBeVisible();
    await expect(page.getByText('Critical Notifications & Offline Alerts.')).toBeVisible();
  });

  test('can connect Social Media Accounts', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Meta Graph API' }).getByRole('button', { name: 'Connect Meta Graph API' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Meta Graph API...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can enable Autonomous Booking Agent', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Google Calendar' }).getByRole('button', { name: 'Connect Google Calendar' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Google Calendar...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Customer Emails and Local Payments', async ({ page }) => {
    const emailBtn = page.locator('div.card.glass').filter({ hasText: 'Resend' }).getByRole('button', { name: 'Start sending emails' });
    page.once('dialog', dialog => dialog.accept());
    await emailBtn.click();

    const paymentBtn = page.locator('div.card.glass').filter({ hasText: 'Mercado Pago' }).getByRole('button', { name: 'Accept local payments' });
    page.once('dialog', dialog => dialog.accept());
    await paymentBtn.click();
  });

  test('can connect Shipping, Text Notifications, and Online Meetings', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const shippingBtn = page.locator('div.card.glass').filter({ hasText: 'Shippo' }).getByRole('button', { name: 'Set up shipping' });
    await shippingBtn.click();
    const smsBtn = page.locator('div.card.glass').filter({ hasText: 'Twilio' }).getByRole('button', { name: 'Enable text messages' });
    await smsBtn.click();
    const meetingBtn = page.locator('div.card.glass').filter({ hasText: 'Zoom' }).getByRole('button', { name: 'Create my meeting room' });
    await meetingBtn.click();
  });
});
