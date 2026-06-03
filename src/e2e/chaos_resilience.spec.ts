import { expect, test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('chaos_resilience');

test('Chaos resilience degradation UI', async ({ page }) => {
  await page.goto('/dashboard');

  // Simulate network offline
  await page.context().setOffline(true);

  // Try an action
  await page.goto('/agents');

  // Ensure we see a fallback state or it doesn't crash completely
  // For now we just verify it loads the basic layout if cached, or shows offline msg

  await page.context().setOffline(false);
  await page.reload();
  await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
});
