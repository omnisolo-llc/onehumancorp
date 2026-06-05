import { test, expect } from './fixtures';

test.describe('Grandmother UX Fixes E2E tests', () => {
  test('login screen uses plain language labels', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username')).toBeVisible();
    await expect(page.getByPlaceholder('Password')).toBeVisible();
    await expect(page.getByRole('button', { name: /Start Business Setup/ })).toBeVisible();
  });

  test('custom software screen uses plain language for external tools', async ({ page }) => {
    await page.goto('/integrations');

    await expect(page.getByRole('heading', { name: 'Tool Integrations' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Twilio Conversations' })).toBeVisible();
    await expect(page.getByText('Central omnichannel inbox via Twilio Conversations API for SMS, WhatsApp, and chat.')).toBeVisible();
  });
});
