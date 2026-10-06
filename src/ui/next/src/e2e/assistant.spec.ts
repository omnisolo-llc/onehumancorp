import { test, expect } from '../../../../e2e/fixtures';
import { createGrowthOwner } from '../../../../e2e/growth_owner';
import { assertUnavailableAssistantComposer } from '../../../../e2e/support/assistant_execution';

test.describe('Assistant Page', () => {
  test('navigates to assistant and verifies authentic unavailable state', async ({ page, baseURL }) => {
    const owner = await createGrowthOwner(page, baseURL);
    await page.goto('/assistant');
    await expect(page.getByTestId('assistant-shell')).toBeVisible();
    await expect(page.getByTestId('assistant-workstation')).toBeVisible();
    await expect(page.getByRole('heading', { name: /Assistant/ })).toBeVisible();
    await expect(page.getByText('Create a personal briefing')).not.toBeVisible();
    await assertUnavailableAssistantComposer(page, owner, 'Summarize only this supplied test report');
    await page.getByRole('button', { name: 'Results', exact: true }).click();
    await expect(page.getByRole('heading', { name: 'Results', exact: true })).toBeVisible();
  });
});
