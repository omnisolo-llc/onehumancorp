import { test, expect } from './fixtures';
import { expectGeneratedDraft, expectGenerationUnavailable, generationAcceptance, generationGateReason, generationPrerequisite, generationResponse, publicationWrites } from './generation-acceptance';

test.describe('Agentic Storefront Editor', () => {
  test('unconfigured generation explains the prerequisite and preserves the owner brief', async ({ page, adminUser, loginAs }) => {
    test.skip(generationAcceptance, 'This contract requires an unconfigured generation service.');
    await loginAs(page, adminUser);
    await page.goto('/storefront-builder');
    const writes = publicationWrites(page);
    await page.getByPlaceholder(/mobile dog grooming service/i).fill('Maya the home baker, I bake custom vegan cakes.');
    const response = generationResponse(page);
    await page.getByRole('button', { name: 'Build My Storefront' }).click();
    await expectGenerationUnavailable(await response);
    await expect(page.getByRole('alert')).toHaveText(generationPrerequisite);
    await expect(page.getByText('Preview Mode')).toHaveCount(0);
    await page.reload();
    await expect(page.getByPlaceholder(/mobile dog grooming service/i)).toHaveValue('Maya the home baker, I bake custom vegan cakes.');
    await expect(page.getByText('Preview Mode')).toHaveCount(0);
    expect(writes).toEqual([]);
  });

  test('Maya generates and edits actual storefront copy @provider-acceptance', async ({ page, adminUser, loginAs }) => {
    test.skip(!generationAcceptance, generationGateReason);
    test.setTimeout(180_000);
    await loginAs(page, adminUser);
    await page.goto('/storefront-builder');
    await page.getByPlaceholder(/mobile dog grooming service/i).fill('Maya the home baker, I bake custom vegan cakes.');
    const response = generationResponse(page);
    await page.getByRole('button', { name: 'Build My Storefront' }).click();
    const draft = await expectGeneratedDraft(await response, adminUser.organizationId);
    await expect(page.getByText('Preview Mode')).toBeVisible();
    await expect(page.locator('body')).toContainText(draft.pages[0].blocks[0].content.headline);
    await page.getByRole('button', { name: /Ask Agent to Edit/ }).click();
    await page.getByPlaceholder(/Add a new product/i).fill('Update the headline to emphasize vegan celebration cakes.');
    const edited = generationResponse(page);
    await page.getByRole('button', { name: 'Send storefront edit' }).click();
    const result = await expectGeneratedDraft(await edited, adminUser.organizationId);
    await expect(page.getByText('Preview Mode')).toBeVisible();
    await expect(page.locator('body')).toContainText(result.pages[0].blocks[0].content.headline);
  });
});
