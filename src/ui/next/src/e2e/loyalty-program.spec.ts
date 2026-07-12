import { test, expect } from '../../../../e2e/fixtures';


test.describe('Automated Loyalty Campaign Growth Loop', () => {
  test('should generate a loyalty program VIP email via AI', async ({ page }) => {
    // Start at the dashboard
    await page.goto('/dashboard');

    // Navigate to the Loyalty Program page
    await page.click('text=Customer Loyalty');

    // Verify we are on the right page
    await expect(page.locator('h1')).toContainText('Customer Loyalty Program 🤝');

    // Verify the empty state is visible
    await expect(page.locator('text=Configure your rules and click Generate')).toBeVisible();

    // The default values are 10 and 10 with percentage. Let's change them to test input handling.
    const giveInput = page.locator('input').nth(0);
    const getInput = page.locator('input').nth(1);
    const select = page.locator('select');

    await select.selectOption('fixed'); // Change to fixed amount
    await giveInput.fill('15');
    await getInput.fill('20');

    // Generate the email
    await page.click('button:has-text("Generate Email")');

    // Wait for the generation to complete and the preview to appear
    await expect(page.locator('text=Email Draft Preview')).toBeVisible();

    // Verify the generated email contains the inputs
    const textarea = page.locator('textarea');
    await expect(textarea).toBeVisible({ timeout: 10000 });
    const emailContent = await textarea.inputValue();

    // Check for standard phrases to ensure generation worked correctly
    expect(emailContent).toContain('VIP Loyalty Program');
    expect(emailContent).toContain('$15 in store credit');
    expect(emailContent).toContain('$20 in store credit');
    expect(emailContent).toContain('⚡ Powered by OHC');
  });
});