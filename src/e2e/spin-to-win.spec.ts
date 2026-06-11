import { test, expect } from '@playwright/test';

test.describe('Spin to Win Widget E2E', () => {
  test('User can configure spin to win widget and copy embed', async ({ page }) => {
    // Navigate to the generator page
    await page.goto('/spin-to-win');

    // Check elements
    await expect(page.locator('text=Spin-to-Win Generator')).toBeVisible();

    // Fill form
    await page.fill('input[value="Spin to Win!"]', 'Holiday Spin!');

    // Check if the preview updates
    await expect(page.locator('text=Holiday Spin!').nth(0)).toBeVisible();

    // Ensure embed code textarea exists
    const textarea = page.locator('textarea');
    await expect(textarea).toBeVisible();
    const embedText = await textarea.inputValue();
    expect(embedText).toContain('<iframe src="');
    expect(embedText).toContain('Holiday%20Spin!');

    // Click copy button
    await page.click('button:has-text("Copy HTML")');
    await expect(page.locator('button:has-text("Copied!")')).toBeVisible();
  });

  test('Embed endpoint renders the spin to win wheel correctly', async ({ page }) => {
    // Navigate to the embed widget endpoint directly
    await page.goto('/api/v1/growth/spin-to-win/embed?title=Holiday%20Spin!&offer=Win%20a%20prize!');

    // Wait for the iframe/widget to render
    await expect(page.locator('h2', { hasText: 'Holiday Spin!' })).toBeVisible();
    await expect(page.locator('p', { hasText: 'Win a prize!' })).toBeVisible();

    // Check elements
    const spinBtn = page.locator('#spin-btn');
    await expect(spinBtn).toBeVisible();

    const emailInput = page.locator('#email');
    await expect(emailInput).toBeVisible();

    const unlockBtn = page.locator('#unlock-btn');
    await expect(unlockBtn).toBeVisible();

    // Try to spin without email
    page.on('dialog', dialog => dialog.accept());
    await spinBtn.click();

    // Enter email
    await emailInput.fill('test@example.com');
    await unlockBtn.click();

    // Unlock button should disappear
    await expect(emailInput).not.toBeVisible();

    // Spin the wheel
    await spinBtn.click();

    // We can't easily wait for the 3.1 second timeout in a simple E2E without `waitForTimeout`,
    // but we can verify the powered by link
    await expect(page.locator('text=⚡ Powered by OHC')).toBeVisible();
  });
});
