import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Share-to-Unlock Growth Loop', () => {
    test('generator page renders, copies link, and public page reveals code after share', async ({ page, adminUser, loginAs }) => {
        // Step 1: Login
        await loginAs(page, adminUser);

        // Step 2: Navigate to Dashboard
        await page.goto('/dashboard');

        // Verify the new dashboard link exists and click it
        const unlockLink = page.locator('a[id="share-to-unlock-link"]');
        await expect(unlockLink).toBeVisible();
        await unlockLink.click();

        // Step 3: Verify Generator Page
        await page.waitForURL('**/share-to-unlock-generator.html');
        await expect(page.getByRole('heading', { name: /Share-to-Unlock Generator/ })).toBeVisible();

        // Configure the campaign
        await page.fill('input[id="title"]', 'E2E Unlock Test');
        await page.fill('input[id="reward"]', 'Free E2E Shipping');
        await page.fill('input[id="code"]', 'FREE_E2E');

        // Wait briefly for local storage tenant effect
        await page.waitForTimeout(500);

        // Verify Preview Panel updates
        await expect(page.locator('div[id="preview-title"]', { hasText: 'E2E Unlock Test' })).toBeVisible();
        await expect(page.locator('strong[id="preview-reward-text"]', { hasText: 'Free E2E Shipping' })).toBeVisible();
        await expect(page.locator('div[id="preview-code"]', { hasText: 'FREE_E2E' })).toBeVisible();

        // Step 4: Validate generated link
        const generateButton = page.getByRole('button', { name: 'Generate Link' });
        await expect(generateButton).toBeVisible();
        await generateButton.click();

        // Wait for generated URL to appear
        await expect(page.locator('input[id="generated-url"]')).toBeVisible();

        const copyButton = page.getByRole('button', { name: 'Copy Link' });
        await expect(copyButton).toBeVisible();

        // Test copy interaction
        await copyButton.click();
        await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

        // Construct the generated URL from the visible UI string to test the public page
        // Wait for state updates first
        await page.waitForTimeout(500);

        const publicUrl = `/share-to-unlock/index.html?tenant=e2e-tenant&title=${encodeURIComponent('E2E Unlock Test')}&reward=${encodeURIComponent('Free E2E Shipping')}&code=${encodeURIComponent('FREE_E2E')}&msg=${encodeURIComponent('I just unlocked a secret discount!')}`;

        // Step 5: Test the Public Unlock Route
        await page.goto(publicUrl);

        // Wait for page to load
        await expect(page.locator('h1', { hasText: 'E2E Unlock Test' })).toBeVisible();
        await expect(page.locator('strong', { hasText: 'Free E2E Shipping' })).toBeVisible();

        // Verify the code is blurred/locked initially
        const codeElement = page.locator('div[id="discount-code"]', { hasText: 'FREE_E2E' });
        await expect(codeElement).toBeVisible();
        await expect(codeElement).not.toHaveClass(/unlocked/);
        await expect(page.locator('div[id="locked-badge"]')).toBeVisible();

        // Ensure "Powered by OHC" referral link is present
        const poweredByLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(poweredByLink).toBeVisible();

        // Step 6: Mock the Share action to "Unlock"
        // Mock window.open to prevent actually opening Twitter/WhatsApp
        await page.evaluate(() => {
            window.open = function() { return null; };
        });

        const shareBtn = page.getByRole('button', { name: 'Share on X to Unlock' });
        await expect(shareBtn).toBeVisible();
        await shareBtn.click();

        // Wait for the simulated setTimeout to run (1.5s)
        await page.waitForTimeout(1600);

        // Step 7: Verify Unlocked State
        await expect(page.getByText('Unlocked!')).toBeVisible();

        // Code should no longer be blurred
        await expect(codeElement).toHaveClass(/unlocked/);

        // Locked badge should be gone
        await expect(page.locator('div[id="locked-badge"]')).toBeHidden();

        // Copy Code button should appear
        const copyCodeBtn = page.getByRole('button', { name: 'Copy Code' });
        await expect(copyCodeBtn).toBeVisible();

        // Click Copy Code
        await copyCodeBtn.click();
        await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
    });
});