import { test, expect } from '@playwright/test';

test.describe('KAIROS Orchestration Walkthrough', () => {

  test('Test 1: Launch KAIROS Orchestration from Dashboard Menu', async ({ page }) => {
    await page.goto('/');
    await page.fill('input[placeholder="Username"]', 'admin');
    await page.fill('input[placeholder="Password"]', 'password');
    await page.click('button:has-text("Sign In")');

    await expect(page.locator('text=My Business')).toBeVisible();

    await page.click('button:has-text("Menu")');

    await page.click('button:has-text("Automate Work Tour")');

    await expect(page.locator('text=How Your Helpers Work Together')).toBeVisible();
    await expect(page.locator('text=The Helper System')).toBeVisible();
    await expect(page.locator('text=Step 1 of 4')).toBeVisible();
  });

  test('Test 2: Launch KAIROS Orchestration from Quick Actions', async ({ page }) => {
    await page.goto('/');
    await page.fill('input[placeholder="Username"]', 'admin');
    await page.fill('input[placeholder="Password"]', 'password');
    await page.click('button:has-text("Sign In")');

    await expect(page.locator('text=My Business')).toBeVisible();

    await page.click('button:has-text("Automations")');

    await expect(page.locator('text=How Your Helpers Work Together')).toBeVisible();
    await expect(page.locator('text=The Helper System')).toBeVisible();
    await expect(page.locator('text=Step 1 of 4')).toBeVisible();
  });

  test('Test 3: Navigate forward through KAIROS Orchestration steps', async ({ page }) => {
    await page.goto('/');
    await page.fill('input[placeholder="Username"]', 'admin');
    await page.fill('input[placeholder="Password"]', 'password');
    await page.click('button:has-text("Sign In")');

    await expect(page.locator('text=My Business')).toBeVisible();
    await page.click('button:has-text("Automations")');

    await expect(page.locator('text=Step 1 of 4')).toBeVisible();
    await expect(page.locator('text=The Helper System')).toBeVisible();

    await page.click('button:has-text("Next Step")');
    await expect(page.locator('text=Step 2 of 4')).toBeVisible();
    await expect(page.locator('text=1. Shared To-Do List')).toBeVisible();

    await page.click('button:has-text("Next Step")');
    await expect(page.locator('text=Step 3 of 4')).toBeVisible();
    await expect(page.locator('text=2. Instant Messaging')).toBeVisible();

    await page.click('button:has-text("Next Step")');
    await expect(page.locator('text=Step 4 of 4')).toBeVisible();
    await expect(page.locator('text=3. Long-Term Memory')).toBeVisible();
  });

  test('Test 4: Navigate backwards through KAIROS Orchestration steps', async ({ page }) => {
    await page.goto('/');
    await page.fill('input[placeholder="Username"]', 'admin');
    await page.fill('input[placeholder="Password"]', 'password');
    await page.click('button:has-text("Sign In")');

    await expect(page.locator('text=My Business')).toBeVisible();
    await page.click('button:has-text("Automations")');

    await page.click('button:has-text("Next Step")');
    await page.click('button:has-text("Next Step")');
    await expect(page.locator('text=Step 3 of 4')).toBeVisible();
    await expect(page.locator('text=2. Instant Messaging')).toBeVisible();

    await page.click('button:has-text("Previous Step")');
    await expect(page.locator('text=Step 2 of 4')).toBeVisible();
    await expect(page.locator('text=1. Shared To-Do List')).toBeVisible();

    await page.click('button:has-text("Previous Step")');
    await expect(page.locator('text=Step 1 of 4')).toBeVisible();
    await expect(page.locator('text=The Helper System')).toBeVisible();
  });

  test('Test 5: Finish and close the KAIROS Orchestration walkthrough', async ({ page }) => {
    await page.goto('/');
    await page.fill('input[placeholder="Username"]', 'admin');
    await page.fill('input[placeholder="Password"]', 'password');
    await page.click('button:has-text("Sign In")');

    await expect(page.locator('text=My Business')).toBeVisible();
    await page.click('button:has-text("Automations")');

    await page.click('button:has-text("Next Step")'); // Go to step 2
    await page.click('button:has-text("Next Step")'); // Go to step 3
    await page.click('button:has-text("Next Step")'); // Go to step 4

    await expect(page.locator('text=Step 4 of 4')).toBeVisible();
    await expect(page.locator('text=Done')).toBeVisible();

    await page.click('button:has-text("Done")');

    await expect(page.locator('text=How Your Helpers Work Together')).toBeHidden();
    await expect(page.locator('text=My Business')).toBeVisible();
  });
});
