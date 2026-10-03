import { expect, test } from './fixtures';
import { expectGeneratedDraft, expectGenerationUnavailable, generationAcceptance, generationGateReason, generationPrerequisite, generationResponse, publicationWrites, publishAndReadAnonymous } from './generation-acceptance';

const brief = 'Luna Loaf is a local bakery selling custom cakes, weekend dessert boxes, and warm pickup experiences for families.';
test.describe('Brand Studio workflow', () => {
  test('unconfigured text generation is honest about provider, URL and media prerequisites', async ({ page, adminUser, loginAs }) => {
    test.skip(generationAcceptance, 'This contract requires an unconfigured generation service.');
    await loginAs(page, adminUser);
    await page.goto('/brand-studio');
    const writes = publicationWrites(page);
    await page.getByLabel('Business', { exact: true }).fill(brief);
    await expect(page.getByLabel('Website URL')).toBeDisabled();
    await expect(page.getByLabel('Product URL')).toBeDisabled();
    await expect(page.getByText('Only supplied text is used. Website fetching and uploaded-media generation are unavailable in this flow.')).toBeVisible();
    await page.getByLabel('Campaign', { exact: true }).fill('launch the summer dessert box');
    const response = generationResponse(page, '/api/v1/builder/brand_toolbox/generate');
    await page.getByRole('button', { name: 'Generate Toolbox' }).click();
    await expectGenerationUnavailable(await response);
    await expect(page.getByRole('alert').filter({ hasText: generationPrerequisite })).toHaveText(generationPrerequisite);
    await expect(page.getByLabel('Business', { exact: true })).toHaveValue(brief);
    await expect(page.getByRole('button', { name: 'Review public version' })).toHaveCount(0);
    await expect(page.getByRole('link', { name: 'Open published website' })).toHaveCount(0);
    expect(writes).toEqual([]);
  });

  test('generates a real text toolbox and publishes its reviewed website @provider-acceptance', async ({ page, anonymousPage, adminUser, loginAs }) => {
    test.skip(!generationAcceptance, generationGateReason);
    test.setTimeout(180_000);
    await loginAs(page, adminUser);
    await page.goto('/brand-studio');
    await page.getByLabel('Business', { exact: true }).fill(brief);
    await page.getByLabel('Campaign', { exact: true }).fill('launch the summer dessert box');
    const response = generationResponse(page, '/api/v1/builder/brand_toolbox/generate');
    await page.getByRole('button', { name: 'Generate Toolbox' }).click();
    const toolbox = await expectGeneratedDraft(await response, adminUser.organizationId);
    expect(toolbox.id).toMatch(/^[a-f0-9-]{36}$/);
    const persisted = await page.request.get(`/api/v1/builder/brand_toolbox/${toolbox.id}`);
    expect(persisted.status()).toBe(200);
    expect(await persisted.json()).toMatchObject({ id: toolbox.id, brand_dna: toolbox.brand_dna, generation: toolbox.generation });
    await expect(page.getByRole('heading', { name: toolbox.brand_dna.name, exact: true })).toBeVisible();
    for (const heading of ['Brand Book', 'Logo Concepts', 'Catalog Suggestions', 'Campaign Ideas', 'Social Calendar', 'Creative Assets', 'Photoshoot Concepts', 'Website Draft']) await expect(page.getByRole('heading', { name: heading, exact: true })).toBeVisible();
    expect(toolbox.logo_concepts).toEqual([]);
    expect(toolbox.photoshoot.shots).toEqual([]);
    await publishAndReadAnonymous(page, anonymousPage, toolbox.store_profile.pages[0].blocks[0].content.headline);
  });
});
