import { test, expect } from '@playwright/test';

test('setup onboarding mobile-first inputs and logic', async ({ page }) => {
  // Route to local file like setup.spec.ts
  const fs = require('fs');
  const path = require('path');
  const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
  await page.route('**/setup.html', async route => {
      const htmlContent = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: htmlContent   });
  });
  // intercept tooltips
  await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({})   });
  });

  // Test from an unauthenticated context simulating a new user arriving at the setup page
  await page.setViewportSize({ width: 375, height: 812 });

  await page.route('**/api/onboarding/start', async route => { await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ success: true, onboardingState: { currentStep: 'done' } }) }); });
  await page.route('**/dashboard.html*', async route => { await route.fulfill({ status: 200, contentType: 'text/html', body: '<html><body>Dashboard</body></html>' }); });
  await page.goto('http://mock/setup.html');

  // Verify it starts on the initial step
  await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible();

  // Test the Instant Setup flow (which has the "instant-bio" and "instant-image-url")
  await page.fill('#instant-bio', 'I am a mobile service mechanic in Austin');

  // Verify mobile attributes are present
  await expect(page.locator('#instant-bio')).toHaveAttribute('enterkeyhint', 'next');

  // Reload to verify the manual path
  await page.reload();
  await page.locator('button', { hasText: 'Step-by-Step Setup' }).click();

  // Step 1: Work Context (from looking at setup.html, step-context comes after initial)
  await expect(page.locator('h1', { hasText: "How do you work?" })).toBeVisible();
  await page.locator('label.context-card').filter({ hasText: 'Service' }).click();
  await page.locator('button[data-next="step-categories"]').click();

  // Step 2: Categories
  await expect(page.locator('h1', { hasText: "What's your category?" })).toBeVisible();
  // Ensure we can interact with it, we just need to pass the page validation
  await page.selectOption('#business-categories', { index: 1 });
  await page.locator('button[data-next="step-name"]').click();

  // Step 3: Business Name & Tagline
  await expect(page.locator('h1', { hasText: "What's the name of your business?" })).toBeVisible();
  const bizName = page.locator('#business-name');
  await expect(bizName).toHaveAttribute('enterkeyhint', 'next');
  await expect(bizName).toHaveAttribute('autocapitalize', 'words');
  await bizName.fill('Austin Mechanics');

  const bizTagline = page.locator('#business-tagline');
  await expect(bizTagline).toHaveAttribute('enterkeyhint', 'next');
  await bizTagline.fill('Fixing your car on the go');
  await page.locator('button[data-next="step-assistant"]').click();

  // Step 4: Assistant Setup
  await page.getByTestId('team-operations').click();


  await page.selectOption('#assistant-tone', { label: 'Professional' });
  await page.locator('button[data-next="step-admin"]').click();

  // Step 5: Admin Credentials
  const adminName = page.locator('#admin-name');
  await adminName.fill('Test Admin');

  const adminEmail = page.locator('#admin-email');
  await expect(adminEmail).toHaveAttribute('enterkeyhint', 'next');
  await adminEmail.fill('admin@austinmechanics.com');

  const adminPassword = page.locator('#admin-password');
  await expect(adminPassword).toHaveAttribute('enterkeyhint', 'next');
  await adminPassword.fill('StrongPass123!');
  await page.locator('button[data-next="step-offer"]').click();

  // Step 6: First Offer
  const firstOffer = page.locator('#first-offer');
  await expect(firstOffer).toHaveAttribute('enterkeyhint', 'next');
  await firstOffer.fill('Mobile Oil Change');
  await page.locator('#step-offer button[data-next="step-location"]').click();

  // Step Location
  await page.locator('#location-input').fill('Austin, TX');
  await page.locator('#step-location button[data-next="step-target-audience"]').click();

  // Step Target Audience
  await page.locator('#target-audience').fill('Car Owners');
  await page.locator('#step-target-audience button[data-next="step-domain"]').click();

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

  await finishBtn.click();
  await page.waitForURL('**/dashboard.html*');
  await expect(page.url()).toContain('dashboard.html');
});
