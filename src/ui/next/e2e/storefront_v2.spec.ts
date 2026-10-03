import { test, expect } from '../../../e2e/fixtures';
import { expectGeneratedDraft, expectGenerationUnavailable, fillBuilderBrief, generationAcceptance, generationGateReason, generationPrerequisite, generationResponse, publishAndReadAnonymous } from '../../../e2e/generation-acceptance';

test.use({ viewport: { width: 390, height: 844 }, isMobile: true, hasTouch: true });

test('mobile storefront retains the brief when generation is unconfigured', async ({ page, loginAs, adminUser }) => {
  test.skip(generationAcceptance, 'This contract requires an unconfigured generation service.');
  await loginAs(page, adminUser);
  await fillBuilderBrief(page, 'Maya Cakes', 'I bake custom cakes for weddings and parties.');
  const response = generationResponse(page);
  await page.getByRole('button', { name: 'Build Store' }).click();
  await expectGenerationUnavailable(await response);
  await expect(page.getByText(generationPrerequisite, { exact: true })).toBeVisible();
  await expect(page.getByPlaceholder(/mobile dog grooming service/i)).toHaveValue('I bake custom cakes for weddings and parties.');
  await expect(page.getByText('Pick your draft')).toHaveCount(0);
});

test('Maya reviews, edits and publishes actual generated copy on mobile @provider-acceptance', async ({ page, anonymousPage, loginAs, adminUser }) => {
  test.skip(!generationAcceptance, generationGateReason);
  test.setTimeout(180_000);
  await loginAs(page, adminUser);
  await fillBuilderBrief(page, 'Maya Cakes', 'I bake custom cakes for weddings and parties.');
  const response = generationResponse(page);
  await page.getByRole('button', { name: 'Build Store' }).click();
  const draft = await expectGeneratedDraft(await response, adminUser.organizationId);
  await page.getByRole('button', { name: 'Customize Selected Draft' }).click();
  await expect(page.getByText('Mobile Editor')).toBeVisible();
  const headline = draft.pages[0].blocks[0].content.headline;
  await page.getByText(headline, { exact: true }).click();
  await expect(page.getByText('Edit Hero Block')).toBeVisible();
  await page.locator('input[type="text"]').filter({ visible: true }).fill('Owner-reviewed custom cake copy');
  await page.getByRole('button', { name: 'Save Changes' }).click();
  await publishAndReadAnonymous(page, anonymousPage, 'Owner-reviewed custom cake copy');
});
