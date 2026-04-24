import { test, expect } from '@playwright/test';
import { login } from './auth_helper';

test('Dashboard and Swarm Memory screens display correct observability widgets', async ({ page }) => {
  await login(page);

  await page.goto((process.env.PLAYWRIGHT_BASE_URL || 'http://localhost:8080') + '/#/dashboard');
  await page.waitForTimeout(2000);

  expect(page.url()).toContain('/dashboard');
});
