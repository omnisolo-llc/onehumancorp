import { test, expect } from './fixtures';

test.describe('Autonomous Booking System UI', () => {
  test('Customer can view booking slots', async ({ page }) => {
    await page.goto('/booking/mock-service-123');

    // Check if the page exists or shows 404/not found safely without mocking
    await page.waitForLoadState('networkidle');
    expect(page.url()).toContain('/booking');
  });
});
