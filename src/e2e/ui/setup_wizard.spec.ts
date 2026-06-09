import { test, expect } from '@playwright/test';
import * as path from 'path';

test.describe('Setup Wizard Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Clear storage before each test
    await page.evaluate(() => localStorage.clear()).catch(() => {});
  });

  test('should render step 1 and allow selecting work context', async ({ page }) => {
    await page.goto('/setup.html');
    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();
    await page.locator('text=Local Service').first().click();
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.getByRole('heading', { name: "What's your category?" })).toBeVisible();
  });

  test('should show error if continuing without selecting context', async ({ page }) => {
    await page.goto('/setup.html');
    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.locator('#context-error')).toBeVisible();
  });

  test('should allow entering category and navigating back', async ({ page }) => {
    await page.goto('/setup.html');
    await page.locator('text=Local Service').first().click();
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: "What's your category?" })).toBeVisible();
    await page.getByPlaceholder('e.g. Graphic Design').fill('Plumbing');
    await page.getByRole('button', { name: 'Back' }).click();

    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();
  });

  test('should validate business name length', async ({ page }) => {
    await page.goto('/setup.html');
    await page.locator('text=Local Service').first().click();
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder('e.g. Graphic Design').fill('Plumbing');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Ma");
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.locator('#name-error')).toBeVisible();

    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Mario's Plumbing");
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.getByRole('heading', { name: 'Set up your Assistant' })).toBeVisible();
  });

  test('should navigate through the entire wizard and persist state', async ({ page }) => {
    await page.goto('/setup.html');

    // Step 1: Work Context
    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();
    await page.locator('text=Local Service').first().click();
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 2: Categories
    await expect(page.getByRole('heading', { name: "What's your category?" })).toBeVisible();
    await page.getByPlaceholder('e.g. Graphic Design').fill('Plumbing');
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 3: Business Name and Tagline
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Mario's Plumbing");
    await page.getByPlaceholder("Tagline (optional)").fill("We fix leaks fast.");
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 4: Assistant Setup
    await expect(page.getByRole('heading', { name: 'Set up your Assistant' })).toBeVisible();
    await page.getByPlaceholder('e.g. Jarvis').fill('Luigi');
    await page.locator('select#assistant-tone').selectOption('Professional');
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 5: First Offer
    await expect(page.getByRole('heading', { name: 'Your First Offer' })).toBeVisible();
    await page.getByPlaceholder('e.g. Custom Birthday Cake').fill('Pipe Inspection');

    await page.getByRole('button', { name: 'Finish Setup' }).click();

    // Since it's going to navigate to success.html which might not exist on the file system in the exact same directory depending on how Playwright resolves it, we can just wait for it
    await page.waitForURL(/.*success.*/);

    // Assert local storage
    const state = await page.evaluate(() => localStorage.getItem('onboardingState'));
    expect(state).toBeTruthy();

    const parsedState = JSON.parse(state!);
    expect(parsedState.workContext).toBe('Local Service');
    expect(parsedState.categories).toBe('Plumbing');
    expect(parsedState.businessName).toBe("Mario's Plumbing");
    expect(parsedState.tagline).toBe("We fix leaks fast.");
    expect(parsedState.assistantName).toBe('Luigi');
    expect(parsedState.assistantTone).toBe('Professional');
    expect(parsedState.firstOffer).toBe('Pipe Inspection');
  });
});
