import { test, expect } from './fixtures';

test.describe('Viral Trial Extension Loop on Dashboard Page', () => {
  test('should display Soft Paywall and trial extension flow', async ({ page }) => {
    await page.goto('/dashboard');

    // Trigger the soft paywall
    await page.getByRole('button', { name: /Send 12 Review Requests/ }).click();

    // Verify modal content
    await expect(page.getByRole('heading', { name: 'Unlock AI Power' })).toBeVisible();
    await expect(page.getByText('Automated AI Review Requests are a Pro feature.')).toBeVisible();

    // The dialog tells us "Thank you for sharing! Your 7-day Pro trial has been activated.", accept it
    page.on('dialog', dialog => dialog.accept());

    // Click trial extension
    await page.getByRole('button', { name: 'Share on X to get 7 Days Free' }).click();

    // Soft paywall should be closed
    await expect(page.getByRole('heading', { name: 'Unlock AI Power' })).not.toBeVisible();
  });
});
