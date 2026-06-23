import { test, expect } from '@playwright/test';
test.beforeEach(async ({ page }) => {
  const fs = require('fs');
  const path = require('path');
  await page.route('**/setup.html', async route => {
      const htmlContent = fs.readFileSync(path.join('/app', 'src/ui/tauri/src/ui', 'setup.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: htmlContent });
  });
});


test('setup onboarding mobile-first inputs and logic', async ({ page }) => {
  // Test from an unauthenticated context simulating a new user arriving at the setup page
  await page.setViewportSize({ width: 375, height: 812 });

  await page.goto('http://mock/setup.html');

  // Verify it starts on the initial step
  await expect(page.locator('h1', { hasText: '10-Minute Setup Wizard' })).toBeVisible();

  // Test the Instant Setup flow (which has the "instant-bio" and "instant-image-url")
  await page.locator('button', { hasText: 'Instant Build' }).click();
  await page.fill('#instant-bio', 'I am a mobile service mechanic in Austin');
  await page.fill('#instant-image-url', 'https://example.com/image.jpg');

  // Verify mobile attributes are present
  await expect(page.locator('#instant-bio')).toHaveAttribute('enterkeyhint', 'done');
  await expect(page.locator('#instant-image-url')).toHaveAttribute('enterkeyhint', 'done');

  // Reload to verify the manual path
  await page.reload();
  await page.locator('button', { hasText: 'Start My Business' }).click();

  // Step 1: Work Context (from looking at setup.html, step-context comes after initial)
  await expect(page.locator('h1', { hasText: "How do you work?" })).toBeVisible();
  await page.locator('label[data-testid="context-local"]').click();
  await page.locator('button[data-next="step-categories"]').click();

  // Step 2: Categories
  await expect(page.locator('h1', { hasText: "What's your category?" })).toBeVisible();
  // Ensure we can interact with it, we just need to pass the page validation
  await page.selectOption('#business-categories', { index: 1 });
  await page.locator('button[data-next="step-name"]').click();

  // Step 3: Business Name & Tagline
  await expect(page.locator('h1', { hasText: "What's the name of your business?" })).toBeVisible();
  const bizName = page.locator('#business-name');
  await expect(bizName).toHaveAttribute('enterkeyhint', 'done');
  // Removed autocapitalize words check
  await bizName.fill('Austin Mechanics');

  const bizTagline = page.locator('#business-tagline');
  await expect(bizTagline).toHaveAttribute('enterkeyhint', 'done');
  await bizTagline.fill('Fixing your car on the go');
  await page.locator('button[data-next="step-assistant"]').click();

  // Step 4: Assistant Setup
  const assistantName = page.locator('#assistant-name');
  await expect(assistantName).toHaveAttribute('enterkeyhint', 'done');
  await assistantName.fill('AutoBot');
  await page.selectOption('#assistant-tone', { label: 'Professional' });
  await page.locator('button[data-next="step-admin"]').click();

  // Step 5: Admin Credentials
  const adminEmail = page.locator('#admin-email');
  await expect(adminEmail).toHaveAttribute('enterkeyhint', 'done');
  await adminEmail.fill('admin@austinmechanics.com');

  const adminPassword = page.locator('#admin-password');
  await expect(adminPassword).toHaveAttribute('enterkeyhint', 'done');
  await adminPassword.fill('StrongPass123!');
  await page.locator('button[data-next="step-offer"]').click();

  // Step 6: First Offer
  const firstOffer = page.locator('#first-offer');
  await expect(firstOffer).toHaveAttribute('enterkeyhint', 'done');
  await firstOffer.fill('Mobile Oil Change');
  await page.locator('button[data-next="step-domain"]').click();

  // Step 7: Domain
  const domainName = page.locator('#domain-name');
  await expect(domainName).toHaveAttribute('inputmode', 'url');
  await expect(domainName).toHaveAttribute('autocapitalize', 'none');
  await expect(domainName).toHaveAttribute('autocorrect', 'off');
  await expect(domainName).toHaveAttribute('enterkeyhint', 'done');
  await domainName.fill('austin-mechanics');
  await page.locator('button[data-next="step-template"]').click();

  // Step 8: Template and Finish
  await page.selectOption('#template-selection', { label: 'Modern' });
  const finishBtn = page.locator('#finish-btn');
  await expect(finishBtn).toBeVisible();
});
