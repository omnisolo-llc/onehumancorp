import { test, expect } from '../../../../e2e/fixtures';

test.describe('Pydantic-First Tool Schema Validation', () => {
  test('should successfully validate correct tool payload', async ({ page }) => {
    test.setTimeout(60000);
    await page.goto('/pydantic-validation');

    await expect(page.getByRole('heading', { name: 'Pydantic-First Tool Schema Validation' })).toBeVisible();

    await page.selectOption('select', 'TopicRetrieve');
    await page.fill('textarea', '{\n  "topic_name": "architecture"\n}');

    await page.getByRole('button', { name: 'Validate Tool Payload' }).click();

    await expect(page.getByTestId('success-message')).toBeVisible();
    await expect(page.locator('text=Validation Successful')).toBeVisible();
  });

  test('should show recoverable error when missing required field', async ({ page }) => {
    test.setTimeout(60000);
    await page.goto('/pydantic-validation');

    await page.selectOption('select', 'TopicRetrieve');
    await page.fill('textarea', '{\n  "wrong_field": "test"\n}');

    await page.getByRole('button', { name: 'Validate Tool Payload' }).click();

    await expect(page.getByTestId('error-message')).toBeVisible();
    await expect(page.locator('text=LLM-Recoverable Validation Error')).toBeVisible();
    await expect(page.locator('text=Validation Error (Pydantic-first tool schema): missing field `topic_name`')).toBeVisible();
    await expect(page.locator('text=This error would be automatically fed back to the LLM for self-correction.')).toBeVisible();
  });

  test('should show recoverable error when type is incorrect', async ({ page }) => {
    test.setTimeout(60000);
    await page.goto('/pydantic-validation');

    await page.selectOption('select', 'TopicRetrieve');
    await page.fill('textarea', '{\n  "topic_name": 123\n}');

    await page.getByRole('button', { name: 'Validate Tool Payload' }).click();

    await expect(page.getByTestId('error-message')).toBeVisible();
    await expect(page.locator('text=LLM-Recoverable Validation Error')).toBeVisible();
    await expect(page.locator('text=Semantic validation failed. Expected string for `topic_name`.')).toBeVisible();
  });

  test('should show error when JSON is invalid', async ({ page }) => {
    test.setTimeout(60000);
    await page.goto('/pydantic-validation');

    await page.selectOption('select', 'TopicRetrieve');
    await page.fill('textarea', '{ invalid json }');

    await page.getByRole('button', { name: 'Validate Tool Payload' }).click();

    await expect(page.getByTestId('error-message')).toBeVisible();
    await expect(page.locator('text=Validation Failed')).toBeVisible();
    await expect(page.locator('text=Invalid JSON format in payload')).toBeVisible();
  });
});
