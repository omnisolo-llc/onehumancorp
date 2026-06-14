import { test, expect } from '@playwright/test';
import * as path from 'path';

test.describe('Onboarding CUJ - Maya the Baker', () => {
  test('Complete onboarding flow as a non-technical user', async ({ page }) => {


    // Testing the UI directly via fileUrl to ensure our UI works locally. Real E2E is handled by Bazel's start up environment testing.
    const fileUrl = `file://${process.cwd()}/src/ui/tauri/src/ui/setup.html`;
    await page.goto(fileUrl);


    // Initial step: Click "Start Setup" or whatever moves us to context.
    // Looking at setup.html, it starts on #step-initial.
    // It has a next-step-btn.
    await page.locator('#step-initial [data-testid="next-step-btn"]').click();

    // Context Step: Click "I'm a Baker" preset
    await page.locator('[data-testid="persona-baker"]').click();
    await page.locator('#step-context [data-testid="next-step-btn"]').click();

    // Categories Step
    // It should be filled with "Custom Cakes" from preset, let's just click next
    await page.locator('#step-categories [data-testid="next-step-btn"]').click();

    // Name Step
    // It might be filled, but let's ensure it has something
    await page.fill('#business-name', "Maya's Custom Cakes");
    await page.locator('#step-name [data-testid="next-step-btn"]').click();

    // Assistant Step
    await page.locator('#step-assistant [data-testid="next-step-btn"]').click();

    // Admin Step
    await page.fill('#admin-email', "maya@example.com");
    await page.fill('#admin-password', "password123");
    await page.locator('#step-admin [data-testid="next-step-btn"]').click();

    // Offer Step
    await page.locator('#step-offer [data-testid="next-step-btn"]').click();

    // Template Step
    // Check if the finish button is visible
    const finishBtn = page.locator('[data-testid="finish-btn"]');
    await expect(finishBtn).toBeVisible({ timeout: 10000 });
  });
});
