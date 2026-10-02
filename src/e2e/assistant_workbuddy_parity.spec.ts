import { test, expect } from './fixtures';

test.describe('Assistant WorkBuddy Parity', () => {
  test('renders the assistant shell with core capability tabs', async ({ page }) => {
    // Navigate from dashboard to assistant to follow full CUJ from logged-in state
    await page.goto('/dashboard');
    // We expect there to be a way to get to assistant from dashboard eventually.
    // For now we navigate directly as requested in the route setup.
    await page.getByRole('link', { name: 'Assistant Tasks' }).first().click();

    // Left rail
    await expect(page.getByRole('navigation').first()).toBeVisible();
    await expect(page.getByText('Task List').first()).toBeVisible();

    // Center conversation
    await expect(page.getByText('Conversation').first()).toBeVisible();

    // Right rail results
    await page.getByRole('button', { name: 'Results' }).first().click();
    await expect(page.getByText('Results Panel').first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'All Files' }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Changes', exact: true }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Preview', exact: true }).first()).toBeVisible();

    // Top actions
    await expect(page.getByRole('link', { name: 'Expert Center' }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Remote Control' }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Memory' }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Automations' }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Skills' }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connectors' }).first()).toBeVisible();
  });

  test('can interact with the task composer', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'Assistant Tasks' }).first().click();

    await page.getByRole('button', { name: 'New Task' }).first().click();
    const promptInput = page.getByLabel('Task prompt');
    await promptInput.fill('Research next.js features and output a markdown file');

    const startButton = page.getByRole('button', { name: 'Start Task' }).first();
    await expect(startButton).toBeVisible();
    await startButton.click();

    // Once we click Start Task, it's typically creating a new running task
    // It should appear in the Conversation area or the Task List.
    await expect(page.getByRole('heading', { name: 'Research next.js features and output a markdown file' })).toBeVisible();
  });

  test('can access Parity Audit Panel', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'Assistant Tasks' }).first().click();

    await page.getByRole('button', { name: 'Parity Audit' }).first().click();
    const parityPanel = page.getByLabel('Parity audit panel');
    await expect(parityPanel).toBeVisible();
    await expect(parityPanel.getByText('implemented').first()).toBeVisible();
  });

  test('can interact with Cloud Runtime controls', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'Assistant Tasks' }).click();

    await page.getByRole('button', { name: 'Cloud Runtime' }).click();
    const cloudPanel = page.getByLabel('Cloud runtime panel');
    await expect(cloudPanel).toBeVisible();
    await expect(cloudPanel.getByText('Background Session')).toBeVisible();
  });

  test('can interact with the results panel tabs', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'Assistant Tasks' }).click();

    await page.getByRole('button', { name: 'Results' }).first().click();
    // Click on All Files
    await page.getByRole('button', { name: 'All Files' }).click();
    await expect(page.getByRole('button', { name: 'All Files' })).toHaveAttribute('aria-pressed', 'true');

    // Click on Changes
    await page.getByRole('button', { name: 'Changes', exact: true }).click();
    await expect(page.getByRole('button', { name: 'Changes', exact: true })).toHaveAttribute('aria-pressed', 'true');
  });
});
