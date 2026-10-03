import { test, expect } from '../../../e2e/fixtures';
import { expectGeneratedDraft, expectGenerationUnavailable, fillBuilderBrief, generationAcceptance, generationGateReason, generationPrerequisite, generationResponse, publishAndReadAnonymous } from '../../../e2e/generation-acceptance';

test('builder does not claim completion without a configured provider', async ({ page, loginAs, adminUser }) => {
  test.skip(generationAcceptance, 'This contract requires an unconfigured generation service.');
  await loginAs(page, adminUser);
  await fillBuilderBrief(page, 'My Awesome Store', 'I run a friendly retail store selling amazing products');
  const response = generationResponse(page);
  await page.getByRole('button', { name: 'Build Store' }).click();
  await expectGenerationUnavailable(await response);
  await expect(page.getByText(generationPrerequisite, { exact: true })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Customize Selected Draft' })).toHaveCount(0);
  await expect(page.getByRole('link', { name: 'Open published website' })).toHaveCount(0);
});

test('builder generates and publishes the owner-reviewed website @provider-acceptance', async ({ page, anonymousPage, loginAs, adminUser }) => {
  test.skip(!generationAcceptance, generationGateReason);
  test.setTimeout(180_000);
  await loginAs(page, adminUser);
  await fillBuilderBrief(page, 'My Awesome Store', 'I run a friendly retail store selling amazing products');
  const response = generationResponse(page);
  await page.getByRole('button', { name: 'Build Store' }).click();
  const draft = await expectGeneratedDraft(await response, adminUser.organizationId);
  await page.getByRole('button', { name: 'Customize Selected Draft' }).click();
  await expect(page.getByText('Mobile Editor')).toBeVisible();
  await publishAndReadAnonymous(page, anonymousPage, draft.pages[0].blocks[0].content.headline);
});
