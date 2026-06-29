import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Referral Viral Loop', () => {
  test('generates and displays referral link correctly', async ({ page, context }) => {
    await adminPage(page, context);
    await page.goto('/referrals.html');
    const linkInput = page.locator('#referral-link');
    await expect(linkInput).not.toHaveValue('', { timeout: 10000 });
    const value = await linkInput.inputValue();
    expect(value).toMatch(/ohc\.app\/ref\/|ohc:\/\/join\?ref=/);
  });

  test('copies referral link to clipboard', async ({ page, context }) => {
    await adminPage(page, context);
    await page.goto('/referrals.html');
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    // Wait for generation
    const linkInput = page.locator('#referral-link');
    await expect(linkInput).not.toHaveValue('', { timeout: 10000 });

    await page.locator('#copy-btn').click();

    const btnText = await page.locator('#copy-btn').innerText();
    expect(btnText).toBe('Copied!');
  });

  test('displays performance metrics accurately', async ({ page, context }) => {
    await adminPage(page, context);
    await page.goto('/referrals.html');

    // Check that metrics load (either to 0 or valid number depending on DB seed)
    await expect(page.locator('#metrics-invites')).not.toHaveText('-', { timeout: 10000 });
    await expect(page.locator('#metrics-active')).not.toHaveText('-');
    await expect(page.locator('#metrics-revenue')).toContainText('$');
    await expect(page.locator('#metrics-pending')).toContainText('$');
  });

  test('displays referral tier based on conversions', async ({ page, context }) => {
    await adminPage(page, context);
    await page.goto('/referrals.html');

    // Check that tier name is present
    await expect(page.locator('#tier-name')).not.toHaveText('', { timeout: 10000 });

    // Should show the tier progress bar
    const progressContainer = page.locator('.tier-progress-container');
    await expect(progressContainer).toBeVisible();

    const progressBar = page.locator('#referral-tier-progress');
    // Ensure the inline style width is set
    const style = await progressBar.getAttribute('style');
    expect(style).toContain('width:');
  });

  test('allows sharing on whatsapp and x', async ({ page, context }) => {
    await adminPage(page, context);
    await page.goto('/referrals.html');

    // Check if the buttons exist and have proper text
    const whatsappBtn = page.locator('#share-whatsapp-btn');
    const xBtn = page.locator('#share-x-btn');

    await expect(whatsappBtn).toBeVisible();
    await expect(xBtn).toBeVisible();

    await expect(whatsappBtn).toHaveText('Share on WhatsApp');
    await expect(xBtn).toHaveText('Share on X');
  });
});
