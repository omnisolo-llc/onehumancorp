import { Page, expect } from '@playwright/test';

export async function loginToDashboard(page: Page, email = 'test@example.com', password = 'password123') {
  await page.goto('/login');
  await page.fill('input[placeholder="Email or Username"]', email);
  await page.fill('input[placeholder="Password"]', password);
  await page.click('button:has-text("Login")');
}
