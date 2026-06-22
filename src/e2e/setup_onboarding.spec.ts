import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test('setup onboarding mobile-first inputs and logic', async ({ page }) => {
  // Setup offline test context without the fixtures that break connection
  const workspaceRoot = process.env.TEST_WORKSPACE
      ? path.join(process.env.TEST_SRCDIR || path.resolve(__dirname, '..', '..'), process.env.TEST_WORKSPACE)
      : path.resolve(__dirname, '..', '..');

  const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

  await page.route('**/setup.html', async route => {
      const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: content });
  });

  // Test from an unauthenticated context simulating a new user arriving at the setup page
  await page.setViewportSize({ width: 375, height: 812 });

  await page.goto('http://mock/setup.html', { waitUntil: 'load', timeout: 60000 });

  // Try clicking conversational setup first
  await page.evaluate(() => { if (typeof (window as any).goToStep === 'function') { (window as any).goToStep('step-instant'); } });

  await expect(page.locator('#step-instant')).toBeVisible();

  // Test the Instant Setup flow (which has the "instant-bio" and "instant-image-url")
  await page.fill('#instant-bio', 'I am a mobile service mechanic in Austin');
  await page.fill('#instant-image-url', 'https://example.com/image.jpg');

  // Verify mobile attributes are present
  await expect(page.locator('#instant-bio')).toHaveAttribute('enterkeyhint', 'done');
  await expect(page.locator('#instant-image-url')).toHaveAttribute('enterkeyhint', 'next');

  // Navigate to manual path
  await page.evaluate(() => { if (typeof (window as any).goToStep === 'function') { (window as any).goToStep('step-context'); } });

  // Step 1: Work Context
  await expect(page.locator('h1', { hasText: "How do you work?" })).toBeVisible();
  await page.locator('label', { hasText: 'Storefront or Cafe' }).evaluate(b => b.click());
  await page.evaluate(() => document.querySelector('button[data-next="step-categories"]')?.click());

  // Step 2: Categories
  await expect(page.locator('h1', { hasText: "What's your category?" })).toBeVisible();
  await page.selectOption('#business-categories', { index: 1 });
  await page.evaluate(() => document.querySelector('button[data-next="step-name"]')?.click());

  // Step 3: Business Name & Tagline
  await expect(page.locator('h1', { hasText: "What's the name of your business?" })).toBeVisible();
  const bizName = page.locator('#business-name');
  await expect(bizName).toHaveAttribute('enterkeyhint', 'next');
  await expect(bizName).toHaveAttribute('autocapitalize', 'words');
  await bizName.fill('Austin Mechanics');

  const bizTagline = page.locator('#business-tagline');
  await expect(bizTagline).toHaveAttribute('enterkeyhint', 'next');
  await bizTagline.fill('Fixing your car on the go');
  await page.evaluate(() => document.querySelector('button[data-next="step-assistant"]')?.click());

  // Step 4: Assistant Setup
  const assistantName = page.locator('#assistant-name');
  await expect(assistantName).toHaveAttribute('enterkeyhint', 'done');
  await assistantName.fill('AutoBot');
  await page.selectOption('#assistant-tone', { label: 'Professional' });
  await page.evaluate(() => document.querySelector('button[data-next="step-admin"]')?.click());

  // Step 5: Admin Credentials
  const adminEmail = page.locator('#admin-email');
  await expect(adminEmail).toHaveAttribute('enterkeyhint', 'next');
  await adminEmail.fill('admin@austinmechanics.com');

  const adminPassword = page.locator('#admin-password');
  await expect(adminPassword).toHaveAttribute('enterkeyhint', 'next');
  await adminPassword.fill('StrongPass123!');
  await page.evaluate(() => document.querySelector('button[data-next="step-offer"]')?.click());

  // Step 6: First Offer
  const firstOffer = page.locator('#first-offer');
  await expect(firstOffer).toHaveAttribute('enterkeyhint', 'next');
  await firstOffer.fill('Mobile Oil Change');
  await page.evaluate(() => document.querySelector('button[data-next="step-domain"]')?.click());

  // Step 7: Domain
  const domainName = page.locator('#domain-name');
  await expect(domainName).toHaveAttribute('inputmode', 'url');
  await expect(domainName).toHaveAttribute('autocapitalize', 'none');
  await expect(domainName).toHaveAttribute('autocorrect', 'off');
  await expect(domainName).toHaveAttribute('enterkeyhint', 'done');
  await domainName.fill('austin-mechanics');
  await page.evaluate(() => document.querySelector('button[data-next="step-template"]')?.click());

  // Step 8: Template and Finish
  await page.selectOption('#template-selection', { label: 'Modern' });
  const finishBtn = page.locator('#finish-btn');
  await expect(finishBtn).toBeVisible();
});
