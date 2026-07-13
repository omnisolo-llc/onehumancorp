import { test, expect } from '../../../../e2e/fixtures';

test.describe('Exit-Intent Pop-up Builder', () => {
  test.use({ bypassCSP: true });
  test('should update preview dynamically, copy embed code, and trigger paywall', async ({ page, context }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);
    // 1. Navigate to the builder page
    await page.goto('http://localhost:3000/exit-intent-builder');

    // 2. Assert page loaded correctly
    await expect(page.getByRole('heading', { name: 'Exit-Intent Pop-up Builder' })).toBeVisible({ timeout: 15000 });

    // 3. Edit input fields
    await page.getByPlaceholder('Wait! Before you go...').fill('Special Limited Time Offer!');
    await page.getByPlaceholder('Get 10% off your first order...').fill('Get 10% off your first order when you sign up for our newsletter.');

    // 4. Assert Live Preview reflects changes
    await expect(page.locator('.max-w-4xl').filter({ hasText: 'Live Preview' }).getByRole('heading', { name: 'Special Limited Time Offer!' })).toBeVisible();

    // 5. Test Copy Embed Code logic
    await page.getByRole('button', { name: 'Copy to Clipboard' }).click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // 6. Test Paywall logic
    // Ensure "Remove OHC Branding" toggle is present and clickable
    const brandingToggle = page.getByRole('switch');
    await expect(brandingToggle).toBeVisible();
    await brandingToggle.click();

    // Ensure the paywall modal opens
    const upgradeButton = page.getByRole('button', { name: 'Upgrade to Pro' });
    await expect(upgradeButton).toBeVisible();

    // Click "Upgrade to Pro" to close modal and simulate upgrade state
    await upgradeButton.click();

    // Ensure modal closes and toggle switches to on
    await expect(upgradeButton).toBeHidden();
    await expect(brandingToggle).toHaveAttribute('aria-checked', 'true');
  });
});
