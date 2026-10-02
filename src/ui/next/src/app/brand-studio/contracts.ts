import DOMPurify from 'dompurify';
import { isPublicationId, prepareSiteSnapshot, type SiteSnapshot } from '../builder/publicationContracts';

export type BrandToolbox = {
  id?: string;
  generation: { kind: 'model_draft'; provider: string; model: string; input_source: 'supplied_text'; website_fetched: false; assets_read: false; business_facts_verified: false; generated_at: string };
  brand_dna: {
    name: string;
    business_type: string;
    positioning: string;
    audience: string;
    tone_of_voice: string[];
    colors: string[];
    fonts: string[];
    image_style: string[];
  };
  logo_concepts: { title: string; svg: string; usage_notes: string[] }[];
  brand_book: { title: string; guidance: string[] }[];
  catalog: {
    name: string;
    price: string;
    description: string;
    photo_prompt: string;
    seo_title: string;
  }[];
  campaign_ideas: { title: string; goal: string; channels: string[]; hook: string }[];
  social_calendar: {
    day: string;
    channel: string;
    caption: string;
    visual_prompt: string;
    call_to_action: string;
  }[];
  assets: {
    asset_type: string;
    channel: string;
    title: string;
    copy: string;
    visual_prompt: string;
    editable_fields: string[];
  }[];
  photoshoot: {
    product_source: string;
    templates: string[];
    prompts: string[];
    shots: { title: string; format: string; prompt: string; usage: string; mockup_svg: string }[];
    refinement_controls: string[];
  };
  store_profile?: {
    pages: SiteSnapshot['pages'];
  };
  website_draft?: {
    pages: SiteSnapshot['pages'];
  };
  export_formats: string[];
};

function record(value: unknown): Record<string, unknown> {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('The returned brand draft is incomplete.');
  return value as Record<string, unknown>;
}
function text(value: unknown): void {
  if (typeof value !== 'string' || value.length > 200_000) throw new Error('The returned brand text is invalid.');
}
function strings(value: unknown): void {
  if (!Array.isArray(value) || value.length > 100) throw new Error('The returned brand list is invalid.');
  value.forEach(text);
}
function rows(value: unknown, textFields: string[], listFields: string[] = []): void {
  if (!Array.isArray(value) || value.length > 100) throw new Error('The returned brand sections are invalid.');
  for (const item of value) {
    const row = record(item);
    textFields.forEach(field => text(row[field]));
    listFields.forEach(field => strings(row[field]));
  }
}

/** Select page fields only; toolbox identity, source URLs and actor fields stay private. */
export function brandPublicationSnapshot(value: BrandToolbox): SiteSnapshot {
  const source = value.website_draft ?? value.store_profile;
  if (!source || !Array.isArray(source.pages)) throw new Error('No website draft was returned.');
  return { domain: null, pages: source.pages.map(page => {
    if (!page || typeof page.path !== 'string' || typeof page.title !== 'string' || !Array.isArray(page.blocks)) throw new Error('The returned website draft is incomplete.');
    record(page.seo_metadata);
    return {
      path: page.path, title: page.title, seo_metadata: page.seo_metadata,
      blocks: page.blocks.map(block => {
        if (!block || typeof block.block_type !== 'string' || !Number.isInteger(block.sort_order)) throw new Error('The returned website block is incomplete.');
        record(block.content);
        return { block_type: block.block_type, content: block.content, sort_order: block.sort_order };
      }),
    };
  }) };
}

export async function readBrandToolbox(value: unknown): Promise<BrandToolbox> {
  const data = record(value);
  if (data.success !== undefined && data.success !== true || data.error != null || !isPublicationId(data.id)
    || data.status !== undefined && !['ready', 'generated'].includes(String(data.status))) throw new Error('The generated brand draft could not be confirmed.');
  const generation = record(data.generation);
  if (generation.kind !== 'model_draft' || generation.input_source !== 'supplied_text' || generation.website_fetched !== false || generation.assets_read !== false || generation.business_facts_verified !== false
    || typeof generation.provider !== 'string' || !generation.provider.trim() || typeof generation.model !== 'string' || !generation.model.trim()
    || typeof generation.generated_at !== 'string' || !/^\d{4}-\d{2}-\d{2}T/.test(generation.generated_at) || !Number.isFinite(Date.parse(generation.generated_at))) throw new Error('The returned brand generation provenance could not be confirmed.');
  const brand = record(data.brand_dna);
  ['name', 'business_type', 'positioning', 'audience'].forEach(field => text(brand[field]));
  if (!(brand.name as string).trim()) throw new Error('The returned brand has no reviewed name.');
  ['tone_of_voice', 'colors', 'fonts', 'image_style'].forEach(field => strings(brand[field]));
  rows(data.logo_concepts, ['title', 'svg'], ['usage_notes']);
  rows(data.brand_book, ['title'], ['guidance']);
  rows(data.catalog, ['name', 'price', 'description', 'photo_prompt', 'seo_title']);
  rows(data.campaign_ideas, ['title', 'goal', 'hook'], ['channels']);
  rows(data.social_calendar, ['day', 'channel', 'caption', 'visual_prompt', 'call_to_action']);
  rows(data.assets, ['asset_type', 'channel', 'title', 'copy', 'visual_prompt'], ['editable_fields']);
  const photoshoot = record(data.photoshoot);
  text(photoshoot.product_source);
  ['templates', 'prompts', 'refinement_controls'].forEach(field => strings(photoshoot[field]));
  rows(photoshoot.shots, ['title', 'format', 'prompt', 'usage', 'mockup_svg']);
  strings(data.export_formats);
  const candidate = data as unknown as BrandToolbox;
  await prepareSiteSnapshot(brandPublicationSnapshot(candidate));
  return structuredClone(candidate);
}

/** SVG stays an isolated image, with active/external content removed before encoding. */
export function safeBrandSvg(svg: string): string | null {
  if (typeof svg !== 'string' || svg.length > 200_000 || svg.includes('\0')) return null;
  if (typeof DOMPurify.sanitize !== 'function' || typeof DOMParser === 'undefined') return null;
  const clean = DOMPurify.sanitize(svg, {
    USE_PROFILES: { svg: true },
    FORBID_TAGS: ['script', 'foreignObject', 'image', 'use', 'a', 'animate', 'animateMotion', 'animateTransform', 'set', 'style'],
    FORBID_ATTR: ['href', 'xlink:href', 'style'],
  });
  const document = new DOMParser().parseFromString(clean, 'image/svg+xml');
  if (document.querySelector('parsererror') || document.documentElement.localName !== 'svg' || document.documentElement.namespaceURI !== 'http://www.w3.org/2000/svg' || document.querySelectorAll('*').length > 2000) return null;
  for (const element of document.querySelectorAll('*')) {
    for (const attribute of Array.from(element.attributes)) {
      const name = attribute.name.toLowerCase();
      if (name.startsWith('on') || name === 'href' || name.endsWith(':href') || name === 'xml:base' || name === 'style' || attribute.value.includes('\\')
        || /url\s*\(/i.test(attribute.value) && !/^url\(\s*['"]?#[A-Za-z_][A-Za-z0-9_.:-]*['"]?\s*\)$/i.test(attribute.value)) element.removeAttributeNode(attribute);
    }
  }
  if (!document.querySelector('path, rect, circle, ellipse, line, polyline, polygon, text')) return null;
  return 'data:image/svg+xml;charset=utf-8,' + encodeURIComponent(document.documentElement.outerHTML);
}
