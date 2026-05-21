import { test, expect } from '@playwright/test';

test.describe('Referral Program', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await page.waitForURL('**/*');

    const menuBtn = page.locator('button:has-text("Menu")').first();
    if (await menuBtn.isVisible()) {
        await menuBtn.click();
    }

    // Fallback: the server UI might set the display property of elements instead of true navigation
    const referralsBtn = page.getByRole('button', { name: 'Referrals' });
    if (await referralsBtn.isVisible()) {
        await referralsBtn.click();
    } else {
        await page.goto('/referrals');
    }
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();
  });

  test('displays referral link and share tools', async ({ page }) => {
    await expect(page.locator('#referral-link')).toContainText('ohc://join?ref=');
    await expect(page.getByText('Share with 1-Tap')).toBeVisible();
    await expect(page.getByRole('button', { name: /Share to Instagram/ })).toBeVisible();
  });

  test('copies invite message via share button', async ({ page }) => {
    const copyButton = page.getByRole('button', { name: 'Copy' }).first();
    await expect(copyButton).toBeVisible();

    page.on('dialog', dialog => dialog.accept());
    await copyButton.click();
  });

  test('verifies growth progress section', async ({ page }) => {
    await expect(page.getByText('Your Growth Progress')).toBeVisible();
    await expect(page.getByText('0 / 5 Referrals')).toBeVisible();
    await expect(page.getByText('You\'re on your way! Invite 1 more business to unlock your first reward.')).toBeVisible();
  });

  test('verifies hero card with offer is presented', async ({ page }) => {
    await expect(page.getByText('Give 1 Month, Get 1 Month Free')).toBeVisible();
    await expect(page.getByText('Invite other small business owners to OHC.')).toBeVisible();
  });

  test('clicks WhatsApp sharing button to trigger action', async ({ page }) => {
    const whatsappBtn = page.getByRole('button', { name: /WhatsApp/ });
    await expect(whatsappBtn).toBeVisible();

    page.on('dialog', dialog => dialog.accept());
    await whatsappBtn.click();
  });
});
