/** Validate only; never normalize the reviewed token through Date, which loses
 * PostgreSQL microseconds. The backend compares the original RFC3339 instant. */
export function isQuoteVersion(value: unknown): value is string {
  return typeof value === 'string' && value.length <= 64
    && /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d{1,9})?(?:Z|[+-]\d{2}:\d{2})$/.test(value)
    && Number.isFinite(Date.parse(value));
}
