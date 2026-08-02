import { Page } from '@playwright/test';

export async function setupTestEnv() {
  // Setup environment if needed
}

export async function teardownTestEnv() {
  // Teardown environment if needed
}

export async function loginAsE2eTenant(page: Page) {
  await page.goto('/login');
  await page.getByLabel('Email or username').fill('test@example.com');
  await page.getByLabel('Password').fill('password123');
  await page.getByLabel(/Organization/).fill('e2e-tenant');
  await Promise.all([
    page.waitForURL('**/dashboard'),
    page.getByRole('button', { name: 'Log in' }).click(),
  ]);
}
