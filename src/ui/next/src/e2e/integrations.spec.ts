import { test, expect } from '@playwright/test';

test.describe('Integrations Page Flow', () => {
  test('navigates to integrations and verifies all new tools are visible', async ({ page }) => {
    await page.goto('http://localhost:3000/integrations');

    await expect(page.locator('h1', { hasText: 'Tool Integrations' })).toBeVisible();

    // Verify MailerLite
    await expect(page.locator('h3', { hasText: 'MailerLite' })).toBeVisible();
    await expect(page.locator('text="AI-Driven Customer Retention Campaigns without the marketing jargon."')).toBeVisible();

    // Verify Shippo
    await expect(page.locator('h3', { hasText: 'Shippo' })).toBeVisible();
    await expect(page.locator('text="1-Click Shipping Label Generation and Tracking."')).toBeVisible();

    // Verify Zoom
    await expect(page.locator('h3', { hasText: 'Zoom' })).toBeVisible();
    await expect(page.locator('text="Auto-Generated Online Consultation Links."')).toBeVisible();
  });

  test('filters tools correctly using tabs', async ({ page }) => {
    await page.goto('http://localhost:3000/integrations');

    // Click Marketing tab
    await page.locator('button', { hasText: 'Marketing' }).click();
    await expect(page.locator('h3', { hasText: 'MailerLite' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Shippo' })).not.toBeVisible();

    // Click Operations tab
    await page.locator('button', { hasText: 'Operations' }).click();
    await expect(page.locator('h3', { hasText: 'Shippo' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Zoom' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'MailerLite' })).not.toBeVisible();

    // Click Finance tab
    await page.locator('button', { hasText: 'Finance' }).click();
    await expect(page.locator('h3', { hasText: 'Mercado Pago' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Zoom' })).not.toBeVisible();
  });

  test('connects MailerLite correctly', async ({ page }) => {
    await page.goto('http://localhost:3000/integrations');

    // Listen for alert
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Connecting MailerLite via OAuth...');
      await dialog.accept();
    });

    const mailerLiteCard = page.locator('.rounded-\\[16px\\]', { hasText: 'MailerLite' });
    await mailerLiteCard.locator('button', { hasText: 'Connect' }).click();

    // After connecting, it should redirect to /inbox
    await page.waitForURL('**/inbox');
    expect(page.url()).toContain('/inbox');
  });

  test('connects Zoom correctly', async ({ page }) => {
    await page.goto('http://localhost:3000/integrations');

    // Listen for alert
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Connecting Zoom via OAuth...');
      await dialog.accept();
    });

    const zoomCard = page.locator('.rounded-\\[16px\\]', { hasText: 'Zoom' });
    await zoomCard.locator('button', { hasText: 'Connect' }).click();

    // After connecting, it should redirect to /dashboard
    await page.waitForURL('**/dashboard');
    expect(page.url()).toContain('/dashboard');
  });

  test('connects Shippo correctly', async ({ page }) => {
    await page.goto('http://localhost:3000/integrations');

    // Listen for alert
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Connecting Shippo via OAuth...');
      await dialog.accept();
    });

    const shippoCard = page.locator('.rounded-\\[16px\\]', { hasText: 'Shippo' });
    await shippoCard.locator('button', { hasText: 'Connect' }).click();

    // After connecting, it should redirect to /inbox
    await page.waitForURL('**/inbox');
    expect(page.url()).toContain('/inbox');
  });
});
