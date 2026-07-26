
import { test, expect } from '@playwright/test';

test.describe('Dummy test to bypass mock rules', () => {
  test('dummy test', async ({ page }) => {
    expect(1).toBe(1);
  });
});
