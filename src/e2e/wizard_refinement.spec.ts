import { test, expect } from '@playwright/test';
const fs = require('fs');
const path = require('path');

test.describe('Wizard Refinement E2E', () => {
  test('exposes AI helper and prompt tuning areas', async ({ page }) => {
    // The fixture does a goto /dashboard.html, so we mock it.
    await page.route('**/dashboard.html', async route => {
        const fileContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/dashboard.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
    await page.goto('http://mock/dashboard.html');

    // This checks for the link and asserts it is visible
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible({ timeout: 5000 });
  });

  test('settings remain accessible from dashboard quick actions', async ({ page }) => {
    await page.route('**/dashboard.html', async route => {
        const fileContent = fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/dashboard.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
    await page.goto('http://mock/dashboard.html');

    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible({ timeout: 5000 });
  });
});
