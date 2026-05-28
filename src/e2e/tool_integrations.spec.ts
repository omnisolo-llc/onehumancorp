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
    await expect(page.getByRole('heading', { name: 'Social Media Integration' })).toBeVisible();
    await expect(page.getByText('Unified Inbox Integration for Instagram, Facebook, WhatsApp, and TikTok.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect with Facebook' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Calendar & Scheduling' })).toBeVisible();
    await expect(page.getByText('Sync and Scheduling via Google Calendar and Outlook.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Shipping & Logistics' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Payment Processing' })).toBeVisible();
    await expect(page.getByText('Real-Time Rates and Label Generation.')).toBeVisible();
    await expect(page.getByText('Global Alternative Payment Methods (Mercado Pago, Paytm, Alipay).')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Email Marketing' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Video Conferencing' })).toBeVisible();
    await expect(page.getByText('Automated Campaigns and Customer Newsletters.')).toBeVisible();
    await expect(page.getByText('Automated Zoom/Meet Link Generation for Services.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'SMS & Notifications' })).toBeVisible();
    await expect(page.getByText('Automated Order Notifications and Alerts.')).toBeVisible();
  });

  test('can connect Social Media Accounts', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Social Media Integration' }).getByRole('button', { name: 'Connect with Facebook' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Facebook...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can enable Autonomous Booking Agent', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Calendar & Scheduling' }).getByRole('button', { name: 'Connect Calendar' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Google Calendar...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Customer Emails and Local Payments', async ({ page }) => {
    const emailBtn = page.locator('div.card.glass').filter({ hasText: 'Email Marketing' }).getByRole('button', { name: 'Send Announcement' });
    page.once('dialog', dialog => dialog.accept());
    await emailBtn.click();

    const paymentBtn = page.locator('div.card.glass').filter({ hasText: 'Payment Processing' }).getByRole('button', { name: 'Enable local payments' });
    page.once('dialog', dialog => dialog.accept());
    await paymentBtn.click();
  });

  test('can connect Shipping, Text Notifications, and Online Meetings', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const shippingBtn = page.locator('div.card.glass').filter({ hasText: 'Shipping & Logistics' }).getByRole('button', { name: 'Print Shipping Label' });
    await shippingBtn.click();
    const smsBtn = page.locator('div.card.glass').filter({ hasText: 'SMS & Notifications' }).getByRole('button', { name: 'Enable SMS Alerts' });
    await smsBtn.click();
    const meetingBtn = page.locator('div.card.glass').filter({ hasText: 'Video Conferencing' }).getByRole('button', { name: 'Enable Video Meeting' });
    await meetingBtn.click();
  });
});
