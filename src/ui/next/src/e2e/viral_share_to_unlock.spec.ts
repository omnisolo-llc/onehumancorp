import { test, expect } from './fixtures';

test.describe('Viral Share to Unlock - Digital Business Card', () => {

  test('Shows soft paywall and allows unlock via share', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    // Navigate to the digital business card generator using relative path
    await page.goto('/ui/digital-business-card.html');

    // Wait for the input to be visible and then fill it
    const nameInput = page.locator('#input-name');
    await expect(nameInput).toBeVisible();
    await nameInput.fill('Test User');

    // Click the "Remove Powered by OHC branding" checkbox
    const removeBrandingCheckbox = page.locator('#input-remove-branding');
    // Using evaluate since standard click on checkbox label can sometimes be tricky
    await removeBrandingCheckbox.evaluate((node: HTMLInputElement) => { node.click(); });

    // The Soft Paywall should appear
    await expect(page.locator('text=Upgrade to Pro').first()).toBeVisible();
    await expect(page.locator('text=Share to Unlock for Free').first()).toBeVisible();

    // Wait for the modal animation
    await page.waitForTimeout(300);

    // Some tests fail because the locator might match multiple things or it opens a new page and the old one closes too fast.
    // Instead of waiting for a popup, we can stub window.open to prevent the actual navigation, which might break tests
    await page.evaluate(() => {
        window.open = () => null;
    });

    await page.click('text=Share to Unlock for Free');

    // The modal should close
    await expect(page.locator('text=Share to Unlock for Free')).not.toBeVisible();

    // The checkbox should be checked
    const checkbox = page.locator('#input-remove-branding');
    await expect(checkbox).toBeChecked();
  });
});
