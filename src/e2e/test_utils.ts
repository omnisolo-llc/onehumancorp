import { Page } from '@playwright/test';

export async function setupTestEnv() {}
export async function teardownTestEnv() {}
export async function loginAsE2eTenant(page: Page) {
  await page.goto('/login');
}
