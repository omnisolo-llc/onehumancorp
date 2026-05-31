import { test, expect } from './fixtures';

test.describe('Tool Integrations UI Premium Dashbaord', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/integrations');
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible();
  });

  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible();
    await expect(page.getByText('Supercharge your workflow by connecting your favorite tools.')).toBeVisible();
  });

  test('displays social media integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Ayrshare' })).toBeVisible();
    await expect(page.getByText('Unified API for posting and retrieving messages across social networks.')).toBeVisible();
    await expect(page.locator('div', { has: page.getByRole('heading', { name: 'Ayrshare' }) }).getByRole('button', { name: 'Connect' }).first()).toBeVisible();
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
    const connectButton = page.locator('div', { has: page.getByRole('heading', { name: 'Ayrshare' }) }).getByRole('button', { name: 'Connect' }).first();
    await connectButton.click({ force: true });
  });

  test('can enable Autonomous Booking Agent', async ({ page }) => {
    const connectButton = page.locator('div', { has: page.getByRole('heading', { name: 'Cal.com' }) }).getByRole('button', { name: 'Connect' }).first();
    await connectButton.click({ force: true });
  });

  test('can connect Customer Emails and Local Payments', async ({ page }) => {
    const emailBtn = page.locator('div', { has: page.getByRole('heading', { name: 'Listmonk' }) }).getByRole('button', { name: 'Connect' }).first();
    await emailBtn.click({ force: true });

    const paymentBtn = page.locator('div', { has: page.getByRole('heading', { name: 'Mercado Pago' }) }).getByRole('button', { name: 'Connect' }).first();
    await paymentBtn.click({ force: true });
  });

  test('can connect Shipping, Text Notifications, and Online Meetings', async ({ page }) => {
    const shippingBtn = page.locator('div', { has: page.getByRole('heading', { name: 'EasyPost' }) }).getByRole('button', { name: 'Connect' }).first();
    await shippingBtn.click({ force: true });
    const smsBtn = page.locator('div', { has: page.getByRole('heading', { name: 'Twilio' }) }).getByRole('button', { name: 'Connect' }).first();
    await smsBtn.click({ force: true });
    const meetingBtn = page.locator('div', { has: page.getByRole('heading', { name: 'Jitsi Meet' }) }).getByRole('button', { name: 'Connect' }).first();
    await meetingBtn.click({ force: true });
  });
});
