import { test, expect } from './fixtures';

test.describe('Pydantic-first tool schema validation', () => {
  test('user can submit tool payloads and see LLM-recoverable validation errors via the real backend', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);

    // Go to the pydantic validation test page
    await page.goto('/pydantic-validation');

    // Test 1: Missing field
    await page.locator('select').selectOption('TopicRetrieve');
    await page.locator('textarea').fill('{\n  "wrong_field": "architecture"\n}');
    await page.getByRole('button', { name: 'Validate Tool Payload' }).click();

    await expect(page.getByTestId('error-message')).toBeVisible();
    await expect(page.getByText('LLM-Recoverable Validation Error')).toBeVisible();
    await expect(page.getByTestId('error-message')).toContainText('missing field `topic_name`');

    // Test 2: Invalid type
    await page.locator('textarea').fill('{\n  "topic_name": 12345\n}');
    await page.getByRole('button', { name: 'Validate Tool Payload' }).click();

    await expect(page.getByTestId('error-message')).toBeVisible();
    await expect(page.getByText('LLM-Recoverable Validation Error')).toBeVisible();
    await expect(page.getByTestId('error-message')).toContainText('Semantic validation failed');

    // Test 3: Success
    await page.locator('textarea').fill('{\n  "topic_name": "architecture"\n}');
    await page.getByRole('button', { name: 'Validate Tool Payload' }).click();

    await expect(page.getByTestId('success-message')).toBeVisible();
    await expect(page.getByTestId('success-message')).toContainText('successfully against the schema');
  });
});
