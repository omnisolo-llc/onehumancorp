/** Browser destinations accepted by both the bio editor and its public page. */
export function safeBioHref(value: unknown): string | null {
  if (typeof value !== 'string' || value.includes('\\') ||
      Array.from(value).some(character => character.charCodeAt(0) < 32 || character.charCodeAt(0) === 127)) {
    return null;
  }
  const href = value.trim();
  if (!href || href.startsWith('//')) return null;
  // Existing saved profiles can link to routes on the same deployment.
  if (href.startsWith('/')) return href;
  if (!/^https?:\/\//i.test(href)) return null;
  try {
    const url = new URL(href);
    return (url.protocol === 'https:' || url.protocol === 'http:') &&
      !!url.hostname && !url.username && !url.password ? href : null;
  } catch {
    return null;
  }
}
