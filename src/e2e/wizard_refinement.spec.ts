import { test, expect } from './fixtures';

test.describe('Wizard Refinement E2E', () => {
  test('keeps the setup flow plain-language and reversible', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByText('Zero tech skills needed. We do the heavy lifting.')).toBeVisible();
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();
    await page.getByRole('button', { name: 'Back' }).click();
    await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' })).toBeVisible();
  });

  test('exposes AI helper and prompt tuning areas', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Manage AI Assistants' }).click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
    await expect(page.getByText('Marketing Pro')).toBeVisible();
  });

  test('settings remain accessible from dashboard quick actions', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Settings', exact: true }).click();
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();
    await expect(page.getByText('Enable Email Notifications')).toBeVisible();
  });

  test('should validate input to prevent empty progression in step 2 and 3', async ({ page }) => {
    await page.goto('/business-setup');

    const startBtn = page.locator('button', { hasText: 'Start My Business' });
    if(await startBtn.isVisible()) {
         await startBtn.click();
    }

    // Playwright listener for alerts
    page.on('dialog', async dialog => {
      await dialog.accept();
    });

    // Step 2 without selecting
    await page.locator('#step-2 button', { hasText: 'Next →' }).click();
    await expect(page.locator('#step-2')).toBeVisible();

    // Select and proceed
    await page.getByRole('button', { name: 'Select Online Store' }).click();

    // Step 3 without text
    await page.locator('#step-3 button', { hasText: 'Next →' }).click();
    await expect(page.locator('#step-3')).toBeVisible();

    // Fill text
    await page.locator('#step-3 input').first().fill("My Store");
    await page.locator('#step-3 button', { hasText: 'Next →' }).click();

    // Assert moved to Step 4
    await expect(page.locator('#step-4')).toBeVisible();
  });

  test('should complete full wizard setup flow successfully', async ({ page }) => {
    await page.goto('/business-setup');
    const startBtn = page.locator('button', { hasText: 'Start My Business' });
    if(await startBtn.isVisible()) {
         await startBtn.click();
    }

    await page.getByRole('button', { name: 'Select Online Store' }).click();

    await page.locator('#step-3 input').first().fill("My E2E Tested Store");
    await page.locator('#step-3 button', { hasText: 'Next →' }).click();

    await page.getByRole('button', { name: 'Next →' }).click(); // Step 4
    await page.getByRole('button', { name: 'Next →' }).click(); // Step 5
    await page.getByRole('button', { name: 'Online' }).click(); // Step 6

    // Step 7 validation check
    await page.locator('#step-7 input[type="text"]').fill('Alice Builder');
    await page.locator('#step-7 input[type="email"]').fill('alice@example.com');
    await page.locator('#step-7 input[type="password"]').fill('secret123');
    await page.getByRole('button', { name: 'Next →' }).click();

    // Step 8 & 9 & 10
    await page.getByRole('button', { name: 'Select Modern Template' }).click();
    await page.getByRole('button', { name: 'Select Free OHC Domain' }).click();
    await page.getByRole('button', { name: 'Publish my business →' }).click();

    // Final state success
    await expect(page.getByRole('heading', { name: '🎉 Success! Your business is live! 🎉' })).toBeVisible();
  });
});
