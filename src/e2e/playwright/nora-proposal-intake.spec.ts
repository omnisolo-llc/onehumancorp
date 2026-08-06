import { test, expect } from '@playwright/test';

test.describe('Nora Proposal Intake via Real Payload', () => {
  test('creates a project naturally', async ({ page }) => {
    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    await page.goto('/projects');
    await page.locator('button:has-text("New Project")').click();
    await page.locator('input[name="title"]').fill('Test Project');
    await page.locator('button:has-text("Save")').click();

    await expect(page.locator('text=Test Project').first()).toBeVisible();
  });
});
