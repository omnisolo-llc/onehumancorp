export const OMNISOLO_BRAND = "OmniSolo" as const;
export const OMNISOLO_CLOUD_ORIGIN = "https://cloud.omnisolo.co" as const;

/**
 * Build a product-owned URL used in generated customer-facing embeds.
 *
 * Keeping this boundary strict prevents a stale legacy origin from silently
 * returning to generated snippets when a feature is copied or extended.
 */
export function cloudUrl(path: string): string {
  if (!path.startsWith("/") || path.startsWith("//") || /^[a-z][a-z\d+.-]*:/i.test(path)) {
    throw new Error("cloud paths must be relative to the OmniSolo cloud origin");
  }
  return new URL(path, OMNISOLO_CLOUD_ORIGIN).toString();
}
