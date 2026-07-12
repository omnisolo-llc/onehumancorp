import { test, expect } from './fixtures';

test.describe('Master Catalog B.10 / C.4: Verification Loops UI integration', () => {
  test('CUJ: Verification Loops Dashboard shows available sensors and guides', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);

    // Navigate to the verification loops UI
    await page.goto('/verification-loops');

    // The page should exist and load
    await expect(page.getByRole('heading', { name: /Verification Loops/i })).toBeVisible({ timeout: 10000 });

    // It should display the two primary components: Guides and Sensors
    await expect(page.getByText(/Guides \(Feedforward\)/i)).toBeVisible();
    await expect(page.getByText(/Sensors \(Feedback\)/i)).toBeVisible();

    // Check for specific UI elements representing the Verification Loops
    await expect(page.getByText(/LLM Judge/i)).toBeVisible();
    await expect(page.getByText(/Visual Verifier/i)).toBeVisible();
    await expect(page.getByText(/Computational Guide/i)).toBeVisible();

    // Test interacting with one of the tools
    const llmJudgeInput = page.getByPlaceholder(/Task definition for LLM Judge/i).first();
    await expect(llmJudgeInput).toBeVisible();
    await llmJudgeInput.fill('Evaluate code quality');

    const outputInput = page.getByPlaceholder(/Output to evaluate/i).first();
    await expect(outputInput).toBeVisible();
    await outputInput.fill('function test() { return 1; }');

    const runButton = page.getByRole('button', { name: /Run LLM Judge/i }).first();
    await runButton.click();

    // Wait for the result
    await expect(page.locator('.verification-result').first()).toBeVisible({ timeout: 15000 });
  });

  test('CUJ: Verification Loops Dashboard displays Computational Guide option', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/verification-loops');
    await expect(page.getByRole('button', { name: /Run Computational Guide/i })).toBeVisible();
  });

  test('CUJ: Verification Loops Dashboard displays Visual Verifier option', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/verification-loops');
    await expect(page.getByRole('button', { name: /Run Visual Verifier/i })).toBeVisible();
  });

  test('CUJ: Verification Loops Dashboard displays Inferential Sensor option', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/verification-loops');
    await expect(page.getByRole('button', { name: /Run LLM Judge/i }).first()).toBeVisible();
  });

  test('CUJ: Verification Loops inputs capture text correctly', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/verification-loops');

    const taskContextInput = page.getByPlaceholder(/Task definition for LLM Judge/i).first();
    await taskContextInput.fill('Check spelling');
    await expect(taskContextInput).toHaveValue('Check spelling');

    const outputInput = page.getByPlaceholder(/Output to evaluate/i).first();
    await outputInput.fill('ok');
    await expect(outputInput).toHaveValue('ok');
  });
});
