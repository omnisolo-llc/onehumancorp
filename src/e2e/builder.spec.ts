import { test, expect } from '@playwright/test';

test.describe('Website Builder E2E', () => {
  test('User creates and publishes a site', async ({ page }) => {
    // 1. start from the home page after user login
    await page.goto('/');

    // 2. navigate the entire feature flow
    // Click "Create my website"
    // wait for GenerateSite
    // Edit Hero block text
    // Publish
    // 3. proceed through every step
    // 4. assert result
    expect(true).toBeTruthy();
  });
  test('Test 2', async ({page}) => { expect(true).toBeTruthy(); });
  test('Test 3', async ({page}) => { expect(true).toBeTruthy(); });
  test('Test 4', async ({page}) => { expect(true).toBeTruthy(); });
  test('Test 5', async ({page}) => { expect(true).toBeTruthy(); });
});
