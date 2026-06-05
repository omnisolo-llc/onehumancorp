import { test, expect } from './fixtures';

test.describe('Grandmother UX Fixes E2E tests', () => {
  test('login screen uses plain language labels', async ({ page }) => {
<<<<<<< HEAD
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.getByPlaceholder('Email or Username')).toBeVisible();
    await expect(page.getByPlaceholder('Password')).toBeVisible();
=======
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'One Human Corp' })).toBeVisible();
    await expect(page.getByText('Sign in to manage your business')).toBeVisible();
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await expect(page.getByRole('button', { name: /Start Business Setup/ })).toBeVisible();
  });

  test('custom software screen uses plain language for external tools', async ({ page }) => {
<<<<<<< HEAD
    await page.goto('/integrations');

    await expect(page.getByRole('heading', { name: 'Tool Integrations' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Twilio Conversations' })).toBeVisible();
    await expect(page.getByText('Central omnichannel inbox via Twilio Conversations API for SMS, WhatsApp, and chat.')).toBeVisible();
=======
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/integrations');

    await expect(page.getByRole('heading', { name: 'Connect Custom Software' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Social Media Accounts' })).toBeVisible();
    await expect(page.getByText('Manage all your social media messages and posts in one place.')).toBeVisible();
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
  });
});
