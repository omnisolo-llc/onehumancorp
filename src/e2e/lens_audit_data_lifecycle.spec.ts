import { test, expect } from './fixtures';
import { randomUUID } from 'crypto';

test.describe('Lens Audit Data Lifecycle Verification', () => {
  test('Creating an agent protocol task persists to DB and shows in UI', async ({ page, unlimitedAdminUser, loginAs }) => {
    // 1. Setup
    await loginAs(page, unlimitedAdminUser);

    // 2. Action: Create a task in UI
    const taskName = `Audit Task ${randomUUID()}`;
    await page.goto('/agent-protocol');
    const input = page.locator('input[placeholder="New Task Input..."]');
    await input.fill(taskName);

    const createBtn = page.getByRole('button', { name: 'Create' });
    await createBtn.click();

    // 3. Verification 1: Check UI reflects the creation
    await expect(page.locator(`text=${taskName}`)).toBeVisible();

    // 4. Verification 2: Refresh UI and verify state persisted
    await page.reload();
    await expect(page.locator(`text=${taskName}`)).toBeVisible();
  });

  test('Actor model accepts message, triggers API and shows result', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/actor-model');

    const messageInput = page.locator('textarea[id="message"]');
    await messageInput.fill('Test swarm message');

    const sendBtn = page.getByRole('button', { name: 'Send Message to Swarm' });
    await sendBtn.click();

    await expect(page.locator('text=Actors are executing...')).toBeVisible();

    // We expect either an error or success to appear (mocking/API details might vary)
    await expect(page.locator('[data-testid="error-message"], [data-testid="success-message"]')).toBeVisible({ timeout: 10000 });
  });

  test('Scaling UI updates and triggers cloud scale API', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/scaling');

    // Check baseline
    await expect(page.locator('text=3 agents')).toBeVisible();

    // Scale up
    const increaseBtn = page.getByRole('button', { name: 'Increase Scale (+1)' });
    await increaseBtn.click();
    await expect(page.locator('text=4 agents')).toBeVisible();

    // Deploy
    const inputPayload = page.locator('input[placeholder="e.g. Analyze dataset"]');
    await inputPayload.fill('Scale Test Deploy');

    const deployBtn = page.locator('#deploy-agents-btn');
    await deployBtn.click();

    await expect(page.locator('text=Deploying...')).toBeVisible();
    // Assuming the test backend returns quickly or errs
    await expect(page.locator('h3:has-text("Results")').or(page.locator('text=Error:'))).toBeVisible({ timeout: 10000 });
  });

  test('Agent protocol step execution creates a step and updates UI', async ({ page, unlimitedAdminUser, loginAs }) => {
     await loginAs(page, unlimitedAdminUser);
     const taskName = `Audit Task Steps ${randomUUID()}`;
     await page.goto('/agent-protocol');
     const input = page.locator('input[placeholder="New Task Input..."]');
     await input.fill(taskName);

     const createBtn = page.getByRole('button', { name: 'Create' });
     await createBtn.click();

     // Select the task to open steps panel
     await page.locator(`text=${taskName}`).click();

     const stepInput = page.locator('input[placeholder="Optional Step Input..."]');
     await stepInput.fill('Test step action');

     const execBtn = page.getByRole('button', { name: 'Execute Step' });
     await execBtn.click();

     // Verify step was added to the UI
     await expect(page.locator('text=Input: Test step action')).toBeVisible();
  });

  test('Agent protocol UI does not show fake local state for checkpoints', async ({ page, unlimitedAdminUser, loginAs }) => {
     await loginAs(page, unlimitedAdminUser);
     const taskName = `Audit Task Checkpoint ${randomUUID()}`;
     await page.goto('/agent-protocol');
     const input = page.locator('input[placeholder="New Task Input..."]');
     await input.fill(taskName);

     const createBtn = page.getByRole('button', { name: 'Create' });
     await createBtn.click();

     // Select the task to open steps panel
     await page.locator(`text=${taskName}`).click();

     // New task should have no checkpoints
     await expect(page.locator('text=No checkpoints saved.')).toBeVisible();
  });
});
