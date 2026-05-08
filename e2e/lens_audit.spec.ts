const { test, expect } = require('@playwright/test');

test.describe('Lens Audit: Database State Verification', () => {
  test('Verify wizard state saves properly to DB', async ({ page }) => {
    await page.goto('/');

    // Simulate user flow starting from home
    await page.click('text=Start Setup Wizard');

    // Wait for the mock currency fetch
    await page.waitForSelector('text=USD');

    await page.fill('input[placeholder="Company Name"]', 'My Audit Company');
    await page.click('text=Next');

    // Verify DB state logic via API mock check
    const response = await page.request.get('/api/v1/wizard/state');
    const state = await response.json();
    expect(state.company_name).toBe('My Audit Company');
  });

  test('Verify currency config logic from database', async ({ page }) => {
    await page.goto('/');

    // Simulate DB interaction in the UI flow
    await page.click('text=Settings');
    await page.click('text=Advanced Mode');

    const response = await page.request.get('/api/v1/wizard/state');
    const state = await response.json();
    expect(state.is_advanced).toBe('true');
  });

  test('Verify welcome checklist completion', async ({ page }) => {
    await page.goto('/welcome');

    // Open a task
    await page.click('text=Open Task');

    // Progress should jump to 100
    await page.waitForSelector('text=100%');
  });

  test('Verify publish updates state', async ({ page }) => {
    await page.goto('/builder');

    await page.click('text=Publish Site');

    // State should update
    const response = await page.request.get('/api/v1/wizard/state');
    const state = await response.json();
    expect(state.is_publishing).toBe('true');
  });

  test('Verify builder updates are saved to database', async ({ page }) => {
    await page.goto('/builder');

    await page.fill('input[name="tagline"]', 'New tagline');
    await page.click('text=Save');

    const response = await page.request.get('/api/v1/wizard/state');
    const state = await response.json();
    expect(state.tagline).toBe('New tagline');
  });
});
