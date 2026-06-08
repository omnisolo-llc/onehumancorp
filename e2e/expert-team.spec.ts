import { test, expect } from '@playwright/test';

test.describe('Expert Team Feature CUJ', () => {

  test('Persona: Business Owner uses Expert Team successfully', async ({ page }) => {
    await page.goto('/expert-team');
    await expect(page.getByRole('heading', { name: /Expert Team/i })).toBeVisible();

    // 2. Owner enters task
    const input = page.getByPlaceholder(/e.g. Write a comprehensive/i);
    await input.fill('Write a comprehensive business plan for a new vegan bakery... Chart: Required. Analysis: Deep.');

    // 3. Setup interception for the API route to avoid real LLM calls in E2E but simulate the real backend behavior
    // Important: we mock the internal NextJS api route so we don't depend on actual backend network requests running!
    await page.route('/api/expert-team', async route => {
      const json = { result: "Combined Executive Summary:\nIndustry Researcher: Done.\nFinancial Analyst: Done.\nStrategic Analyst: Done.\nProcess Supervisor: Done.\nQuality Auditor: Done.\n\nOverall Strategy:\nProceed based on above.\nChart: Included.\nAnalysis: Completed.\n\n" + " word".repeat(20000) };
      await route.fulfill({ json });
    });

    // 4. Execute
    const button = page.getByRole('button', { name: /Execute Task/i });
    await button.click();

    // 5. Loading state is transient, we just wait for the result

    // 6. Verify result is shown
    await expect(page.getByText('Combined Executive Summary:')).toBeVisible();
    await expect(page.getByText('Chart: Included.')).toBeVisible();
  });

  test('Persona: Owner sees error when Pre-flight gate fails', async ({ page }) => {
    await page.goto('/expert-team');

    // Fill empty task (the UI requires some text to enable button)
    await page.getByPlaceholder(/e.g. Write a comprehensive/i).fill(' ');

    await page.route('/api/expert-team', async route => {
      await route.fulfill({ status: 400, json: { error: 'Pre-flight Gate Failed: Task context cannot be empty.' } });
    });

    await page.getByRole('button', { name: /Execute Task/i }).click();

    await expect(page.getByText('Quality Gate or Execution Error:')).toBeVisible();
    await expect(page.getByText('Pre-flight Gate Failed: Task context cannot be empty.')).toBeVisible();
  });

  test('Persona: Owner sees error when Pre-merge similarity is too high', async ({ page }) => {
    await page.goto('/expert-team');

    await page.getByPlaceholder(/e.g. Write a comprehensive/i).fill('Analyze something basic');

    await page.route('/api/expert-team', async route => {
      await route.fulfill({ status: 400, json: { error: 'Pre-merge Gate Failed: High similarity detected (>75%) between expert outputs.' } });
    });

    await page.getByRole('button', { name: /Execute Task/i }).click();

    await expect(page.getByText('Pre-merge Gate Failed: High similarity detected (>75%) between expert outputs.')).toBeVisible();
  });

  test('Persona: Owner sees error when Pre-deliver missing chart/analysis', async ({ page }) => {
    await page.goto('/expert-team');

    await page.getByPlaceholder(/e.g. Write a comprehensive/i).fill('Do not include chart');

    await page.route('/api/expert-team', async route => {
      await route.fulfill({ status: 400, json: { error: 'Pre-deliver Gate Failed: Missing required chart/analysis verification in final output.' } });
    });

    await page.getByRole('button', { name: /Execute Task/i }).click();

    await expect(page.getByText('Pre-deliver Gate Failed: Missing required chart/analysis verification in final output.')).toBeVisible();
  });

  test('Persona: Owner sees error when Pre-deliver missing words', async ({ page }) => {
    await page.goto('/expert-team');

    await page.getByPlaceholder(/e.g. Write a comprehensive/i).fill('Short answer');

    await page.route('/api/expert-team', async route => {
      await route.fulfill({ status: 400, json: { error: 'Pre-deliver Gate Failed: Final output is too short (5 words). Required >= 20000 words for delivery.' } });
    });

    await page.getByRole('button', { name: /Execute Task/i }).click();

    await expect(page.getByText('Required >= 20000 words for delivery.')).toBeVisible();
  });
});
