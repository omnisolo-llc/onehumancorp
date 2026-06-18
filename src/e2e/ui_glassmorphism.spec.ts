import { test, expect } from './fixtures';

test.describe('Premium Glassmorphism UI Validations', () => {
  test('Agent Protocol UI has premium glassmorphism layout', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agent-protocol');
    await expect(page.locator('.glass-card').first()).toBeVisible();
    await expect(page.locator('.bg-white\\/50').first()).toBeVisible();
  });

  test('Actor Model UI has premium glassmorphism layout', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/actor-model');
    await expect(page.locator('.glass-card').first()).toBeVisible();
    await expect(page.locator('.backdrop-blur-xl').first()).toBeHidden(); // Hidden until error
  });

  test('Scaling UI has premium glassmorphism layout', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/scaling');
    await expect(page.locator('.glass-panel').first()).toBeVisible();
    await expect(page.locator('.glass-card').first()).toBeVisible();
  });

  test('Agent Protocol inputs interact correctly', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agent-protocol');
    const input = page.locator('input[placeholder="New Task Input..."]');
    await expect(input).toBeEnabled();
  });

  test('Scaling interactions verify visual workflow scale updates', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/scaling');
    await expect(page.locator('text=3 agents')).toBeVisible();
  });
});
