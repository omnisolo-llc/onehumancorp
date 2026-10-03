import { expect, type Page, type Response } from '@playwright/test';

// These switches select separately owned acceptance gates. They do not provision
// credentials, grant spending authority, or substitute a provider/runtime.
export const generationAcceptance = process.env.OMNISOLO_E2E_GENERATION_ACCEPTANCE === '1';
export const runtimeAcceptance = process.env.OMNISOLO_E2E_RUNTIME_ACCEPTANCE === '1';
export const generationPrerequisite = 'Configure the builder operator tenant and an authorized text-generation provider before generating a draft.';
export const generationGateReason = 'Release owner must supply an authorized real provider, tenant and usage budget. See docs/development/generation-acceptance.md. This journey is unverified when skipped.';
export const runtimeGateReason = 'Release owner must supply a tenant-bound repository runtime. See docs/development/generation-acceptance.md. RepoMap completion is unverified when skipped.';

export function generationResponse(page: Page, endpoint = '/api/v1/builder/generate') {
  return page.waitForResponse(response => new URL(response.url()).pathname === endpoint && response.request().method() === 'POST', { timeout: 120_000 });
}
export async function expectGenerationUnavailable(response: Response) {
  expect(response.status(), 'Unconfigured tests must not accept arbitrary errors or fabricated generation').toBe(503);
  const body = await response.json();
  expect(body.code).toBe('generation_unavailable');
  expect(body.error).toContain('authorized text-generation provider');
  expect(body.pages).toBeUndefined();
  expect(body.store_profile).toBeUndefined();
  expect(body.id).toBeUndefined();
}
export async function expectGeneratedDraft(response: Response, tenantId: string) {
  expect(response.status(), 'The real configured-provider journey must actually generate a draft').toBe(200);
  const body = await response.json();
  expect(body.error).toBeUndefined();
  expect(body.generation).toMatchObject({ tenant_id: tenantId, kind: 'model_draft', input_source: 'supplied_text', website_fetched: false, assets_read: false, business_facts_verified: false });
  expect(body.generation.provider).toEqual(expect.any(String));
  expect(body.generation.provider.trim()).not.toBe('');
  expect(body.generation.model.trim()).not.toBe('');
  const profile = body.store_profile ?? body;
  expect(profile.pages.length).toBeGreaterThan(0);
  expect(profile.pages[0].blocks.length).toBeGreaterThan(0);
  return body;
}
export async function fillBuilderBrief(page: Page, name: string, description: string) {
  await page.goto('/builder');
  await page.getByRole('button', { name: 'Selling Products' }).click();
  await page.getByPlaceholder('e.g. Acme Corp').fill(name);
  await page.getByPlaceholder('e.g. Retail, Consulting, Tech').fill('Bakery');
  await expect(page.getByText('✓ Looks good')).toBeVisible();
  await expect(page.getByText('✓ Sounds great')).toBeVisible();
  await page.getByRole('button', { name: 'Next: Choose Vibe' }).click();
  await page.getByRole('button', { name: /Friendly/ }).click();
  await page.getByRole('button', { name: 'Next: Details' }).click();
  await page.getByPlaceholder(/mobile dog grooming service/i).fill(description);
  await expect(page.getByText('Draft saved on this device')).toBeVisible();
}
export function publicationWrites(page: Page) {
  const writes: string[] = [];
  page.on('request', request => {
    if (request.method() === 'POST' && /^\/api\/v1\/builder\/(publications|sites|publish)/.test(new URL(request.url()).pathname)) writes.push(request.url());
  });
  return writes;
}
export async function publishAndReadAnonymous(page: Page, anonymousPage: Page, expectedText: string) {
  await page.getByRole('button', { name: 'Review public version' }).click();
  await expect(page.getByLabel('Public website snapshot')).toContainText(expectedText);
  await page.getByRole('button', { name: 'Publish reviewed version' }).click();
  const link = page.getByRole('link', { name: 'Open published website' });
  await expect.poll(async () => {
    if (await link.isVisible()) return true;
    const check = page.getByRole('button', { name: 'Check publication status' });
    if (await check.isVisible() && await check.isEnabled()) await check.click();
    return link.isVisible();
  }, { timeout: 30_000, message: 'A real publication receipt and public path are required' }).toBe(true);
  const path = await link.getAttribute('href');
  expect(path).toMatch(/^\/api\/v1\/public\/sites\/[a-f0-9-]{36}$/);
  const published = await anonymousPage.goto(new URL(path!, page.url()).href);
  expect(published?.status()).toBe(200);
  await expect(anonymousPage.locator('body')).toContainText(expectedText);
}
