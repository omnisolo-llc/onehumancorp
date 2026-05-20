import { test, expect } from './fixtures';

test.describe('Integrations Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Connect Custom Software' }).first()).toBeVisible();
  });

  test('shows the custom software integration page', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Custom Integration' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Custom Software', exact: true })).toBeVisible();
  });

  test('shows product data access copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Product Data Access' })).toBeVisible();
    await expect(page.getByText('Read Product List')).toBeVisible();
    await expect(page.getByText('Manage your custom software connections here.')).toBeVisible();
  });

  test('can return to dashboard from integrations', async ({ page }) => {
    await page.getByRole('button', { name: 'Back to Dashboard' }).click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('opens integrations from dashboard quick actions', async ({ page }) => {
    await page.getByRole('button', { name: 'Back to Dashboard' }).click();
    await page.getByRole('button', { name: 'Integrations' }).click();

    await expect(page.getByRole('heading', { name: /Facebook/ })).toBeVisible();
    await expect(page.locator('#facebook-integration').getByRole('button', { name: 'Configure' })).toBeVisible();
  });

  test('configure action routes to inbox', async ({ page }) => {
    await page.getByRole('button', { name: 'Back to Dashboard' }).click();
    await page.getByRole('button', { name: 'Integrations' }).click();
    await page.locator('#facebook-integration').getByRole('button', { name: 'Configure' }).click();

    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();
    await expect(page.getByText('Facebook User')).toBeVisible();
  });

  test('displays all 7 required external tool integrations on the Connect Tools page', async ({ page }) => {
    // Check for WhatsApp Business
    await expect(page.getByRole('heading', { name: 'WhatsApp Business' })).toBeVisible();
    await expect(page.getByText('Unified Customer Inbox. Manage your WhatsApp customer inquiries alongside other messages.')).toBeVisible();

    // Check for Calendly
    await expect(page.getByRole('heading', { name: 'Calendly' })).toBeVisible();
    await expect(page.getByText('Automated Booking. Share a booking link that automatically syncs with your availability.')).toBeVisible();

    // Check for Mailchimp
    await expect(page.getByRole('heading', { name: 'Mailchimp' })).toBeVisible();
    await expect(page.getByText('Email Marketing. Automatically sync new contacts into your newsletter tool.')).toBeVisible();

    // Check for Mercado Pago
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByText('Payment Processing. Simple checkout with local payment methods for LATAM customers.')).toBeVisible();

    // Check for Shippo
    await expect(page.getByRole('heading', { name: 'Shippo' })).toBeVisible();
    await expect(page.getByText('Shipping & Logistics. Generate shipping labels directly from your order dashboard.')).toBeVisible();

    // Check for Twilio SMS
    await expect(page.getByRole('heading', { name: 'Twilio SMS' })).toBeVisible();
    await expect(page.getByText('SMS & Notifications. Send text messages directly from your management dashboard.')).toBeVisible();

    // Check for Zoom
    await expect(page.getByRole('heading', { name: 'Zoom' })).toBeVisible();
    await expect(page.getByText('Video Conferencing. Automatically generate Zoom links when a session is booked.')).toBeVisible();
  });

  test('can click Connect on WhatsApp Business integration', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const connectButton = page.locator('.card:has-text("WhatsApp Business")').getByRole('button', { name: 'Connect' });
    await expect(connectButton).toBeVisible();
    await connectButton.click();
  });

  test('can click Connect on Calendly integration', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const connectButton = page.locator('.card:has-text("Calendly")').getByRole('button', { name: 'Connect' });
    await expect(connectButton).toBeVisible();
    await connectButton.click();
  });

  test('can click Connect on Mailchimp integration', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const connectButton = page.locator('.card:has-text("Mailchimp")').getByRole('button', { name: 'Connect' });
    await expect(connectButton).toBeVisible();
    await connectButton.click();
  });

  test('can click Connect on Mercado Pago integration', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const connectButton = page.locator('.card:has-text("Mercado Pago")').getByRole('button', { name: 'Connect' });
    await expect(connectButton).toBeVisible();
    await connectButton.click();
  });
});
