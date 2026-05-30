import { test, expect } from './fixtures';

test.describe('Viral Team Invite Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the Team page where the growth loop is implemented
    await page.goto('/team');
    await page.waitForLoadState('networkidle');
  });

  test('should display invite team member modal and generate a viral link', async ({ page }) => {
    // 1. Locate the "Invite Team Member" button in the header
    const inviteBtn = page.getByRole('button', { name: /Invite Team Member/i });
    await expect(inviteBtn).toBeVisible();

    // 2. Click to trigger the modal
    await inviteBtn.click();

    // 3. Verify the modal appears
    const modalHeading = page.getByRole('heading', { name: 'Invite Collaborator' });
    await expect(modalHeading).toBeVisible();
    await expect(page.getByText('Invite a team member to collaborate')).toBeVisible();

    // 4. Fill in the email input
    const emailInput = page.getByPlaceholder('colleague@example.com');
    await expect(emailInput).toBeVisible();
    await emailInput.fill('test@example.com');

    // 5. Submit the form to generate the link
    const generateBtn = page.getByRole('button', { name: 'Generate Invite Link' });
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // 6. Verify the link is generated and shown
    await expect(generateBtn).toBeHidden();
    const linkInput = page.locator('input[readonly]');
    await expect(linkInput).toBeVisible();
    await expect(linkInput).toHaveValue(/ohc:\/\/join\?ref=team_invite/);

    // 7. Verify the copy button is present
    const copyBtn = page.getByRole('button', { name: 'Copy Link' });
    await expect(copyBtn).toBeVisible();

    // 8. Test closing the modal
    const closeBtn = page.locator('div.absolute').locator('button').first();
    await closeBtn.click();
    await expect(modalHeading).toBeHidden();
  });
});
