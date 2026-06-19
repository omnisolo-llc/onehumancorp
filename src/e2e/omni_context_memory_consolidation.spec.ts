import { test, expect } from './fixtures';

test.describe('Omni Context Memory Consolidation', () => {
  test('Agents page loads without errors', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/agents');

    // Basic smoke test to ensure the page doesn't crash
    await expect(page.getByRole('heading', { name: 'Agents', exact: false })).toBeVisible();
  });

  test('Memory panel renders properly', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/agents');

    await expect(page.getByRole('heading', { name: 'Agents', exact: false })).toBeVisible();
    await page.locator('nav').getByRole('button', { name: 'Memory', exact: true }).click();

    // Wait for the panel to be visible
    await expect(page.locator('h2', { hasText: 'Consolidated Memory' }).first()).toBeVisible({ timeout: 5000 });
  });

  test('Memory list displays empty state', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/agents');

    await expect(page.getByRole('heading', { name: 'Agents', exact: false })).toBeVisible();
    await page.locator('nav').getByRole('button', { name: 'Memory', exact: true }).click();

    await expect(page.locator('h2', { hasText: 'Consolidated Memory' }).first()).toBeVisible({ timeout: 5000 });

    // Since the database is freshly spun up, there will be no memories
    await expect(page.getByText('No consolidated memories found.')).toBeVisible();
  });

  test('Memory detail subtitle is visible', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/agents');

    await expect(page.getByRole('heading', { name: 'Agents', exact: false })).toBeVisible();
    await page.locator('nav').getByRole('button', { name: 'Memory', exact: true }).click();

    await expect(page.locator('h2', { hasText: 'Consolidated Memory' }).first()).toBeVisible({ timeout: 5000 });

    await expect(page.getByText('Review and override what AI agents remember about your business.')).toBeVisible();
  });

  test('Explore panel is visible', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/agents');

    await expect(page.getByRole('heading', { name: 'Agents', exact: false })).toBeVisible();
    await page.locator('nav').getByRole('button', { name: 'Templates', exact: true }).click();

    await expect(page.locator('h2', { hasText: 'Explore Templates' }).first()).toBeVisible({ timeout: 5000 });
    await expect(page.getByText('Make my version with one click.').first()).toBeVisible();
  });
});
