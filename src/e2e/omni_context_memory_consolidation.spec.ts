import { test, expect } from './fixtures';

test.describe('Omni Context Memory Consolidation', () => {
  test('Agents page loads without errors', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/agents');

    // Basic smoke test to ensure the page doesn't crash
    await expect(page.getByRole('heading', { name: 'My Agents', exact: false })).toBeVisible();
  });

  test('Memory panel renders properly', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/agents');

    // It should render the consolidated memory section header
    await expect(page.getByText('Consolidated Memory')).toBeVisible();
  });

  test('Memory list displays empty state', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/agents');

    // Since the database is freshly spun up, there will be no memories
    await expect(page.getByText('No consolidated memories found.')).toBeVisible();
  });

  test('Memory detail subtitle is visible', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/agents');

    await expect(page.getByText('Review and override what AI agents remember about your business.')).toBeVisible();
  });

  test('Explore panel is visible', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/agents');

    await expect(page.getByText('Explore Templates')).toBeVisible();
    await expect(page.getByText('Make my version with one click.').first()).toBeVisible();
  });
});
