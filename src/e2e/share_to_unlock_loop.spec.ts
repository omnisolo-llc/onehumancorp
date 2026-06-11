import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('share_to_unlock_loop');

test.describe('Share-to-Unlock Growth Loop', () => {
    test('generator page renders, copies link, and public page reveals code after share', async ({ page, adminUser, loginAs }) => {
        // Step 1: Login
        await loginAs(page, adminUser);

        // Step 2: Navigate to Dashboard
        await page.goto('/dashboard');

        // Verify the new dashboard link exists and click it
        const unlockLink = page.locator('a[href="/share-to-unlock-generator"]');
        await expect(unlockLink).toBeVisible();
        await unlockLink.click();

        // Step 3: Verify Generator Page
        await page.waitForURL('**/share-to-unlock-generator');
        await expect(page.getByRole('heading', { name: /Share-to-Unlock Generator/ })).toBeVisible();

        // Configure the campaign
        await page.fill('input[placeholder="e.g. Secret Weekend Deal"]', 'E2E Unlock Test');
        await page.fill('input[placeholder="e.g. 20% Off Your Entire Order"]', 'Free E2E Shipping');
        await page.fill('input[placeholder="e.g. SECRET20"]', 'FREE_E2E');

        // Wait briefly for local storage tenant effect
        await page.waitForTimeout(500);

        // Verify Preview Panel updates
        await expect(page.locator('h2', { hasText: 'E2E Unlock Test' })).toBeVisible();
        await expect(page.locator('p', { hasText: 'Free E2E Shipping' })).toBeVisible();
        await expect(page.locator('span', { hasText: 'FREE_E2E' })).toBeVisible();

        // Check if "Powered by OHC" is visible on preview
        await expect(page.getByText('⚡ Powered by OHC').first()).toBeVisible();

        // Step 4: Validate generated link
        const copyButton = page.getByRole('button', { name: 'Copy Link' });
        await expect(copyButton).toBeVisible();

        // Test copy interaction
        await copyButton.click();
        await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

        // Construct the generated URL from the visible UI string to test the public page
        // Wait for state updates first
        await page.waitForTimeout(500);

        // We can just construct it manually to test the public page since reading clipboard from playwright is finicky
        const publicUrl = `/unlock?tenant=test-tenant&title=${encodeURIComponent('E2E Unlock Test')}&reward=${encodeURIComponent('Free E2E Shipping')}&code=${encodeURIComponent('FREE_E2E')}&msg=test&theme=light`;

        // Step 5: Test the Public Unlock Route
        await page.goto(publicUrl);

        // Wait for page to load
        await expect(page.locator('h1', { hasText: 'E2E Unlock Test' })).toBeVisible();
        await expect(page.locator('strong', { hasText: 'Free E2E Shipping' })).toBeVisible();

        // Verify the code is blurred/locked initially
        const codeElement = page.locator('span', { hasText: 'FREE_E2E' });
        await expect(codeElement).toBeVisible();
        await expect(codeElement).toHaveClass(/filter blur-md/);
        await expect(page.getByText('Locked')).toBeVisible();

        // Ensure "Powered by OHC" referral link is present
        const poweredByLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(poweredByLink).toBeVisible();
        await expect(poweredByLink).toHaveAttribute('href', '/api/v1/growth/referrals/click?target=/onboarding&ref=test-tenant');

        // Step 6: Mock the Share action to "Unlock"
        // Mock window.open to prevent actually opening Twitter/WhatsApp
        await page.evaluate(() => {
            window.open = function() { return null; };
        });

        const shareBtn = page.getByRole('button', { name: 'Share on X' });
        await expect(shareBtn).toBeVisible();
        await shareBtn.click();

        // Wait for the simulated setTimeout to run (1.5s)
        await page.waitForTimeout(1600);

        // Step 7: Verify Unlocked State
        await expect(page.getByText('Congratulations! Here is your reward:')).toBeVisible();

        // Code should no longer be blurred
        await expect(codeElement).toHaveClass(/blur-none/);

        // Locked badge should be gone
        await expect(page.getByText('Locked')).toBeHidden();

        // Copy Code button should appear
        const copyCodeBtn = page.getByRole('button', { name: 'Copy Code' });
        await expect(copyCodeBtn).toBeVisible();

        // Click Copy Code
        await copyCodeBtn.click();
        await expect(page.getByRole('button', { name: 'Code Copied!' })).toBeVisible();
    });
});
