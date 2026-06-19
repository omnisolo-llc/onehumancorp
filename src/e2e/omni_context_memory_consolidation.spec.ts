import { test, expect } from './fixtures';

test.describe.skip('Omni Context Memory Consolidation', () => {
  test('Agents page loads without errors', async ({ page, loginAs,  }) => {
    await loginAs(page);
    await page.goto('/agents.html');

    // Basic smoke test to ensure the page doesn't crash
    await expect(page.getByRole('heading', { name: 'My Agents', exact: false })).toBeVisible();
  });

  test('Memory panel renders properly', async ({ page, loginAs,  }) => {
    await loginAs(page);
    await page.goto('/agents.html');

    // It should render the consolidated memory section header
    await expect(page.getByText('Consolidated Memory')).toBeVisible();
  });

  test('Memory list displays empty state', async ({ page, loginAs,  }) => {
    await loginAs(page);
    await page.goto('/agents.html');

    // Since the database is freshly spun up, there will be no memories
    await expect(page.getByText('No consolidated memories found.')).toBeVisible();
  });

  test('Memory detail subtitle is visible', async ({ page, loginAs,  }) => {
    await loginAs(page);
    await page.goto('/agents.html');

    await expect(page.getByText('Review and override what AI agents remember about your business.')).toBeVisible();
  });

  test('Explore panel is visible', async ({ page, loginAs,  }) => {
    await loginAs(page);
    await page.goto('/agents.html');

    await expect(page.getByText('Explore Templates')).toBeVisible();
    await expect(page.getByText('Make my version with one click.').first()).toBeVisible();
  });
});
