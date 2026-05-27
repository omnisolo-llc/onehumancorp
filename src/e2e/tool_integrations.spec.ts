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
    await expect(page.getByRole('heading', { name: 'Social Media Accounts' })).toBeVisible();
    await expect(page.getByText('Manage all your social media messages and posts in one place.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect my Instagram and Facebook' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Customer Booking' })).toBeVisible();
    await expect(page.getByText('Let customers book appointments directly on your personal calendar.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Shipping Labels' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Payment Processing' })).toBeVisible();
    await expect(page.getByText('Print shipping labels and automatically track packages for your orders.')).toBeVisible();
    await expect(page.getByText('Accept credit cards and digital wallets securely from anywhere in the world.')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Customer Emails' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Online Meetings' })).toBeVisible();
    await expect(page.getByText('Send email updates and promotions to your customers.')).toBeVisible();
    await expect(page.getByText('Host online video meetings with your customers easily without extra downloads.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Text Notifications' })).toBeVisible();
    await expect(page.getByText('Send automatic text message updates to your customers about their orders.')).toBeVisible();
  });

  test('can connect Social Media Accounts', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Social Media Accounts' }).getByRole('button', { name: 'Connect my Instagram and Facebook' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Ayrshare...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Customer Booking', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Customer Booking' }).getByRole('button', { name: 'Set up my booking link' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Cal.com...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Customer Emails and Local Payments', async ({ page }) => {
    const emailBtn = page.locator('div.card.glass').filter({ hasText: 'Customer Emails' }).getByRole('button', { name: 'Start sending emails' });
    page.once('dialog', dialog => dialog.accept());
    await emailBtn.click();

    const paymentBtn = page.locator('div.card.glass').filter({ hasText: 'Payment Processing' }).getByRole('button', { name: 'Set up payments' });
    page.once('dialog', dialog => dialog.accept());
    await paymentBtn.click();
  });

  test('can connect Shipping, Text Notifications, and Online Meetings', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const shippingBtn = page.locator('div.card.glass').filter({ hasText: 'Shipping Labels' }).getByRole('button', { name: 'Set up shipping' });
    await shippingBtn.click();
    const smsBtn = page.locator('div.card.glass').filter({ hasText: 'Text Notifications' }).getByRole('button', { name: 'Enable text messages' });
    await smsBtn.click();
    const meetingBtn = page.locator('div.card.glass').filter({ hasText: 'Online Meetings' }).getByRole('button', { name: 'Create my meeting room' });
    await meetingBtn.click();
  });
});
