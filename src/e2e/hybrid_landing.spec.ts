import { test, expect } from './fixtures';

test.describe('Hybrid Landing Page', () => {
  test('should display Local-First and Cloud options', async ({ page }) => {
    await page.goto('/');

    await expect(page.getByRole('heading', { name: 'OneHumanCorp' })).toBeVisible();
    await expect(page.getByRole('heading', { name: /Hybrid Agentic OS/ })).toBeVisible();

    await expect(page.getByRole('heading', { name: 'Local-First Sovereignty' })).toBeVisible();
    await expect(page.getByText('Zero Cloud Telemetry')).toBeVisible();

    await expect(page.getByRole('heading', { name: 'Cloud Convenience' })).toBeVisible();
    await expect(page.getByText('Seamless Team Expansion')).toBeVisible();

    await expect(page.getByRole('button', { name: 'Start Local Workspace' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Deploy to Cloud' })).toBeVisible();
  });
});
