import { test, expect } from '@playwright/test';

test.describe('CEO Dashboard & HR/Ops', () => {
  test.beforeEach(async ({ page, request }) => {
    // Login manually with the dev credentials to seed the token as instructed by the repository memory
    const loginRes = await request.post('http://127.0.0.1:8080/api/auth/login', {
      data: { username: 'admin', password: 'admin' },
    });

    expect(loginRes.ok()).toBeTruthy();
    const data = await loginRes.json();
    const token = data.token;

    // Navigates to the frontend
    await page.goto('/');

    // Inject token for auth bypass exactly as instructed by memory
    await page.evaluate(`window.localStorage.setItem('flutter.auth_token', '${token}')`);

    // Reload to apply token
    await page.reload();
  });

  test('Dashboard loads and displays initial empty state or auto-seeded data', async ({ page }) => {

    // Hide cursor to meet Aesthetic Excellence mandate for screenshots
    await page.addStyleTag({ content: 'body { cursor: none !important; }' });

    // Wait for data to load
    await expect(page.locator('h1').filter({ hasText: 'Dashboard' })).toBeVisible({ timeout: 10000 });

    // Click on HR & Ops
    await page.locator('li').filter({ hasText: 'HR & Ops' }).click();

    // Verify HR & Ops header
    await expect(page.locator('h1').filter({ hasText: 'HR & Operations' })).toBeVisible();

    // Take screenshot of HR & Ops list
    await page.screenshot({ path: 'tests/screenshots/hr-ops-dashboard.png' });

    // Click Hire Agent
    await page.locator('button').filter({ hasText: '+ Hire Agent' }).click();

    // Verify Modal
    await expect(page.locator('h2').filter({ hasText: 'Hire New Agent' })).toBeVisible();

    // Fill the form
    await page.locator('input[type="text"]').fill('Playwright Test Engineer');
    await page.locator('select').first().selectOption('SOFTWARE_ENGINEER');

    // Take screenshot of modal
    await page.screenshot({ path: 'tests/screenshots/hire-agent-modal.png' });

    // Submit the form
    await page.locator('button').filter({ hasText: 'Deploy Agent' }).click();

    // Wait for API call to complete by checking for a delay or an element
    await page.waitForTimeout(1000);

    // Verify the new agent is in the list
    await expect(page.locator('h3').filter({ hasText: 'Playwright Test Engineer' })).toBeVisible();

    // Take screenshot of updated list
    await page.screenshot({ path: 'tests/screenshots/hr-ops-dashboard-updated.png' });
  });
});