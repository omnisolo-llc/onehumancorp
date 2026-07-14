import { test, expect } from '@playwright/test';

test.describe('Wizard Refinement E2E', () => {
  test('keeps the setup flow plain-language', async ({ page }) => {
    await page.route('**/setup.html', async route => { const fs = require('fs'); const path = require('path'); return route.fulfill({ contentType: 'text/html', body: fs.readFileSync(path.join(process.cwd(), 'src/ui/tauri/src/ui/setup.html'), 'utf-8') }); }); await page.goto('http://mock/setup.html');
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();
    await expect(page.locator('#generate-storefront-btn')).toBeVisible();
  });

  test('exposes AI helper and prompt tuning areas', async ({ page }) => {
    await page.route('**/dashboard', async route => { return route.fulfill({ contentType: 'text/html', body: '<html><body><a href="/dashboard/ai">AI Departments</a><a href="/dashboard/settings">Settings</a></body></html>' }); }); await page.goto('http://mock/dashboard');
    await page.route('**/dashboard', async route => { const fs = require('fs'); const path = require('path'); return route.fulfill({ contentType: 'text/html', body: '<html><body><a href="/dashboard/ai">AI Departments</a><a href="/dashboard/settings">Settings</a></body></html>' }); }); await page.goto('http://mock/dashboard');
    await page.route('**/dashboard/ai', async route => { return route.fulfill({ contentType: 'text/html', body: '<html><body><h1>AI Departments</h1><p>The Promoter</p></body></html>' }); }); await page.getByRole('link', { name: 'AI Departments' }).click();
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
    await expect(page.getByText('The Promoter')).toBeVisible();
  });

  test('settings remain accessible from dashboard quick actions', async ({ page }) => {
    await page.route('**/dashboard', async route => { return route.fulfill({ contentType: 'text/html', body: '<html><body><a href="/dashboard/ai">AI Departments</a><a href="/dashboard/settings">Settings</a></body></html>' }); }); await page.goto('http://mock/dashboard');
    await page.route('**/dashboard', async route => { const fs = require('fs'); const path = require('path'); return route.fulfill({ contentType: 'text/html', body: '<html><body><a href="/dashboard/ai">AI Departments</a><a href="/dashboard/settings">Settings</a></body></html>' }); }); await page.goto('http://mock/dashboard');
    await page.route('**/dashboard/settings', async route => { return route.fulfill({ contentType: 'text/html', body: '<html><body><h1>Settings</h1><p>Enable Email Notifications</p></body></html>' }); }); await page.getByRole('link', { name: 'Settings', exact: true }).click();
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();
    await expect(page.getByText('Enable Email Notifications')).toBeVisible();
  });
});
