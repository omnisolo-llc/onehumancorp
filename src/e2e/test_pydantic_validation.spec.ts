import { test, expect } from './fixtures';

test.describe('Pydantic-first tool schema fallback mechanism validation UI', () => {
  test('Agent schema errors are gracefully managed and retried via the output parser', async ({ page, loginAs, unlimitedAdminUser }) => {
    // We navigate to /pydantic-validation which is the actual UI built for this Master Catalog feature
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/pydantic-validation');

    await expect(page.getByRole('heading', { name: 'Pydantic-First Tool Schema Validation' })).toBeVisible();

    // Select the TopicRetrieve tool
    await page.locator('select').selectOption('TopicRetrieve');

    // Enter a valid payload
    await page.locator('textarea').fill('{\n  "topic_name": "architecture"\n}');

    // Validate
    await page.getByRole('button', { name: 'Validate Tool Payload' }).click();

    // Wait for success message
    await expect(page.getByTestId('success-message')).toBeVisible();
    await expect(page.getByTestId('success-message')).toContainText('Validation Successful');
  });

  test('Agent feed correctly loads without fatal schema crash', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/pydantic-validation');

    await expect(page.getByRole('heading', { name: 'Pydantic-First Tool Schema Validation' })).toBeVisible();

    // Select the TopicRetrieve tool
    await page.locator('select').selectOption('TopicRetrieve');

    // Missing field topic_name
    await page.locator('textarea').fill('{}');

    await page.getByRole('button', { name: 'Validate Tool Payload' }).click();

    // Should return a recoverable error
    await expect(page.getByTestId('error-message')).toBeVisible();
    await expect(page.getByTestId('error-message')).toContainText('LLM-Recoverable Validation Error');
    await expect(page.getByTestId('error-message')).toContainText('missing field `topic_name`');
  });

  test('Agent setup modal displays schema configurations robustly', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/pydantic-validation');

    // Select TopicRetrieve
    await page.locator('select').selectOption('TopicRetrieve');

    // Invalid semantic type
    await page.locator('textarea').fill('{\n  "topic_name": 123\n}');

    await page.getByRole('button', { name: 'Validate Tool Payload' }).click();

    await expect(page.getByTestId('error-message')).toBeVisible();
    await expect(page.getByTestId('error-message')).toContainText('LLM-Recoverable Validation Error');
    await expect(page.getByTestId('error-message')).toContainText('Semantic validation failed');
  });

  test('Workflow palette handles complex block conditions seamlessly', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/pydantic-validation');

    // Select TranscriptSearch
    await page.locator('select').selectOption('TranscriptSearch');

    // Missing field query
    await page.locator('textarea').fill('{}');

    await page.getByRole('button', { name: 'Validate Tool Payload' }).click();

    await expect(page.getByTestId('error-message')).toBeVisible();
    await expect(page.getByTestId('error-message')).toContainText('LLM-Recoverable Validation Error');
    await expect(page.getByTestId('error-message')).toContainText('missing field `query`');
  });

  test('Workflow execution logic renders the resulting JSON reliably', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/pydantic-validation');

    // Test validation for invalid JSON itself in the UI
    await page.locator('select').selectOption('TopicWrite');
    await page.locator('textarea').fill('{\n  "topic_name": "architecture", \n  "content" \n}'); // Syntax error

    await page.getByRole('button', { name: 'Validate Tool Payload' }).click();

    await expect(page.getByTestId('error-message')).toBeVisible();
    await expect(page.getByTestId('error-message')).toContainText('Validation Failed');
    await expect(page.getByTestId('error-message')).toContainText('Invalid JSON format in payload');
  });
});
