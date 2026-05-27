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
    await expect(page.getByRole('heading', { name: 'Tool Integrations' })).toBeVisible();
    await expect(page.getByText('Supercharge your workflow by connecting your favorite tools.')).toBeVisible();
  });

  test('displays social media integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Manychat' })).toBeVisible();
    await expect(page.getByText('Unified social media inbox for Instagram, Facebook, and WhatsApp.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).first()).toBeVisible();
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
    await expect(page.getByRole('heading', { name: 'Resend' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Zoom' })).toBeVisible();
    await expect(page.getByText('Developer-friendly email marketing and transactional emails.')).toBeVisible();
    await expect(page.getByText('Seamless Video Call Link Generation.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio' })).toBeVisible();
    await expect(page.getByText('Reliable SMS alerts for new orders and customer notifications.')).toBeVisible();
  });

  test('can connect Manychat', async ({ page }) => {
    const connectButton = page.getByRole('heading', { name: 'Manychat' }).locator('..').getByRole('button', { name: 'Connect' });
    await connectButton.click();
    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();
  });

  test('can connect Cal.com', async ({ page }) => {
    await page.goto('/integrations');
    const connectButton = page.getByRole('heading', { name: 'Cal.com' }).locator('..').getByRole('button', { name: 'Connect' });
    await connectButton.click();
    await expect(page.getByRole('heading', { name: 'Calendar' })).toBeVisible();
  });

  test('can connect Resend and Mercado Pago', async ({ page }) => {
    await page.goto('/integrations');
    const listmonkBtn = page.getByRole('heading', { name: 'Resend' }).locator('..').getByRole('button', { name: 'Connect' });
    await listmonkBtn.click();
    await expect(page.getByRole('heading', { name: 'Email Campaigns' })).toBeVisible();

    await page.goto('/integrations');
    const mercadoBtn = page.getByRole('heading', { name: 'Mercado Pago' }).locator('..').getByRole('button', { name: 'Connect' });
    await mercadoBtn.click();
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();
  });

  test('can connect Shippo, Twilio, and Zoom', async ({ page }) => {
    await page.goto('/integrations');
    const easypostBtn = page.getByRole('heading', { name: 'Shippo' }).locator('..').getByRole('button', { name: 'Connect' });
    await easypostBtn.click();
    await expect(page.getByRole('heading', { name: 'Order Shipping' })).toBeVisible();

    await page.goto('/integrations');
    const twBtn = page.getByRole('heading', { name: 'Twilio' }).locator('..').getByRole('button', { name: 'Connect' });
    await twBtn.click();
    await expect(page.getByRole('heading', { name: 'Calendar' })).toBeVisible();

    await page.goto('/integrations');
    const jitsiBtn = page.getByRole('heading', { name: 'Zoom' }).locator('..').getByRole('button', { name: 'Connect' });
    await jitsiBtn.click();
    await expect(page.getByRole('heading', { name: 'Calendar' })).toBeVisible();
  });
});
