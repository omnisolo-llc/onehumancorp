import { test, expect } from '../fixtures';

test.describe('Nora Autonomous Proposal Intake Flow', () => {
  test('Client intake creates proposal automatically via UI', async ({ page }) => {
    await page.goto('/proposals');
    const heading = page.getByRole('heading', { name: 'Proposals' });
    await expect(heading).toBeVisible();

    const newProposalBtn = page.getByRole('button', { name: 'New Proposal' });
    await newProposalBtn.click();

    await page.getByPlaceholder('Client Name').fill('Ada Baker');
    await page.getByPlaceholder('Project Scope').fill('Website Redesign & Branding');
    await page.getByRole('button', { name: 'Create Proposal' }).click();

    await expect(page.getByText('Website Redesign & Branding').first()).toBeVisible();
  });
});
