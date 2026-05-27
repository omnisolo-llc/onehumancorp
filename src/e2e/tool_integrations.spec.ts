import { test, expect } from './fixtures';

test.describe('Tool Integrations UI Premium Dashbaord', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to integrations directly using the mock login fixture
    page.on('dialog', async dialog => {
      try {
        await dialog.accept();
      } catch (e) {
        // Ignore "dialog already handled" errors
      }
    });
    await page.goto('/integrations');
  });

  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible();
    await expect(page.getByText('Supercharge your workflow by connecting your favorite tools.')).toBeVisible();
  });

  test('displays social media integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Manychat' })).toBeVisible();
    await expect(page.getByText('Unified social media inbox for Instagram, Facebook, and WhatsApp.')).toBeVisible();
    await expect(page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Manychat' }).getByRole('button', { name: 'Connect' })).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Calendly' })).toBeVisible();
    await expect(page.getByText('Automated Booking widget for your store.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Shippo' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByText('Painless Shipping Labels & Tracking.')).toBeVisible();
    await expect(page.getByText('Accept credit cards and local payment methods in Latin America.')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Mailchimp' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Zoom' })).toBeVisible();
    await expect(page.getByText('Embedded, No-Jargon Email Campaigns.')).toBeVisible();
    await expect(page.getByText('Zero-Setup Online Lessons and video conferencing.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio', exact: true })).toBeVisible();
    await expect(page.getByText('Reliable SMS alerts for new orders and customer notifications.')).toBeVisible();
  });

  test('can connect Social Media Accounts', async ({ page }) => {
    const connectButton = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Manychat' }).getByRole('button', { name: 'Connect' }).first();
    await connectButton.click();
    await expect(page).toHaveURL(/.*\/inbox/);
  });

  test('can connect Customer Booking', async ({ page }) => {
    const connectButton = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Calendly' }).getByRole('button', { name: 'Connect' }).first();
    await connectButton.click();
    await expect(page).toHaveURL(/.*\/dashboard/);
  });

  test('can connect Customer Emails and Local Payments', async ({ page }) => {
    const emailBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Mailchimp' }).getByRole('button', { name: 'Connect' }).first();
    await emailBtn.click();

    const paymentBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Mercado Pago' }).getByRole('button', { name: 'Connect' }).first();
    await paymentBtn.click();
  });

  test('can connect Shipping, Text Notifications, and Online Meetings', async ({ page }) => {
    const shippingBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Shippo' }).getByRole('button', { name: 'Connect' }).first();
    await shippingBtn.click();
    const smsBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Twilio' }).getByRole('button', { name: 'Connect' }).first();
    await smsBtn.click();
    const meetingBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Zoom' }).getByRole('button', { name: 'Connect' }).first();
    await meetingBtn.click();
    await expect(page).toHaveURL(/.*\/dashboard/);
  });
});
