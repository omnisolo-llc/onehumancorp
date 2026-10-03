import { test, expect } from './fixtures';
import { expectGeneratedDraft, expectGenerationUnavailable, fillBuilderBrief, generationAcceptance, generationGateReason, generationPrerequisite, generationResponse, publicationWrites } from './generation-acceptance';

test.describe('Builder Wizard E2E Flow', () => {
  test('saves wizard inputs locally and explains an unconfigured generation provider', async ({ page, loginAs, adminUser }) => {
    test.skip(generationAcceptance, 'This contract requires an unconfigured generation service.');
    await loginAs(page, adminUser);
    await fillBuilderBrief(page, 'Maya Cakes', 'I bake amazing custom cakes.');
    const writes = publicationWrites(page);
    const response = generationResponse(page);
    await page.getByRole('button', { name: 'Build Store' }).click();
    await expectGenerationUnavailable(await response);
    await expect(page.getByText(generationPrerequisite, { exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pick your draft' })).toHaveCount(0);
    await page.reload();
    await expect(page.getByPlaceholder(/mobile dog grooming service/i)).toHaveValue('I bake amazing custom cakes.');
    await expect(page.getByText('Draft saved on this device')).toBeVisible();
    expect(writes).toEqual([]);
  });

  test('owner selects the actual generated draft and opens the editor @provider-acceptance', async ({ page, loginAs, adminUser }) => {
    test.skip(!generationAcceptance, generationGateReason);
    test.setTimeout(180_000);
    await loginAs(page, adminUser);
    await fillBuilderBrief(page, 'Maya Cakes', 'I bake amazing custom cakes.');
    const response = generationResponse(page);
    await page.getByRole('button', { name: 'Build Store' }).click();
    await expectGeneratedDraft(await response, adminUser.organizationId);
    await expect(page.getByRole('heading', { name: 'Pick your draft' })).toBeVisible();
    await expect(page.getByText('Draft 1', { exact: true })).toBeVisible();
    await expect(page.getByText('Draft 2', { exact: true })).toHaveCount(0);
    await page.getByRole('button', { name: 'Customize Selected Draft' }).click();
    await expect(page.getByText('Mobile Editor')).toBeVisible();
  });
});
