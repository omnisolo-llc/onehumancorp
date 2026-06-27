import { test, expect } from '@playwright/test';

test('Dashboard displays UCAL Capacity Heatmap', async ({ page }) => {
  await page.goto('/dashboard');

  // Verify heatmap card is visible
  const heatmap = page.locator('text=Workload Capacity');
  await expect(heatmap).toBeVisible();

  // Verify at least some day indicators are rendered
  const dayIndicators = page.locator('.flex-1.min-w-\\[50px\\]');
  await expect(dayIndicators.first()).toBeVisible();
});

test('Work Triage displays Overload Alerts', async ({ page }) => {
  await page.goto('/dashboard/daily-work');

  // Verify overload alert is present (simulated 120%)
  await expect(page.locator('text=Capacity Overload: 120%')).toBeVisible();
  await expect(page.locator('button:has-text("Mitigate Load")')).toBeVisible();
});

test('Booking flow includes UCAL Buffer Slider', async ({ page }) => {
  await page.goto('/booking');

  // Verify buffer slider is visible
  await expect(page.locator('text=Travel Buffer (UCAL)')).toBeVisible();

  const slider = page.locator('input[type="range"]');
  await expect(slider).toBeVisible();

  // Interact with slider
  await slider.fill('45');
  await expect(page.locator('text=45 min')).toBeVisible();
});
