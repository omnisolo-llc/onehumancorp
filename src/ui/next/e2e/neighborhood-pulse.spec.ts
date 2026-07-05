import { test, expect } from '@playwright/test';

test.describe('Neighborhood Pulse Dashboard UI', () => {
  test('should display neighborhood pulse card when neighbors are found', async ({ page }) => {
    // Intercept API call to mock a response that returns neighbors
      }
    });

    // Navigate to dashboard
    await page.goto('/dashboard');

    // Wait for the Neighborhood Pulse card to load and display
    const pulseCardTitle = page.locator('text=Neighborhood Pulse');
    await expect(pulseCardTitle).toBeVisible();

    // Verify neighbors appear
    await expect(page.locator('text=Carlos Repairs')).toBeVisible();
    await expect(page.locator('text=Fatima Food Cart')).toBeVisible();
    await expect(page.locator('text=There are 2 OHC businesses in your area')).toBeVisible();

    // Setup an alert dialog handler since our mock uses `alert`
    let dialogMessage = '';
    page.on('dialog', dialog => {
      dialogMessage = dialog.message();
      dialog.accept();
    });

    // Click invite partner
    const inviteButton = page.locator('button:has-text("Invite Partner")').first();
    await inviteButton.click();

    // Wait for the alert to trigger and verify the text
    await page.waitForTimeout(500);
    expect(dialogMessage).toBe('Invitation sent successfully!');
  });
});
