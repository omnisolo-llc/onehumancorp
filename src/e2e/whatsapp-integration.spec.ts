import { test, expect } from './fixtures';

function integrationCard(page: import('@playwright/test').Page, name: string) {
  return page
    .getByRole('heading', { name })
    .locator('xpath=ancestor::div[contains(@class, "rounded")][1]');
}

test.describe('WhatsApp Integration UI', () => {
  test.beforeEach(async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/integrations');
    await expect(page.getByRole('heading', { name: 'Tool Integrations' })).toBeVisible();
  });

  test('displays Twilio for WhatsApp and WhatsApp Cloud API integration cards', async ({ page }) => {
    const twilioCard = integrationCard(page, 'Twilio for WhatsApp');
    await expect(twilioCard).toBeVisible();
    await expect(twilioCard.getByText('Central WhatsApp Inbox for Work Triage and Customer Assistant powered by Twilio.')).toBeVisible();
    await expect(twilioCard.getByRole('button', { name: 'Connect' })).toBeVisible();

    const cloudApiCard = integrationCard(page, 'WhatsApp Cloud API');
    await expect(cloudApiCard).toBeVisible();
    await expect(cloudApiCard.getByText('Direct WhatsApp Cloud API connection for messages.')).toBeVisible();
    await expect(cloudApiCard.getByRole('button', { name: 'Connect' })).toBeVisible();
  });

  test('can open Twilio for WhatsApp modal and interact with inputs', async ({ page }) => {
    await integrationCard(page, 'Twilio for WhatsApp').getByRole('button', { name: 'Connect' }).click();

    // Verify modal appears
    await expect(page.getByRole('heading', { name: 'Connect Twilio for WhatsApp' })).toBeVisible();
    await expect(page.getByText('Enter your Twilio API credentials to securely link your WhatsApp Business account.')).toBeVisible();

    // Verify inputs exist and can be filled
    const sidInput = page.getByLabel('Account SID');
    await sidInput.fill('AC1234567890abcdef1234567890abcdef');
    await expect(sidInput).toHaveValue('AC1234567890abcdef1234567890abcdef');

    const tokenInput = page.getByLabel('Auth Token');
    await tokenInput.fill('supersecrettoken123');
    await expect(tokenInput).toHaveValue('supersecrettoken123');

    const phoneInput = page.getByLabel('WhatsApp Phone Number');
    await phoneInput.fill('+1234567890');
    await expect(phoneInput).toHaveValue('+1234567890');

    // Close modal
    await page.locator('.fixed.inset-0 button[aria-label="Close modal"]').click();
    await expect(page.getByRole('heading', { name: 'Connect Twilio for WhatsApp' })).toBeHidden();
  });

  test('keeps Twilio for WhatsApp unconnected when verification is unavailable', async ({ page }) => {
    await integrationCard(page, 'Twilio for WhatsApp').getByRole('button', { name: 'Connect' }).click();

    const sidInput = page.getByLabel('Account SID');
    await sidInput.fill('AC1234567890abcdef1234567890abcdef');

    const tokenInput = page.getByLabel('Auth Token');
    await tokenInput.fill('supersecrettoken123');

    const phoneInput = page.getByLabel('WhatsApp Phone Number');
    await phoneInput.fill('+1234567890');

    // These are synthetic fixture values. The actual route returns 501 before
    // provider execution; this test must never claim a verified connection.
    const pending = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/integrations/whatsapp/connect' && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Save & Connect' }).click();
    const response = await pending;
    expect(response.status()).toBe(501);
    expect(await response.json()).toMatchObject({ success: false, status: 'pending_verification', usable: false });
    await expect(page.getByText('Failed to connect Twilio for WhatsApp.', { exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Connect Twilio for WhatsApp', exact: true })).toBeVisible();
    await expect(page.getByText('Twilio for WhatsApp connected.', { exact: true })).toHaveCount(0);
    await expect(integrationCard(page, 'Twilio for WhatsApp').getByRole('button', { name: 'Connect', exact: true })).toBeVisible();
    await expect(page).toHaveURL(/\/integrations$/);
  });

  test('can open WhatsApp Cloud API modal', async ({ page }) => {
    await integrationCard(page, 'WhatsApp Cloud API').getByRole('button', { name: 'Connect' }).click();

    // Verify modal appears
    await expect(page.getByRole('heading', { name: 'Connect WhatsApp Cloud API' })).toBeVisible();
    await expect(page.getByText('Connect your WhatsApp Business Account directly using the WhatsApp Cloud API.')).toBeVisible();

    // Close modal
    await page.locator('.fixed.inset-0 button[aria-label="Close modal"]').click();
    await expect(page.getByRole('heading', { name: 'Connect WhatsApp Cloud API' })).toBeHidden();
  });

  test('shows unavailable Meta sign-in without inventing a connected state', async ({ page }) => {
    await integrationCard(page, 'WhatsApp Cloud API').getByRole('button', { name: 'Connect' }).click();

    const configured = await page.evaluate(() => typeof (window as Window & { FB?: { login?: unknown } }).FB?.login === 'function');
    expect(configured, 'This unavailable fixture must not start real Meta sign-in').toBe(false);
    const requests: string[] = [];
    page.on('request', request => {
      if (request.method() === 'POST' && new URL(request.url()).pathname === '/api/v1/integrations/whatsapp_cloud_api/connect') requests.push(request.url());
    });
    await page.getByRole('button', { name: 'Continue with Meta' }).click();
    await expect(page.getByText('WhatsApp connection is unavailable because Meta sign-in is not configured.', { exact: true })).toBeVisible();
    expect(requests).toEqual([]);
    await expect(page.getByText('WhatsApp Cloud API connected.', { exact: true })).toHaveCount(0);
    await expect(integrationCard(page, 'WhatsApp Cloud API').getByRole('button', { name: 'Connect', exact: true })).toBeVisible();
    await expect(page).toHaveURL(/\/integrations$/);
  });
});
