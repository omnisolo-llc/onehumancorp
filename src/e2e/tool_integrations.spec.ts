import { test, expect } from './fixtures';

test.describe('Tool Integrations UI Premium Dashbaord', () => {
  test.beforeEach(async ({ page }) => {
    // Dismiss the upgrade modal if it appears
    page.on('dialog', dialog => dialog.accept().catch(() => {}));
    await page.goto('/dashboard');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible();
  });

  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible();
    await expect(page.getByText('Supercharge your workflow by connecting your favorite tools.')).toBeVisible();
  });

  test('displays social media integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'ManyChat' })).toBeVisible();
    await expect(page.getByText('Unified social media inbox for Instagram, Facebook, and WhatsApp.')).toBeVisible();
    await expect(page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'ManyChat' }).getByRole('button', { name: 'Connect' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Cal.com' })).toBeVisible();
    await expect(page.getByText('Zero-Config Booking & Calendar Sync.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Shippo' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByText('Painless Shipping Labels & Tracking.')).toBeVisible();
    await expect(page.getByText('Accept credit cards and local payment methods in Latin America.')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'MailerLite' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Whereby' })).toBeVisible();
    await expect(page.getByText('Embedded, No-Jargon Email Campaigns.')).toBeVisible();
    await expect(page.getByText('Zero-Setup Online Lessons and video conferencing.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio Conversations' })).toBeVisible();
    await expect(page.getByText('Unified omnichannel inbox via Twilio Conversations API for SMS, WhatsApp, and chat.')).toBeVisible();
  });

  test('can connect Social Media Accounts', async ({ page }) => {
    const connectButton = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'ManyChat' }).getByRole('button', { name: 'Connect' }).first();
    await connectButton.click();
    await expect(page).toHaveURL(/\/inbox/);
  });

  test('can enable Autonomous Booking Agent', async ({ page }) => {
    const connectButton = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Cal.com' }).getByRole('button', { name: 'Connect' }).first();
    await connectButton.click();
    await expect(page).toHaveURL(/\/dashboard/);
  });

  test('can connect Customer Emails and Local Payments', async ({ page }) => {
    const emailCard = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'MailerLite' });
    await emailCard.getByRole('button', { name: 'Connect' }).click();
    await expect(emailCard.getByRole('button')).toHaveText('Manage');

    const paymentCard = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Mercado Pago' });
    await paymentCard.getByRole('button', { name: 'Connect' }).click();
    await expect(paymentCard.getByRole('button')).toHaveText('Manage');
  });

  test('can connect Shipping, Text Notifications, and Online Meetings', async ({ page }) => {
    const shippingCard = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Shippo' });
    await shippingCard.getByRole('button', { name: 'Connect' }).click();
    await expect(shippingCard.getByRole('button')).toHaveText('Manage');

    const smsCard = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Twilio Conversations' });
    await smsCard.getByRole('button', { name: 'Connect' }).click();

    // Modal opens up
    await page.getByRole('button', { name: 'Save & Connect' }).click();
    await expect(page).toHaveURL(/\/inbox/);
  });
});
