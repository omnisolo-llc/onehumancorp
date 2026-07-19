import { test, expect } from '@playwright/test';

test.describe('Staff Manager CUJ', () => {
  test('Manager views shifts, simulates event, staff completes it, manager generates summary', async ({ page }) => {
    // Navigate to the main dashboard
    await page.goto('/');

    // Wait for dashboard to load and click on Manager Dashboard
    await page.waitForSelector('h1:has-text("Work Command Center")');
    await page.click('text=Manager Dashboard');

    // Verify Manager Dashboard loaded
    await expect(page.locator('h1:has-text("Manager View (Jun)")')).toBeVisible();

    // Verify Initial Active Shifts are shown (or empty state if not seeded, but we should see the title)
    await expect(page.locator('h2:has-text("Active Shifts")')).toBeVisible();

    // Simulate Business Event (Inventory Low)
    await page.click('button:has-text("Simulate Business Event (Inventory Low)")');

    // Wait a brief moment for the refetch to complete
    await page.waitForTimeout(1000);

    // Verify the new task appears in the manager view
    await expect(page.locator('text=Simulated Event: Low Inventory').first()).toBeVisible();

    // Navigate to Staff Dashboard
    await page.goto('/');
    await page.click('text=Staff Dashboard');

    // Verify Staff Dashboard loaded
    await expect(page.locator('h1:has-text("My Shifts & Tasks")')).toBeVisible();

    // Find the pending task we just created and mark it complete
    await page.locator('div', { hasText: 'Simulated Event: Low Inventory' }).locator('input[type="checkbox"]').first().click({force: true});

    // Wait a brief moment for the patch to complete
    await page.waitForTimeout(1000);

    // Verify it's marked as complete by checking the line-through class or checked state
    // We expect the text element to have line-through
    const completedText = page.locator('span.line-through', { hasText: 'Simulated Event: Low Inventory' }).first();
    await expect(completedText).toBeVisible();

    // Navigate back to Manager Dashboard
    await page.goto('/');
    await page.click('text=Manager Dashboard');
    await expect(page.locator('h1:has-text("Manager View (Jun)")')).toBeVisible();

    // Handle alert for summary generation
    page.once('dialog', dialog => {
      expect(dialog.message()).toBe('Shift Summary Generated!');
      dialog.accept();
    });

    // Generate End of Shift Summary
    await page.click('button:has-text("Generate End of Shift Summary")');

    // We wait briefly to ensure the summary is generated
    await page.waitForTimeout(1000);
  });
});
