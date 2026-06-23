import { test, expect } from '@playwright/test';

test('Mobile-First Zero-Click Conversational Setup Onboarding', async ({ page }) => {
  // Set viewport to mobile (375px wide)
  await page.setViewportSize({ width: 375, height: 667 });

  // Mock the intake, start, and launch APIs to avoid needing a real backend running
  await page.route('/api/onboarding/intake', async route => {
    const json = { business_name: 'Carlos Plumbing', industry: 'Plumbing', services: ['Plumbing Repair'], products: [] };
    await route.fulfill({ status: 200, contentType: 'application/json', json });
  });

  await page.route('/api/onboarding/start', async route => {
    await route.fulfill({ status: 200, contentType: 'application/json', json: { success: true } });
  });

  await page.route('/api/onboarding/launch', async route => {
    await route.fulfill({ status: 200, contentType: 'application/json', json: { success: true } });
  });

  // Navigate to onboarding
  await page.goto('/onboarding');

  // Verify we are on the Setup mode selection screen
  await expect(page.getByText('Conversational Setup')).toBeVisible();

  // Select Conversational Setup
  await page.getByText('Conversational Setup').click();

  // Verify chat interface
  await expect(page.getByText("Hi, I'm your OHC assistant. Tell me about your business.")).toBeVisible();

  // User enters business description
  await page.getByPlaceholder('Type a message...').fill('I am Carlos, I fix plumbing');

  // User clicks Send
  await page.getByRole('button', { name: /send/i }).click();

  // Verify the Translucent Glass loading modal appears
  await expect(page.getByText('Creating your catalog, setting up booking, preparing initial offers...')).toBeVisible();

  // Verify redirect to dashboard
  await expect(page).toHaveURL(/\/dashboard/);
});
