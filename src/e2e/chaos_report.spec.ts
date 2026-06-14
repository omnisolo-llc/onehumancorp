import { test, expect } from '@playwright/test';

test('Chaos Report Dashboard should render and display charts', async ({ page }) => {
  await page.goto('/chaos-report');
  await expect(page.locator('text=System Reliability Report')).toBeVisible();
  await expect(page.locator('text=Latency Distribution')).toBeVisible();
  await expect(page.locator('text=Error Rate Over Time')).toBeVisible();

  // Test Theme Toggle Interaction
  const themeToggle = page.locator('button', { hasText: /Toggle (Dark|Light) Mode/ });
  await expect(themeToggle).toBeVisible();

  const body = page.locator('body');
  const initialText = await themeToggle.textContent();

  if (initialText === 'Toggle Light Mode') {
      await expect(body).toHaveClass(/dark-mode/);
      await themeToggle.click();
      await expect(body).toHaveClass(/light-mode/);
      await expect(themeToggle).toHaveText('Toggle Dark Mode');
  } else {
      await expect(body).toHaveClass(/light-mode/);
      await themeToggle.click();
      await expect(body).toHaveClass(/dark-mode/);
      await expect(themeToggle).toHaveText('Toggle Light Mode');
  }
});
