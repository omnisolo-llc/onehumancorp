import { test, expect } from './fixtures';

function integrationCard(page: import('@playwright/test').Page, name: string) {
  return page
    .getByRole('heading', { name })
    .locator('xpath=ancestor::div[contains(@class, "rounded")][1]');
}

test.describe('WhatsApp Integration UI', () => {
  test.beforeEach(async ({ page }) => {
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
    await page.locator('button').filter({ has: page.locator('svg') }).click();
    await expect(page.getByRole('heading', { name: 'Connect Twilio for WhatsApp' })).toBeHidden();
  });

  test('can connect Twilio for WhatsApp successfully', async ({ page }) => {
    await integrationCard(page, 'Twilio for WhatsApp').getByRole('button', { name: 'Connect' }).click();

    const sidInput = page.getByLabel('Account SID');
    await sidInput.fill('AC1234567890abcdef1234567890abcdef');

    const tokenInput = page.getByLabel('Auth Token');
    await tokenInput.fill('supersecrettoken123');

    const phoneInput = page.getByLabel('WhatsApp Phone Number');
    await phoneInput.fill('+1234567890');

    await page.getByRole('button', { name: 'Save & Connect' }).click();

    // Check for success status updates
    await expect(page.locator('[role="status"]', { hasText: 'Twilio for WhatsApp connected.' })).toBeVisible();
  });

  test('can open WhatsApp Cloud API modal', async ({ page }) => {
    await integrationCard(page, 'WhatsApp Cloud API').getByRole('button', { name: 'Connect' }).click();

    // Verify modal appears
    await expect(page.getByRole('heading', { name: 'Connect WhatsApp Cloud API' })).toBeVisible();
    await expect(page.getByText('Connect your WhatsApp Business Account directly using the WhatsApp Cloud API.')).toBeVisible();

    // Close modal
    await page.locator('button').filter({ has: page.locator('svg') }).click();
    await expect(page.getByRole('heading', { name: 'Connect WhatsApp Cloud API' })).toBeHidden();
  });

  test('can connect WhatsApp Cloud API successfully', async ({ page }) => {
    await integrationCard(page, 'WhatsApp Cloud API').getByRole('button', { name: 'Connect' }).click();

    await page.getByRole('button', { name: 'Continue with Meta' }).click();

    // Check for success status updates
    await expect(page.locator('[role="status"]', { hasText: 'WhatsApp Cloud API connected.' })).toBeVisible();
  });
});
