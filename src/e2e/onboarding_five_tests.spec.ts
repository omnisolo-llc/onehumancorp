import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Additional Five Tests', () => {
  test('Test 1: Navigate first step', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.getByText("Setup Assistant")).toBeVisible();
    await page.getByRole('button', { name: 'Start My Business' }).click();
    await expect(page.getByText("What's the name of your business?")).toBeVisible();
  });

  test('Test 2: Complete first two steps', async ({ page }) => {
    await page.goto('/onboarding');
    await page.getByRole('button', { name: 'Start My Business' }).click();
    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test2 Bakery');
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.getByText("What do you sell?")).toBeVisible();
  });

  test('Test 3: Complete up to Location', async ({ page }) => {
    await page.goto('/onboarding');
    await page.getByRole('button', { name: 'Start My Business' }).click();
    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test3 Bakery');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Test2 Bakery goods');
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.getByText("Where are you located?")).toBeVisible();
  });

  test('Test 4: Verify Instant Build is available', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.getByRole('button', { name: 'Instant Build' })).toBeVisible();
  });

  test('Test 5: Check empty values in Instant Build', async ({ page }) => {
    await page.goto('/onboarding');
    await page.getByRole('button', { name: 'Instant Build' }).click();
    await expect(page.getByText("Tell us about your business")).toBeVisible();
    await page.getByRole('button', { name: 'Generate Storefront' }).click();
    await expect(page.getByText('Please tell us about your business.')).toBeVisible();
  });
});
