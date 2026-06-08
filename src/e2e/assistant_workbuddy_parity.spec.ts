import { test, expect } from './fixtures';

test.describe('Assistant WorkBuddy Parity', () => {
  test('renders the assistant shell with core capability tabs', async ({ page }) => {
    // Navigate from dashboard to assistant to follow full CUJ from logged-in state
    await page.goto('/dashboard');
    // We expect there to be a way to get to assistant from dashboard eventually.
    // For now we navigate directly as requested in the route setup.
    await page.getByRole('link', { name: 'Open WorkBuddy Assistant' }).click();

    // Left rail
    await expect(page.getByRole('navigation').first()).toBeVisible();
    await expect(page.getByText('Task List')).toBeVisible();

    // Center conversation
    await expect(page.getByText('Conversation')).toBeVisible();

    // Right rail results
    await expect(page.getByText('Results Panel')).toBeVisible();
    await expect(page.getByRole('button', { name: 'All Files' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Changes', exact: true })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Preview', exact: true })).toBeVisible();

    // Top actions
    await expect(page.getByRole('link', { name: 'Expert Center' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Remote Control' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Memory' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Automations' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Skills' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connectors' })).toBeVisible();
  });

  test('can interact with the task composer', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'Open WorkBuddy Assistant' }).click();

    const promptInput = page.getByLabel('Task prompt');
    await promptInput.fill('Research next.js features and output a markdown file');

    const startButton = page.getByRole('button', { name: 'Start Task' });
    await expect(startButton).toBeVisible();
    await startButton.click();

    // Once we click Start Task, it's typically creating a new running task
    // It should appear in the Conversation area or the Task List.
    await expect(page.getByRole('heading', { name: 'Research next.js features and output a markdown file' })).toBeVisible();
  });

  test('can access Parity Audit Panel', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'Open WorkBuddy Assistant' }).click();

    await page.getByRole('button', { name: 'Parity Audit' }).click();
    const parityPanel = page.getByLabel('Parity audit panel');
    await expect(parityPanel).toBeVisible();
    await expect(parityPanel.getByText('implemented').first()).toBeVisible();
  });

  test('can interact with Cloud Runtime controls', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'Open WorkBuddy Assistant' }).click();

    await page.getByRole('button', { name: 'Cloud Runtime' }).click();
    const cloudPanel = page.getByLabel('Cloud runtime panel');
    await expect(cloudPanel).toBeVisible();
    await expect(cloudPanel.getByText('Background Session')).toBeVisible();
  });

  test('can interact with the results panel tabs', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'Open WorkBuddy Assistant' }).click();

    // Click on All Files
    await page.getByRole('button', { name: 'All Files' }).click();
    await expect(page.getByRole('button', { name: 'All Files' })).toHaveAttribute('aria-pressed', 'true');

    // Click on Changes
    await page.getByRole('button', { name: 'Changes', exact: true }).click();
    await expect(page.getByRole('button', { name: 'Changes', exact: true })).toHaveAttribute('aria-pressed', 'true');
  });
});
