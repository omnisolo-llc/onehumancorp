import { test, expect } from '@playwright/test';

test.describe('Voice Receptionist Settings Provisioning', () => {
  test('should allow provisioning a new voice number', async ({ page }) => {
    // 1. Navigate to settings
    await page.goto('/settings');

    // 2. Enable Voice Receptionist
    const checkbox = page.getByRole('checkbox', { name: /Enable AI Voice Receptionist/i });
    await expect(checkbox).toBeVisible();
    await checkbox.check();

    // 3. Verify elements are visible
    const getNumberBtn = page.getByRole('button', { name: /Get Number/i });
    await expect(getNumberBtn).toBeVisible();

    // 4. Provision Number
    await getNumberBtn.click();

    // 5. Verify number is provisioned
    const input = page.getByRole('textbox', { name: /Assigned Phone Number/i });
    await expect(input).toBeVisible();
    await expect(input).not.toHaveValue('Not assigned');
    await expect(input).not.toHaveValue('');
    const value = await input.inputValue();
    expect(value).toMatch(/^\+1555123\d{4}$/);
  });
});
