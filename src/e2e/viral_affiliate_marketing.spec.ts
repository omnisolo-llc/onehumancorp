import { test, expect } from './fixtures';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Viral Affiliate Marketing', () => {
  test('should allow customer to sign up as affiliate and track commission', async ({ browser }) => {
    const page = await browser.newPage();
    const context = page.context();

    // Grant clipboard permissions
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('http://mock/dashboard.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'dashboard.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('http://mock/affiliate-dashboard.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'affiliate-dashboard.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    // We mock the API route to return a consistent mock for the E2E test, as we are testing the UI flow.
    await page.route('/api/v1/growth/affiliate/generate-link', async route => {
      await route.fulfill({ json: { affiliate_link: 'http://example.com/ref/maya20', affiliate_code: 'maya20' } });
    });

    await page.route('/api/v1/growth/affiliate/stats', async route => {
        await route.fulfill({ json: { clicks: 10, conversions: 2 } });
    });

    // Navigate to the Dashboard
    await page.goto('http://mock/dashboard.html');

    // Navigate to the Affiliate Dashboard via the link
    const link = page.locator('#affiliate-dashboard-link');
    await expect(link).toBeVisible();
    await link.click();

    // Verify page loads
    await expect(page).toHaveURL(/.*affiliate-dashboard\.html/);
    await expect(page.getByRole('heading', { name: 'Affiliate Dashboard' }).first()).toBeVisible();

    // Verify stats loaded
    await expect(page.locator('#clicks-stat')).toHaveText('10');
    await expect(page.locator('#conversions-stat')).toHaveText('2');

    // Fill form
    await page.locator('#customerId').fill('maya');
    await page.locator('#discountPercentage').fill('20');
    await page.locator('#commissionPercentage').fill('20');

    // Click Generate Link
    await page.click('#generate-affiliate');

    // Verify link is shown
    const linkContainer = page.locator('#affiliate-link-container');
    await expect(linkContainer).toBeVisible();
    await expect(page.locator('#affiliate-link')).toHaveValue('http://example.com/ref/maya20');

    // Test copy button
    const copyBtn = page.locator('#copy-btn');
    await expect(copyBtn).toBeVisible();

    // Set up a listener for clipboard
    await page.evaluate(() => {
        Object.assign(navigator, {
            clipboard: {
                writeText: () => Promise.resolve()
            }
        });
    });

    await copyBtn.click();
    await expect(copyBtn).toHaveText('Copied!');

    await page.close();
  });
});