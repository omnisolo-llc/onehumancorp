import type { OnboardingResult } from '@/lib/builder-types';
export type ReviewedProduct = { product_id: string; name: string; price: string; description: string; variants: { variant_id: string; name: string; price_modifier: string }[] };
export type Preparation = {
  preparation_id: string;
  status: 'prepared' | 'launched';
  organization_id: string;
  user_id: string;
  primary_product_id: string;
  reviewed_request: Record<string, unknown>;
  catalog: ReviewedProduct[];
};
export type PreparedResult = OnboardingResult & { success: true; preparation_id: string; user_id: string; preparation: Preparation; status: 'prepared' | 'launched'; organization_id: string };
function record(value: unknown): Record<string, unknown> {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('Setup response is incomplete');
  return value as Record<string, unknown>;
}
function text(value: unknown): value is string { return typeof value === 'string' && value.trim().length > 0; }
export function readPreparation(value: unknown): Preparation {
  const data = record(value);
  if (!text(data.preparation_id) || !text(data.organization_id) || !text(data.user_id) || !text(data.primary_product_id) || !['prepared', 'launched'].includes(String(data.status)) || !Array.isArray(data.catalog)) throw new Error('Setup preparation could not be verified');
  record(data.reviewed_request);
  const ids = new Set<string>();
  for (const value of data.catalog) {
    const product = record(value);
    if (!text(product.product_id) || ids.has(product.product_id) || !text(product.name) || !text(product.price) || typeof product.description !== 'string' || !Array.isArray(product.variants)) throw new Error('Setup catalog receipt is incomplete');
    ids.add(product.product_id);
    for (const item of product.variants) {
      const variant = record(item);
      if (!text(variant.variant_id) || !text(variant.name) || typeof variant.price_modifier !== 'string') throw new Error('Setup variant receipt is incomplete');
    }
  }
  if (!ids.has(data.primary_product_id)) throw new Error('Setup primary product was not acknowledged');
  return data as Preparation;
}
export function resultForPreparation(preparation: Preparation): PreparedResult {
  return { success: true, preparation_id: preparation.preparation_id, status: preparation.status, organization_id: preparation.organization_id, user_id: preparation.user_id, product_ids: preparation.catalog.map(product => product.product_id), preparation };
}
export function readPreparedResult(value: unknown, prior?: Preparation): PreparedResult {
  const data = record(value);
  if (data.success !== true || data.error != null) throw new Error(typeof data.error === 'string' ? data.error : 'Setup preparation was not acknowledged');
  const preparation = readPreparation(data.preparation);
  if (prior && (preparation.organization_id !== prior.organization_id || preparation.user_id !== prior.user_id || preparation.primary_product_id !== prior.primary_product_id)) throw new Error('Setup revision identity does not match the committed preparation');
  if (data.preparation_id !== preparation.preparation_id || data.organization_id !== preparation.organization_id || data.user_id !== preparation.user_id || data.status !== preparation.status) throw new Error('Setup preparation acknowledgement does not match');
  return { ...resultForPreparation(preparation), ...(typeof data.message === 'string' ? { message: data.message } : {}), ...(typeof data.business_name === 'string' ? { business_name: data.business_name } : {}), ...(typeof data.website_url === 'string' ? { website_url: data.website_url } : {}), ...(typeof data.storefront_url === 'string' ? { storefront_url: data.storefront_url } : {}) };
}
export function readLaunchResult(value: unknown, expected: Preparation): Preparation {
  const data = record(value);
  if (data.success !== true || data.status !== 'launched' || data.preparation_id !== expected.preparation_id || data.organization_id !== expected.organization_id || data.user_id !== expected.user_id || data.error != null) throw new Error('Setup completion could not be verified');
  return { ...expected, status: 'launched' };
}
export function observedWebsite(result: OnboardingResult | null): string | null {
  const candidate = result?.website_url ?? result?.storefront_url ?? result?.url ?? result?.site?.url;
  if (typeof candidate !== 'string' || !candidate.trim()) return null;
  try {
    const url = new URL(candidate, window.location.origin);
    if (url.username || url.password || (url.protocol !== 'https:' && !(url.protocol === 'http:' && url.origin === window.location.origin))) return null;
    return url.href;
  } catch { return null; }
}
export function canonicalRequest(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map(canonicalRequest).join(',')}]`;
  if (value && typeof value === 'object') return `{${Object.entries(value).filter(([, nested]) => nested !== undefined).sort(([a], [b]) => a.localeCompare(b)).map(([key, nested]) => `${JSON.stringify(key)}:${canonicalRequest(nested)}`).join(',')}}`;
  return JSON.stringify(value);
}

export async function readDraftAcknowledgement(response: Response): Promise<void> {
  if (!response.ok || (response.status !== undefined && response.status !== 200 && response.status !== 204)) throw new Error('Draft save was not acknowledged');
  let data: unknown;
  if (typeof response.text === 'function') {
    const body = await response.text();
    if (!body.trim()) return;
    data = JSON.parse(body);
  } else { data = await response.json(); }
  if (data && typeof data === 'object') {
    const result = data as Record<string, unknown>;
    if (result.success === false || result.error != null) throw new Error(typeof result.error === 'string' ? result.error : 'Draft save was rejected');
  }
}
