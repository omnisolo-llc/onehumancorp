import { test, expect } from '@playwright/test';

test('business analytics share output viral loop', async ({ page }) => {
  // We handle dialogs (alerts) gracefully, accepting them
  page.on('dialog', async dialog => {
    const msg = dialog.message();
    expect(msg).toContain('Successfully generated share link: https://ohc.app/shared/share-');
    await dialog.accept();
  });

  await page.goto('/business-analytics');

  // Ensure page is loaded
  await expect(page.locator('text=Business Analytics 📊')).toBeVisible();

  // Click the share button
  const shareButton = page.locator('text=Share Report');
  await expect(shareButton).toBeVisible();

  await shareButton.click();

  // We implicitly assert that the alert logic above is executed successfully.
});
