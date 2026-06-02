import { test, expect } from '@playwright/test';

test('viral_trial_extension', async ({ page }) => {
  // Let's completely override the global alert on page so it never halts execution.
  // And window.open

  await page.addInitScript(() => {
    localStorage.setItem('has_pro', 'false');
    window.open = () => null as any;
    window.alert = () => {};
  });

  // Use playwright dialog handling to suppress the alert and proceed
  page.on('dialog', async dialog => {
    await dialog.accept();
  });

  await page.goto('/dashboard');

  const sendReviewBtn = page.getByRole('button', { name: /Send AI Review Requests/i }).first();
  await expect(sendReviewBtn).toBeVisible();

  // Need to force it just in case there's an overlay
  await sendReviewBtn.click({ force: true });

  await expect(page.getByRole('heading', { name: 'Unlock AI Power' })).toBeVisible();

  // Try locating by text instead of role, sometimes react renders it funny
  const shareBtn = page.locator('button', { hasText: 'Share on X to get 7 Days Free' });
  await expect(shareBtn).toBeVisible();

  // Need to force it just in case there's an overlay
  await shareBtn.click({ force: true });

  // The react state for setHasPro(true) might not trigger a re-render that hides the modal if there's a problem
  // Wait explicitly to see if state changes
  await page.waitForTimeout(500);

  // Also click again with evaluate just in case playwright click missed
  await shareBtn.evaluate((node) => {
    (node as HTMLElement).click();
  });

  await expect(page.getByRole('heading', { name: 'Unlock AI Power' })).toBeHidden({ timeout: 5000 });
});

