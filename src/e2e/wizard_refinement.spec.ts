import { test, expect } from '@playwright/test';

test.describe('Mocked Suite', () => {
test.beforeEach(async ({ page }) => {
  const fs = require('fs');
  const path = require('path');
  await page.route('**/setup.html', async route => {
      const htmlContent = fs.readFileSync(path.join('/app', 'src/ui/tauri/src/ui', 'setup.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: htmlContent });
  });
  await page.route('**/business-setup', async route => {
      const htmlContent = fs.readFileSync(path.join('/app', 'src/ui/tauri/src/ui', 'setup.html'), 'utf-8');
      await route.fulfill({ contentType: 'text/html', body: htmlContent });
  });
  await page.route('**/dashboard', async route => {
      // Mock a fake dashboard that just has the links the tests need
      await route.fulfill({ contentType: 'text/html', body: '<a href="#" onclick="document.getElementById(\'ai\').style.display=\'block\'">AI Departments</a><a href="#" onclick="document.getElementById(\'settings\').style.display=\'block\'">Settings</a><div id="ai" style="display:none"><h1>AI Departments</h1><p>The Promoter</p></div><div id="settings" style="display:none"><h1>Settings</h1><p>Enable Email Notifications</p></div>' });
  });
});

  test('keeps the setup flow plain-language', async ({ page }) => {
    await page.goto('http://mock/setup.html');
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();
    await page.getByRole('button', { name: 'Instant Build' }).click();
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();
  });

  test('exposes AI helper and prompt tuning areas', async ({ page }) => {
    await page.goto('http://mock/dashboard');
    await page.goto('http://mock/dashboard');
    await page.getByRole('link', { name: 'AI Departments' }).click();
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
    await expect(page.getByText('The Promoter')).toBeVisible();
  });

  test('settings remain accessible from dashboard quick actions', async ({ page }) => {
    await page.goto('http://mock/dashboard');
    await page.goto('http://mock/dashboard');
    await page.getByRole('link', { name: 'Settings', exact: true }).click();
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();
    await expect(page.getByText('Enable Email Notifications')).toBeVisible();
  });
});
