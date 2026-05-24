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
    await expect(page.getByRole('heading', { name: 'Connect Tools' })).toBeVisible();
    await expect(page.getByText('Seamlessly connect your favorite apps to streamline your business operations.')).toBeVisible();
  });

  test('displays social media integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Manychat' })).toBeVisible();
    await expect(page.getByText('Unified Social Media Inbox for Instagram, Facebook, and WhatsApp.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Cal.com' })).toBeVisible();
    await expect(page.getByText('Zero-Config Booking & Calendar Sync.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Shippo' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByText('Automated Label Generation and real-time shipping rates.')).toBeVisible();
    await expect(page.getByText('Accept credit cards and local payment methods in Latin America.')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Resend' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Zoom' })).toBeVisible();
    await expect(page.getByText('AI-Powered Email Marketing and simple customer newsletters.')).toBeVisible();
    await expect(page.getByText('Auto-Generated Meeting Links for online services.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio' })).toBeVisible();
    await expect(page.getByText('Reliable SMS alerts for new orders and customer notifications.')).toBeVisible();
  });

  test('can connect Manychat', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Manychat' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Manychat...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Cal.com', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Cal.com' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Cal.com...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Resend and Mercado Pago', async ({ page }) => {
    const resendBtn = page.locator('div.card.glass').filter({ hasText: 'Resend' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await resendBtn.click();

    const mercadoBtn = page.locator('div.card.glass').filter({ hasText: 'Mercado Pago' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await mercadoBtn.click();
  });

  test('can connect Shippo, Twilio, and Zoom', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const shippoBtn = page.locator('div.card.glass').filter({ hasText: 'Shippo' }).getByRole('button', { name: 'Connect' });
    await shippoBtn.click();
    const twBtn = page.locator('div.card.glass').filter({ hasText: 'Twilio' }).getByRole('button', { name: 'Connect' });
    await twBtn.click();
    const zoomBtn = page.locator('div.card.glass').filter({ hasText: 'Zoom' }).getByRole('button', { name: 'Connect' });
    await zoomBtn.click();
  });
});

test.describe('Tool Integrations E2E Workflows', () => {
  test('Cal.com integration flow', async ({ page }) => {
    await page.goto('/');
    await page.locator('button.nav-item', { hasText: 'Meetings' }).click();
    await expect(page.locator('h1', { hasText: 'AI Service Booking' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Cal.com Booking Hours' })).toBeVisible();

    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Availability saved!');
      dialog.accept();
    });
    const saveBtn = page.locator('button#save-cal-hours');
    await saveBtn.click();
    await expect(saveBtn).toHaveText('Saved!');
  });

  test('ManyChat integration flow', async ({ page }) => {
    await page.goto('/');
    await page.getByRole('button', { name: 'Check Messages' }).click();
    await expect(page.locator('h1', { hasText: 'Customer Inbox' })).toBeVisible();

    const draftBtn = page.locator('button', { hasText: '✨ AI Draft' }).first();
    await draftBtn.click();

    await expect(page.locator('#reply-input')).not.toHaveValue('');
    await page.getByRole('button', { name: 'Send' }).click();
    await expect(page.locator('#messages-list')).toContainText(await page.locator('#reply-input').inputValue());
  });

  test('Resend integration flow', async ({ page }) => {
    await page.goto('/');
    await page.getByRole('button', { name: 'Your Team' }).click();
    await expect(page.locator('h1', { hasText: 'Agents' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Email Broadcast' })).toBeVisible();

    await page.getByRole('button', { name: '✨ Generate AI Draft' }).click();
    await expect(page.locator('#email-subject')).toHaveValue('Subject: Check out our New Summer Collection!');
    await expect(page.locator('#email-body')).toContainText('summer collection');

    const dialogPromise = page.waitForEvent('dialog');
    await page.getByRole('button', { name: 'Send' }).click();
    const dialog = await dialogPromise;
    expect(dialog.message()).toContain('Sent via Resend!');
    await dialog.accept();
  });

  test('Mercado Pago integration flow', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => localStorage.setItem('tenant_region', 'LATAM'));
    await page.evaluate(() => {
      // @ts-ignore
      showScreen('checkout-screen');
    });
    await expect(page.locator('h1', { hasText: 'Checkout' })).toBeVisible();

    await expect(page.locator('#mercado-pago-option')).toBeVisible();

    await page.locator('input[value="mercadopago"]').click();

    const dialogPromise = page.waitForEvent('dialog');
    await page.getByRole('button', { name: 'Pay Now' }).click();
    const dialog = await dialogPromise;
    expect(dialog.message()).toContain('Payment successful via mercadopago!');
    await dialog.accept();
  });

  test('Shippo integration flow', async ({ page }) => {
    await page.goto('/');
    await page.locator('button.nav-item', { hasText: 'Orders' }).click();
    await expect(page.locator('h1', { hasText: 'Orders' })).toBeVisible();

    const purchaseBtn = page.locator('button', { hasText: 'Purchase Shipping Label ($4.50)' });
    await expect(purchaseBtn).toBeVisible();
    await purchaseBtn.click();

    await expect(page.locator('#shippo-fulfillment-1042')).toContainText('Fulfilled - Tracking: SHP923485');
  });

  test('Twilio SMS integration flow', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => {
      // @ts-ignore
      showScreen('settings-screen');
    });
    await expect(page.locator('h1', { hasText: 'Settings' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Notification Preferences' })).toBeVisible();

    const smsToggle = page.locator('#twilio-sms-orders');
    await smsToggle.check();
    await expect(smsToggle).toBeChecked();
  });

  test('Daily.co virtual meeting integration flow', async ({ page }) => {
    await page.goto('/');
    await page.locator('button.nav-item', { hasText: 'Meetings' }).click();
    await expect(page.locator('h1', { hasText: 'AI Service Booking' })).toBeVisible();

    const dialogPromise = page.waitForEvent('dialog');
    await page.locator('button#daily-co-join-btn').click();
    const dialog = await dialogPromise;
    expect(dialog.message()).toContain('Joining Virtual Meeting via https://ohc.daily.co/mock-room-123');
    await dialog.accept();
  });
});
