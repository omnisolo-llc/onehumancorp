import { test, expect } from '@playwright/test';

test.describe('Expert Team Workflow', () => {

  test.beforeEach(async ({ page }) => {
    // 1. Log in using standard user flow
    await page.goto('http://localhost:8080/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password');
    await page.click('button:has-text("Sign in")');
    await page.waitForURL('http://localhost:8080/dashboard');
  });

  test('User can execute a task through the collaborative expert team', async ({ page }) => {
    await page.goto('http://localhost:8080/expert-team');
    await expect(page.locator('h1')).toHaveText('Collaborative Expert Team');
    const taskInput = page.locator('textarea');
    await taskInput.fill('Write a comprehensive business plan for a new vegan bakery... Chart: Required. Analysis: Deep. Words: We need it long.');
    const executeButton = page.getByRole('button', { name: 'Execute Task via Expert Team' });
    await executeButton.click();
    await expect(page.getByRole('button', { name: 'Orchestrating Expert Team...' })).toBeDisabled();
  });

  test('Execute button is disabled when task input is empty', async ({ page }) => {
    await page.goto('http://localhost:8080/expert-team');
    const executeButton = page.getByRole('button', { name: 'Execute Task via Expert Team' });
    await expect(executeButton).toBeDisabled();
  });

  test('Execute button becomes enabled after typing in task context', async ({ page }) => {
    await page.goto('http://localhost:8080/expert-team');
    const executeButton = page.getByRole('button', { name: 'Execute Task via Expert Team' });
    await expect(executeButton).toBeDisabled();
    await page.locator('textarea').fill('Test task context');
    await expect(executeButton).toBeEnabled();
  });

  test('User instructions are clearly visible on the page', async ({ page }) => {
    await page.goto('http://localhost:8080/expert-team');
    const instructions = page.getByText('Lead Agent will coordinate 5 domain experts');
    await expect(instructions).toBeVisible();
    await expect(page.getByText('Business Task Context')).toBeVisible();
  });

  test('Textarea contains a helpful placeholder', async ({ page }) => {
    await page.goto('http://localhost:8080/expert-team');
    const textarea = page.locator('textarea');
    await expect(textarea).toHaveAttribute('placeholder', /e.g. Write a comprehensive business plan/);
  });
});
