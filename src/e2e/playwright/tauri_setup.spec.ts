import { test, expect } from '@playwright/test';
import * as path from 'path';

test('Tauri Setup UI completes the flow and has domain choice/auto-respond', async ({ page }) => {
  // Load the tauri html file via file:// protocol
  const filePath = path.resolve(__dirname, '../../ui/tauri/src/ui/setup.html');
  await page.goto(`file://${filePath}`);

  // It should show the initial step
  await expect(page.locator('#step-initial')).toHaveClass(/active/);

  // Navigate to Context Step
  await page.locator('#step-initial .next-step-btn').click();
  await expect(page.locator('#step-context')).toHaveClass(/active/);

  // Fill Context (Choose Handyman)
  await page.locator('.persona-chip', { hasText: "I'm a Handyman" }).click();

  // Next
  await page.locator('#step-context .next-step-btn').click();
  await expect(page.locator('#step-categories')).toHaveClass(/active/);

  // Next
  await page.locator('#step-categories .next-step-btn').click();
  await expect(page.locator('#step-name')).toHaveClass(/active/);

  // Next
  await page.locator('#step-name .next-step-btn').click();
  await expect(page.locator('#step-assistant')).toHaveClass(/active/);

  // Next
  await page.locator('#step-assistant .next-step-btn').click();
  await expect(page.locator('#step-admin')).toHaveClass(/active/);

  // Fill Admin
  await page.locator('#admin-email').fill('test@example.com');
  await page.locator('#admin-password').fill('test12345');

  // Verify domain choice exists
  const domainChoice = page.locator('#domain-choice');
  await expect(domainChoice).toBeVisible();
  await domainChoice.selectOption('custom');

  // Verify toggle exists
  const aiToggle = page.locator('#ai-auto-respond');
  // it is hidden by CSS, so we just check it is attached
  await expect(aiToggle).toBeAttached();
  // Uncheck
  await page.locator('.slider').click();
  await expect(aiToggle).not.toBeChecked();

  // Next
  await page.locator('#step-admin .next-step-btn').click();
  await expect(page.locator('#step-offer')).toHaveClass(/active/);

  // Next
  await page.locator('#step-offer .next-step-btn').click();
  await expect(page.locator('#step-template')).toHaveClass(/active/);

  // Fill Template
  await page.locator('#template-selection').selectOption('Modern');

  // Mock fetch
  await page.route('**/api/onboarding/start', async route => {
    const request = route.request();
    expect(request.method()).toBe('POST');
    const postData = JSON.parse(request.postData() || '{}');
    expect(postData.domain_choice).toBe('custom');
    expect(postData.auto_respond).toBe(false);
    expect(postData.admin_email).toBe('test@example.com');

    await route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({ organization_id: "org_123" }),
    });
  });

  // Finish
  await page.locator('#finish-btn').click();
  // It redirects to success.html
  await page.waitForURL(/success\.html$/);
});
