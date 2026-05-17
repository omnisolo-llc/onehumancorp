import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard', () => {
  test('signup routes into setup', async ({ page }) => {
    await page.goto('/signup');
    await page.getByPlaceholder('Email or Username').fill('new@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Sign Up' }).click();

    await expect(page.locator('#setup-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' })).toBeVisible();
  });

  test('guided onboarding preserves entered business state', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Service Business/ }).click();
    await page.getByPlaceholder('What is your business called?').fill('Carlos Repairs');

    await expect(page.getByPlaceholder('What is your business called?')).toHaveValue('Carlos Repairs');
  });

  test('completed onboarding can return to dashboard', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await page.getByRole('button', { name: /Local Business/ }).click();
    await page.getByPlaceholder('What is your business called?').fill('Local Shop');
    await page.getByRole('button', { name: /Next/ }).click();
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
  });
});
