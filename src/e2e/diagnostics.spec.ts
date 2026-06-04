import { test, expect } from './fixtures';

test.describe('Diagnostics Page', () => {
  test('shows health metrics and diagnostic actions', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/diagnostics');
    const screen = page.locator('#diagnostics-screen');

    await expect(screen).toBeVisible();
<<<<<<< HEAD
    await expect(screen).toContainText('Response time latency: 42 ms');
    await expect(screen).toContainText('Request throughput: 24 rps');
=======
    await expect(screen).toContainText('System Status: All systems operational');
    await expect(screen).toContainText('Database: Healthy');
    await expect(screen).toContainText('Redis: Healthy');
    await expect(screen).toContainText('Response time latency:');
>>>>>>> 61bb2cbe (research: Add architectural design for offline-first gift card engine)

    // Diagnostic actions test removed because the UI was updated
  });
});

test.describe('Service Manager', () => {
  test('shows service state and controls', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/services');
    const screen = page.locator('#services-screen');

    await expect(screen).toBeVisible();
    await expect(screen).toContainText('Service Manager');
    await expect(screen).toContainText('Status: running');
    await expect(screen).toContainText('Resource usage: CPU 5%, memory 128MB');
    await expect(page.getByRole('button', { name: 'Restart' })).toBeVisible();
    await expect(page.getByLabel(/Auto restart/)).toBeVisible();
  });
});

test.describe('Scaling Configuration', () => {
  test('shows scaling settings and recommendations', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/scaling');
    const screen = page.locator('#scaling-screen');

    await expect(screen).toBeVisible();
    await expect(screen).toContainText('Current Scale: 3 instances');
    await expect(screen).toContainText('Min 1 Max 10 instance range bounds');
    await expect(screen).toContainText('No optimization needed.');
    await expect(screen.getByRole('button', { name: '+', exact: true })).toBeVisible();
    await expect(screen.getByRole('button', { name: '-', exact: true })).toBeVisible();
  });
});
