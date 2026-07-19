import { test, expect } from '@playwright/test';

test.describe('Customer Subscription Portal', () => {
  // Use a fixed id for testing
  const subscriptionId = 'sub_test_123';

  test.beforeEach(async ({ page }) => {
    // Navigate directly to the mock customer portal
    await page.goto(`/customer/subscriptions/${subscriptionId}`);

    // Wait for the mock data to load (simulated 500ms delay)
    // await expect(page.getByText('Loading your subscription...')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Manage Subscription' })).toBeVisible({ timeout: 10000 });
  });

  test('displays subscription details correctly', async ({ page }) => {
    await expect(page.getByText('Artisan Coffee Blend')).toBeVisible();
    await expect(page.getByText('Monthly')).toBeVisible();
    await expect(page.getByText('Active').first()).toBeVisible();
    await expect(page.getByText('$21.60')).toBeVisible();
    // Next delivery date from mock
    await expect(page.getByText('2023-11-15')).toBeVisible();
  });

  test('allows customer to skip next delivery', async ({ page }) => {
    const skipButton = page.getByRole('button', { name: 'Skip Next Delivery' });
    await expect(skipButton).toBeEnabled();
    await skipButton.click();

    // Verify the simulated network request and state update (1000ms delay)
    await expect(page.getByText('Your next delivery has been skipped.')).toBeVisible({ timeout: 5000 });
    // Next delivery date updated in mock
    await expect(page.getByText('2023-12-15')).toBeVisible();
  });

  test('allows customer to pause subscription', async ({ page }) => {
    const pauseButton = page.getByRole('button', { name: 'Pause Subscription' });
    await expect(pauseButton).toBeEnabled();
    await pauseButton.click();

    // Verify the simulated network request and state update (1000ms delay)
    await expect(page.getByText('Your subscription has been paused.')).toBeVisible({ timeout: 5000 });
    await expect(page.getByText('Paused').first()).toBeVisible();

    // The button should now say "Subscription Paused" and be disabled
    const pausedButton = page.getByRole('button', { name: 'Subscription Paused' });
    await expect(pausedButton).toBeDisabled();

    // Skip should also be disabled if paused
    const skipButton = page.getByRole('button', { name: 'Skip Next Delivery' });
    await expect(skipButton).toBeDisabled();
  });

  test('allows customer to cancel subscription', async ({ page }) => {
    const cancelButton = page.getByRole('button', { name: 'Cancel Subscription' });
    await expect(cancelButton).toBeEnabled();
    await cancelButton.click();

    // Verify the simulated network request and state update (1000ms delay)
    await expect(page.getByText('Your subscription has been cancelled.')).toBeVisible({ timeout: 5000 });
    await expect(page.getByText('Cancelled').first()).toBeVisible();

    // Next delivery should be replaced by '-'
    const nextDeliveryRow = page.locator('div').filter({ hasText: /^Next Delivery-$/ });
    await expect(nextDeliveryRow).toBeVisible();

    // Action buttons should disappear, replaced by cancellation text
    await expect(page.getByRole('button', { name: 'Pause Subscription' })).not.toBeVisible();
    await expect(page.getByRole('button', { name: 'Skip Next Delivery' })).not.toBeVisible();
    await expect(page.getByText('You have cancelled this subscription.')).toBeVisible();
  });
});
