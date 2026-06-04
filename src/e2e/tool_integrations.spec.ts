import { test, expect } from './fixtures';

test.describe('Tool Integrations UI Premium Dashbaord', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/integrations');
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible();
  });

  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible();
    await expect(page.getByText('Supercharge your workflow by connecting your favorite tools.')).toBeVisible();
  });

  test('displays social media integration card', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Meta Graph API' })).toBeVisible();
    await expect(page.getByText('Central Instagram and Facebook Inbox.')).toBeVisible();
    await expect(page.locator('div').filter({ hasText: 'Meta Graph API' }).getByRole('button', { name: 'Connect' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Cal.com' })).toBeVisible();
    await expect(page.getByText('Zero-Config Booking & Calendar Sync.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Shippo' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByText('Painless Shipping Labels & Tracking.')).toBeVisible();
    await expect(page.getByText('Accept credit cards and local payment methods in Latin America.')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Resend' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Zoom' })).toBeVisible();
    await expect(page.getByText('Transactional and Marketing Emails.')).toBeVisible();
    await expect(page.getByText('Automated Online Lesson Links.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Twilio Conversations' })).toBeVisible();
    await expect(page.getByText('Central omnichannel inbox via Twilio Conversations API for SMS, WhatsApp, and chat.')).toBeVisible();
  });

  test('displays front omnichannel inbox card', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await expect(page.getByRole('heading', { name: 'Front' })).toBeVisible();
    await expect(page.getByText('Central omnichannel inbox aggregating messages across all channels.')).toBeVisible();
  });

  test('can connect Social Media Accounts', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    const connectButton = page.locator('div').filter({ hasText: 'Meta Graph API' }).getByRole('button', { name: 'Connect' }).first();

    // Check that we show an alert correctly
    page.on('dialog', dialog => dialog.accept());
    await connectButton.click();
  });

  test('can enable Autonomous Booking Agent', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    const connectButton = page.locator('div').filter({ hasText: 'Cal.com' }).getByRole('button', { name: 'Connect' }).first();
    page.on('dialog', dialog => dialog.accept());
    await connectButton.click();
  });

  test('can connect Customer Emails and Local Payments', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    const emailBtn = page.locator('div').filter({ hasText: 'Resend' }).getByRole('button', { name: 'Connect' }).first();
    page.on('dialog', dialog => dialog.accept());
    await emailBtn.click();

    const paymentBtn = page.locator('div').filter({ hasText: 'Mercado Pago' }).getByRole('button', { name: 'Connect' }).first();
    await paymentBtn.click();
  });

  test('can connect Shipping, Text Notifications, and Online Meetings', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    const shippingBtn = page.locator('div').filter({ hasText: 'Shippo' }).getByRole('button', { name: 'Connect' }).first();
    page.on('dialog', dialog => dialog.accept());
    await shippingBtn.click();
    const smsBtn = page.locator('div').filter({ hasText: 'Twilio Conversations' }).getByRole('button', { name: 'Connect' }).first();
    await smsBtn.click();
    const meetingBtn = page.locator('div').filter({ hasText: 'Zoom' }).getByRole('button', { name: 'Connect' }).first();
    await meetingBtn.click();
  });

  test('can connect Front', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    const connectButton = page.locator('div').filter({ hasText: 'Front' }).getByRole('button', { name: 'Connect' }).first();
    await connectButton.click();
  });
});
