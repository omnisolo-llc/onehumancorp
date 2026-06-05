import { test, expect } from './fixtures';

test.describe('Tool Integrations UI Premium Dashbaord', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/integrations');
    await expect(page.getByRole('heading', { name: 'Connect Custom Software' }).first()).toBeVisible();
  });

  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Connect Custom Software' }).first()).toBeVisible();
    await expect(page.getByText('Seamlessly connect your favorite apps to streamline your business operations.')).toBeVisible();
  });

  test('displays social media integration card', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Social Media Accounts' })).toBeVisible();
    await expect(page.getByText('Manage all your social media messages and posts in one place.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect my Instagram and Facebook' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Autonomous Booking Agent' })).toBeVisible();
    await expect(page.getByText('Let your AI agent negotiate meeting times with clients over text, update your calendar, and send payment links.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Shipping Labels' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Local Payments' })).toBeVisible();
    await expect(page.getByText('Print shipping labels and automatically track packages for your orders.')).toBeVisible();
    await expect(page.getByText('Get paid easily using local payment methods in Latin America.')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Customer Emails' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Online Meetings' })).toBeVisible();
    await expect(page.getByText('Send email updates and promotions to your customers.')).toBeVisible();
    await expect(page.getByText('Host online video meetings with your customers easily without extra downloads.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Text Notifications' })).toBeVisible();
    await expect(page.getByText('Send automatic text message updates to your customers about their orders.')).toBeVisible();
  });

  test('displays front omnichannel inbox card', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Omnichannel Inbox' })).toBeVisible();
    await expect(page.getByText('Unified inbox aggregating messages from Front, Instagram, WhatsApp, and email.')).toBeVisible();
  });

  test('can connect Social Media Accounts', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    const connectButton = page.locator('div').filter({ hasText: /^Social Media AccountsManage all your social media messages and posts in one place\.Connect$/ }).getByRole('button', { name: 'Connect' })

    // Check that we show an alert correctly
    page.on('dialog', dialog => dialog.accept());
    await connectButton.click();
  });

  test('can enable Autonomous Booking Agent', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    const connectButton = page.locator('div').filter({ hasText: /^Autonomous Booking AgentLet your AI agent negotiate meeting times with clients over text, update your calendar, and send payment links\.Connect$/ }).getByRole('button', { name: 'Connect' });
    page.on('dialog', dialog => dialog.accept());
    await connectButton.click();
  });

  test('can connect Customer Emails and Local Payments', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    const emailBtn = page.locator('div').filter({ hasText: /^Customer EmailsSend email updates and promotions to your customers\.Connect$/ }).getByRole('button', { name: 'Connect' });
    page.on('dialog', dialog => dialog.accept());
    await emailBtn.click();

    const paymentBtn = page.locator('div').filter({ hasText: /^Local PaymentsGet paid easily using local payment methods in Latin America\.Connect$/ }).getByRole('button', { name: 'Connect' });
    await paymentBtn.click();
  });

  test('can connect Shipping, Text Notifications, and Online Meetings', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    const shippingBtn = page.locator('div').filter({ hasText: /^Shipping LabelsPrint shipping labels and automatically track packages for your orders\.Connect$/ }).getByRole('button', { name: 'Connect' });
    page.on('dialog', dialog => dialog.accept());
    await shippingBtn.click();
    const smsBtn = page.locator('div').filter({ hasText: /^Text NotificationsSend automatic text message updates to your customers about their orders\.Connect$/ }).getByRole('button', { name: 'Connect' });
    await smsBtn.click();
    const meetingBtn = page.locator('div').filter({ hasText: /^Online MeetingsHost online video meetings with your customers easily without extra downloads\.Connect$/ }).getByRole('button', { name: 'Connect' });
    await meetingBtn.click();
  });

  test('can connect Front', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    const connectButton = page.locator('div').filter({ hasText: /^Omnichannel InboxUnified inbox aggregating messages from Front, Instagram, WhatsApp, and email\.Connect$/ }).getByRole('button', { name: 'Connect' });
    await connectButton.click();
  });
});
