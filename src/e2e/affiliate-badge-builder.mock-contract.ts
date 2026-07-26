import { test, expect } from './fixtures';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Affiliate Badge Builder', () => {
  test('should generate affiliate badge HTML based on inputs', async ({ browser }) => {
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

        await page.route('http://mock/affiliate-badge-builder.html', async route => {
            const content = fs.readFileSync(path.join(tauriUiDir, 'affiliate-badge-builder.html'), 'utf-8');
            await route.fulfill({ contentType: 'text/html', body: content });
        });

    // Navigate to the Dashboard, which contains the Growth section
    await page.goto('http://mock/dashboard.html');

    // Navigate to the Affiliate Badge Builder via the link in the Growth section
    const link = page.locator('#affiliate-badge-link');
    await expect(link).toBeVisible();
    await link.click();

    // Verify page loads
    await expect(page).toHaveURL(/.*affiliate-badge-builder\.html/);
    await expect(page.getByRole('heading', { name: 'Affiliate Badge Builder 💸' })).toBeVisible();

    // Verify default preview text
    const previewText = page.locator('#badgeTextPreview');
    await expect(previewText).toHaveText('Powered by OHC');

    // Change Badge Text
    const textInput = page.locator('#badgeText');
    await textInput.fill('Built with OHC');
    await expect(previewText).toHaveText('Built with OHC');

    // Change Theme to Dark
    const themeSelect = page.locator('#badgeTheme');
    await themeSelect.selectOption('dark');

    // Verify the preview element has the 'dark' class
    const badgeElement = page.locator('#badgeElement');
    await expect(badgeElement).toHaveClass(/dark/);

    // Verify the embed code contains the updated text, theme inline styles, and the referral URL
    const embedCode = page.locator('#embedCode');
    const embedValue = await embedCode.inputValue();

    expect(embedValue).toContain('Built with OHC');
    expect(embedValue).toContain('background-color: #111827'); // Dark theme background
    expect(embedValue).toContain('api/v1/growth/referrals/click?target=/onboarding&ref=e2e-tenant&source=affiliate_badge');

    // Copy HTML Code button
    const copyBtn = page.locator('#copyBtn');
    await expect(copyBtn).toBeVisible();

    // Set up a listener for clipboard
    await page.evaluate(() => {
        // mock clipboard due to insecure origin restrictions in playwright webkit
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
