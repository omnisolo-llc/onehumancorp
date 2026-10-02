import type { QueueOwner } from '@/lib/sync/queueIdentity';
import canonicalize from 'canonicalize';
import { visit } from 'jsonc-parser';

export const SITE_SNAPSHOT_ENCODING = 'jcs-rfc8785-v1' as const;
export type PublicationJson = null | boolean | number | string | PublicationJson[] | { [key: string]: PublicationJson };
export type SiteSnapshot = { domain: null; pages: Array<{ path: string; title: string; seo_metadata: Record<string, PublicationJson>; blocks: Array<{ block_type: string; content: Record<string, PublicationJson>; sort_order: number }> }> };
export type SitePublicationStatus = 'pending' | 'processing' | 'published' | 'failed' | 'revoked';
export type SitePublicationBinding = { owner: QueueOwner; operation_id: string; site_id: string | null; snapshot_sha256: string; snapshot_encoding: typeof SITE_SNAPSHOT_ENCODING };
export type SitePublicationReceipt = {
  schema_version: 1; user_id: string; organization_id: string; publication_id: string; operation_id: string;
  site_id: string; version: number; status: SitePublicationStatus; snapshot_sha256: string; snapshot_encoding: typeof SITE_SNAPSHOT_ENCODING; public_path: string | null;
};
export function isPublicationId(value: unknown): value is string {
  return typeof value === 'string' && /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/.test(value);
}
function record(value: unknown): Record<string, unknown> {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('The publication receipt could not be verified. Check its saved status.');
  return value as Record<string, unknown>;
}
function validText(value: string): boolean {
  return !value.includes('\0') && !Array.from(value).some(character => {
    const point = character.codePointAt(0)!;
    return point >= 0xd800 && point <= 0xdfff;
  });
}
function assertNumericLiteral(raw: string, value: number): void {
  const matched = /^-?(0|[1-9]\d*)(?:\.(\d+))?(?:[eE]([+-]?\d+))?$/.exec(raw);
  if (!matched || !Number.isFinite(value)) throw new Error('The reviewed site contains an invalid numeric literal.');
  const allDigits = matched[1] + (matched[2] ?? '');
  const leadingZeros = /^0*/.exec(allDigits)![0].length;
  const digits = allDigits.slice(leadingZeros);
  if (!digits) return;
  if (value === 0) throw new Error('The reviewed site contains a numeric underflow. Use a reviewed string for high precision.');
  // Compare decimal magnitude before binary64 rounding. This is admission
  // validation only; the pinned JCS implementation owns serialization.
  const position = matched[1].length + Number(matched[3] ?? 0) - leadingZeros;
  const boundary = '9007199254740991';
  const integer = digits.padEnd(16, '0').slice(0, 16);
  if (position > 16 || position === 16 && (integer > boundary || integer === boundary && /[1-9]/.test(digits.slice(16)))) {
    throw new Error('The reviewed site contains a numeric literal beyond the supported magnitude. Use a reviewed string.');
  }
}
/** Preserve raw JSON meaning before JSON.parse can discard duplicate keys. */
function parsePublicationJsonWithLimit(raw: string, containerLimit: number): unknown {
  if (new TextEncoder().encode(raw).length > 2 * 1024 * 1024) throw new Error('The publication JSON exceeds its wire limit.');
  const objects: Array<Set<string>> = []; let depth = 0;
  const enter = () => { if (++depth > containerLimit) throw new Error('The publication JSON is too deeply nested.'); };
  visit(raw, {
    onObjectBegin: () => { enter(); objects.push(new Set()); },
    onObjectProperty: key => {
      const current = objects.at(-1);
      if (!current || !validText(key) || current.has(key)) throw new Error('The publication JSON contains an invalid or duplicate field.');
      current.add(key);
    },
    onObjectEnd: () => { objects.pop(); depth--; },
    onArrayBegin: enter, onArrayEnd: () => { depth--; },
    onLiteralValue: (value, offset, length) => {
      if (typeof value === 'string' && !validText(value)) throw new Error('The publication JSON contains invalid Unicode.');
      if (typeof value === 'number') assertNumericLiteral(raw.slice(offset, offset + length), value);
    },
    onError: () => { throw new Error('The publication body must be strict JSON.'); },
  }, { disallowComments: true, allowTrailingComma: false, allowEmptyContent: false });
  return JSON.parse(raw);
}
export function parsePublicationJson(raw: string): unknown { return parsePublicationJsonWithLimit(raw, 32); }
/** A local marker wraps the wire-valid snapshot in one additional operation container. */
export function parseSavedPublicationJson(raw: string): unknown { return parsePublicationJsonWithLimit(raw, 33); }
export async function readPublicationResponse(response: Response, assertCurrent: () => void = () => {}): Promise<unknown> {
  assertCurrent();
  const maximum = 65_536;
  const declared = response.headers.get('content-length');
  if (response.headers.get('content-type')?.split(';', 1)[0].trim().toLowerCase() !== 'application/json'
    || declared !== null && (!/^(0|[1-9]\d*)$/.test(declared) || Number(declared) > maximum)) {
    void response.body?.cancel().catch(() => undefined);
    throw new Error('The publication response is not a bounded JSON receipt.');
  }
  if (!response.body) throw new Error('The publication response body is missing.');
  const reader = response.body.getReader(); const chunks: Uint8Array[] = []; let size = 0;
  try {
    for (;;) {
      assertCurrent(); const { done, value } = await reader.read(); assertCurrent();
      if (done) break;
      size += value.byteLength;
      if (size > maximum) throw new Error('The publication response exceeds the receipt limit.');
      chunks.push(value);
    }
  } catch (cause) { void reader.cancel().catch(() => undefined); throw cause; }
  finally { reader.releaseLock(); }
  const bytes = new Uint8Array(size); let offset = 0;
  for (const chunk of chunks) { bytes.set(chunk, offset); offset += chunk.byteLength; }
  assertCurrent();
  return parsePublicationJson(new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(bytes));
}
/** Reject lossy inputs before JCS can omit or coerce them. */
function assertJson(value: unknown, seen = new Set<object>(), depth = 0): asserts value is PublicationJson {
  if (value === null || typeof value === 'boolean') return;
  if (typeof value === 'string' && validText(value)) return;
  if (typeof value === 'number' && Number.isFinite(value) && Math.abs(value) <= Number.MAX_SAFE_INTEGER) return;
  // Reserve one container for the POST envelope: snapshot containers occupy at most 31 levels.
  if (value && typeof value === 'object' && depth >= 31) throw new Error('The reviewed site is too deeply nested.');
  if (!value || typeof value !== 'object' || seen.has(value)) throw new Error('The reviewed site contains invalid JSON content.');
  if (!Array.isArray(value) && ![Object.prototype, null].includes(Object.getPrototypeOf(value))) throw new Error('The reviewed site must contain plain JSON content.');
  seen.add(value);
  const descriptors = Object.getOwnPropertyDescriptors(value);
  for (const key of Reflect.ownKeys(descriptors)) {
    if (typeof key !== 'string' || !validText(key) || !('value' in descriptors[key])) throw new Error('The reviewed site contains invalid JSON fields.');
  }
  if (Array.isArray(value)) {
    for (let index = 0; index < value.length; index++) {
      if (!Object.hasOwn(value, index)) throw new Error('The reviewed site contains an incomplete list.');
      assertJson(value[index], seen, depth + 1);
    }
    if (Object.keys(value).some(key => !/^(0|[1-9]\d*)$/.test(key) || Number(key) >= value.length)) throw new Error('The reviewed site contains unexpected list fields.');
  } else {
    for (const child of Object.values(value)) assertJson(child, seen, depth + 1);
  }
  seen.delete(value);
}
function exact(value: Record<string, unknown>, fields: string[]) {
  if (Object.keys(value).length !== fields.length || Object.keys(value).some(key => !fields.includes(key))) throw new Error('The reviewed site contains missing or unexpected fields.');
}
function freeze<T>(value: T): T {
  if (value && typeof value === 'object') { for (const child of Object.values(value)) freeze(child); Object.freeze(value); }
  return value;
}
export async function prepareSiteSnapshot(value: unknown): Promise<{ snapshot: SiteSnapshot; canonical: string; snapshot_encoding: typeof SITE_SNAPSHOT_ENCODING; snapshot_sha256: string }> {
  assertJson(value);
  const data = record(value); exact(data, ['domain', 'pages']);
  if (data.domain !== null || !Array.isArray(data.pages) || !data.pages.length || data.pages.length > 50) throw new Error('Review between 1 and 50 pages for application-hosted publication.');
  const paths = new Set<string>();
  for (const raw of data.pages) {
    const page = record(raw); exact(page, ['path', 'title', 'seo_metadata', 'blocks']);
    if (typeof page.path !== 'string' || !page.path.startsWith('/') || page.path.startsWith('//')
      || new TextEncoder().encode(page.path).length > 512 || /[%?#\\]/.test(page.path)
      || Array.from(page.path).some(character => { const code = character.charCodeAt(0); return code <= 31 || code >= 127 && code <= 159; })
      || (page.path !== '/' && page.path.slice(1).split('/').some(part => !part || part === '.' || part === '..')) || paths.has(page.path)) throw new Error('Review unique local document paths.');
    paths.add(page.path);
    if (typeof page.title !== 'string' || !page.title.trim() || Array.from(page.title).length > 200
      || !Array.isArray(page.blocks) || page.blocks.length > 100) throw new Error('Review each page title and its bounded block list.');
    record(page.seo_metadata);
    const orders = new Set<number>();
    for (const rawBlock of page.blocks) {
      const block = record(rawBlock); exact(block, ['block_type', 'content', 'sort_order']);
      if (typeof block.block_type !== 'string' || !block.block_type.trim() || new TextEncoder().encode(block.block_type).length > 64
        || !Number.isInteger(block.sort_order) || Number(block.sort_order) < -2147483648 || Number(block.sort_order) > 2147483647
        || orders.has(Number(block.sort_order))) throw new Error('Review each block type and its unique position.');
      orders.add(Number(block.sort_order)); record(block.content);
    }
  }
  if (!paths.has('/')) throw new Error('The reviewed site needs a root page.');
  const canonical = canonicalize(data);
  if (typeof canonical !== 'string') throw new Error('The reviewed site could not be encoded.');
  const bytes = new TextEncoder().encode(canonical);
  if (bytes.length > 1024 * 1024) throw new Error('The reviewed site exceeds 1 MiB.');
  const digest = Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256', bytes)), byte => byte.toString(16).padStart(2, '0')).join('');
  return { snapshot: freeze(JSON.parse(canonical) as SiteSnapshot), canonical, snapshot_encoding: SITE_SNAPSHOT_ENCODING, snapshot_sha256: digest };
}
/** HTTP acceptance and a publication receipt are different facts. */
export function readSitePublicationReceipt(
  httpStatus: number, value: unknown, expected: SitePublicationBinding,
  action: 'submit' | 'read' | 'revoke', previous?: SitePublicationReceipt,
): SitePublicationReceipt {
  const data = record(value);
  const allowed = ['schema_version', 'user_id', 'organization_id', 'publication_id', 'operation_id', 'site_id', 'version', 'status', 'snapshot_sha256', 'snapshot_encoding', 'public_path'];
  const pending = data.status === 'pending' || data.status === 'processing';
  const terminal = data.status === 'published' || data.status === 'failed' || data.status === 'revoked';
  const previousTerminal = previous && ['published', 'failed', 'revoked'].includes(previous.status);
  if (Object.keys(data).some(key => !allowed.includes(key))
    || data.schema_version !== 1 || !expected.owner.userId || !expected.owner.tenantId
    || data.user_id !== expected.owner.userId || data.organization_id !== expected.owner.tenantId
    || !isPublicationId(data.operation_id) || data.operation_id !== expected.operation_id
    || !isPublicationId(data.publication_id) || !isPublicationId(data.site_id)
    || (expected.site_id !== null && data.site_id !== expected.site_id)
    || !Number.isSafeInteger(data.version) || Number(data.version) < 1
    || (!pending && !terminal) || typeof data.snapshot_sha256 !== 'string'
    || data.snapshot_encoding !== SITE_SNAPSHOT_ENCODING || expected.snapshot_encoding !== SITE_SNAPSHOT_ENCODING
    || !/^[0-9a-f]{64}$/.test(data.snapshot_sha256) || data.snapshot_sha256 !== expected.snapshot_sha256
    || (data.public_path !== null && (data.status !== 'published' || data.public_path !== '/api/v1/public/sites/' + data.site_id))
    || (action === 'submit' ? (pending ? httpStatus !== 202 : httpStatus !== 200) : httpStatus !== 200)
    || (action === 'revoke' && data.status !== 'revoked')
    || (previousTerminal && data.status !== previous.status && data.status !== 'revoked')
    || (previous?.status === 'revoked' && data.status !== 'revoked')
    || (previous && (data.publication_id !== previous.publication_id || data.site_id !== previous.site_id || data.version !== previous.version))) {
    throw new Error('The publication receipt could not be verified. Check its saved status.');
  }
  return {
    schema_version: 1, user_id: data.user_id as string, organization_id: data.organization_id as string,
    publication_id: data.publication_id, operation_id: data.operation_id, site_id: data.site_id,
    version: Number(data.version), status: data.status as SitePublicationStatus,
    snapshot_sha256: data.snapshot_sha256, snapshot_encoding: SITE_SNAPSHOT_ENCODING, public_path: data.public_path as string | null,
  };
}
