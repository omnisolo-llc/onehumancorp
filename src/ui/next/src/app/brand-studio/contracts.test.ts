import { expect, it, vi } from 'vitest';
import { brandPublicationSnapshot, readBrandToolbox, safeBrandSvg } from './contracts';
vi.unmock('dompurify');

function validDraft() {
  return {
    id: '10000000-0000-4000-8000-000000000001',
    generation: { kind: 'model_draft', provider: 'test-provider', model: 'test-model', input_source: 'supplied_text', website_fetched: false, assets_read: false, business_facts_verified: false, generated_at: '2026-10-02T19:45:00Z' },
    brand_dna: { name: 'Owner draft', business_type: 'Service', positioning: 'Proposal', audience: 'Proposed audience', tone_of_voice: [], colors: [], fonts: [], image_style: [] },
    logo_concepts: [], brand_book: [], catalog: [], campaign_ideas: [], social_calendar: [], assets: [],
    photoshoot: { product_source: 'Supplied text', templates: [], prompts: [], shots: [], refinement_controls: [] }, export_formats: [],
    store_profile: { domain: 'unverified.invalid', pages: [{ path: '/', title: 'Owner draft', seo_metadata: {}, blocks: [{ block_type: 'HeroBlock', content: { headline: 'Owner headline' }, sort_order: 0 }] }] },
  };
}
it('selects actual page fields and preserves the original private response', async () => {
  const original = validDraft(); const before = JSON.stringify(original);
  const parsed = await readBrandToolbox({ ...original, private_actor: 'not-public' });
  expect(brandPublicationSnapshot(parsed)).toEqual({ domain: null, pages: original.store_profile.pages });
  expect(JSON.stringify(brandPublicationSnapshot(parsed))).not.toMatch(/not-public|unverified.invalid|generation/);
  expect(JSON.stringify(original)).toBe(before);
});
it.each(['missing', 'wrong-source', 'false-verification', 'missing-model', 'bad-time'])('rejects unconfirmed generation provenance: %s', async mode => {
  const data = validDraft(); const source: Record<string, unknown> = data;
  if (mode === 'missing') delete source.generation;
  if (mode === 'wrong-source') data.generation.input_source = 'unfetched_website';
  if (mode === 'false-verification') data.generation.business_facts_verified = true;
  if (mode === 'missing-model') data.generation.model = '';
  if (mode === 'bad-time') data.generation.generated_at = 'not-a-date';
  await expect(readBrandToolbox(source)).rejects.toThrow();
});
it.each(['bad-id', 'missing-blocks', 'duplicate-page', 'invalid-text'])('rejects malformed brand data: %s', async mode => {
  const data = validDraft();
  if (mode === 'bad-id') data.id = 'invented';
  if (mode === 'missing-blocks') (data.store_profile.pages[0] as unknown as Record<string, unknown>).blocks = null;
  if (mode === 'duplicate-page') data.store_profile.pages.push(data.store_profile.pages[0]);
  if (mode === 'invalid-text') (data.brand_dna as unknown as Record<string, unknown>).name = { unexpected: 'object' };
  await expect(readBrandToolbox(data)).rejects.toThrow();
});
it.each([{ success: 'false' }, { success: 0 }, { status: 'failed' }, { status: 'pending' }])('rejects contradictory acknowledgement fields %j', async fields => {
  await expect(readBrandToolbox({ ...validDraft(), ...fields })).rejects.toThrow(/could not be confirmed/);
});
it('drops external SVG paint URLs and active content using the actual sanitizer', () => {
  const raw = '<svg xmlns="http://www.w3.org/2000/svg" xml:base="https://example.invalid"><rect width="10" height="10" fill="url(https://example.invalid/paint)" onclick="alert(1)"/><script>alert(1)</script></svg>';
  const image = safeBrandSvg(raw);
  expect(image).not.toBeNull(); expect(decodeURIComponent(image!)).not.toMatch(/example.invalid|onclick|<script/i);
  expect(raw).toContain('example.invalid');
});
it('reports non-SVG, oversize or non-renderable assets as unavailable', () => {
  expect(safeBrandSvg('<div>Not an image</div>')).toBeNull();
  expect(safeBrandSvg('<svg xmlns="http://www.w3.org/2000/svg"><script>alert(1)</script></svg>')).toBeNull();
  expect(safeBrandSvg('x'.repeat(200_001))).toBeNull();
  expect(safeBrandSvg('<svg><text>Missing namespace</text></svg>')).toBeNull();
});
it('drops escaped external paint URLs rather than depending on CSS interpretation', () => {
  const raw = String.raw`<svg xmlns="http://www.w3.org/2000/svg"><rect width="10" height="10" fill="u\72l(https://example.invalid/paint)"/></svg>`;
  const image = safeBrandSvg(raw); expect(image).not.toBeNull();
  expect(decodeURIComponent(image!)).not.toContain('example.invalid');
});
