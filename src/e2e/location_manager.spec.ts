import { test, expect } from '@playwright/test';

test.describe('Location Manager Escalation Flow', () => {
  test('Jun can view local tasks and escalate an issue', async ({ page }) => {
    // Navigate to the location manager dashboard
    await page.goto('/location-dashboard');

    // Verify dashboard elements
    await expect(page.locator('h1')).toHaveText('Location Dashboard');

    // Verify tasks are present
    await expect(page.getByText('Restock coffee beans')).toBeVisible();
    await expect(page.getByText('Fix receipt printer')).toBeVisible();

    // Verify staff on shift
    await expect(page.getByText('Alice')).toBeVisible();
    await expect(page.getByText('Barista')).toBeVisible();

    // Verify active alert and escalate
    const alertMessage = page.getByText('3 customer complaints regarding slow pickup in the last hour.');
    await expect(alertMessage).toBeVisible();

    const escalateBtn = page.getByRole('button', { name: 'Escalate to Owner' }).first();
    await expect(escalateBtn).toBeVisible();
    await escalateBtn.click();

    // Verify modal appears
    await expect(page.getByText('Escalate Issue')).toBeVisible();
    await expect(page.getByText('The Operations Agent is preparing a summary for the owner.')).toBeVisible();

    // Wait for the agent to finish drafting
    await page.waitForTimeout(2000);

    // Verify draft text
    const draftTextarea = page.locator('textarea');
    await expect(draftTextarea).toBeVisible();
    const draftValue = await draftTextarea.inputValue();
    expect(draftValue).toContain('Spike in pickup complaints at Location A');

    // Submit escalation
    await page.getByRole('button', { name: 'Send to Owner' }).click();

    // Verify modal closes and alert is removed (in this test implementation)
    await expect(page.getByText('Escalate Issue')).not.toBeVisible();
  });
});
