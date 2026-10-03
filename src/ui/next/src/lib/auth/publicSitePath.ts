const UUID = '[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}';
const publication = new RegExp('^/api/v1/public/sites/(' + UUID + ')(.*)$');
/** The only anonymous publication resources are a root, reviewed local page, or selected product. */
export function isPublicSiteDocumentPath(pathname: string): boolean {
  const found = publication.exec(pathname);
  if (!found) return false;
  const suffix = found[2];
  if (!suffix) return true;
  if (new RegExp('^/products/' + UUID + '$').test(suffix)) return true;
  if (!suffix.startsWith('/pages/')) return false;
  const encoded = suffix.slice('/pages'.length);
  try {
    const decoded = decodeURIComponent(encoded);
    if (encodeURI(decoded) !== encoded || new TextEncoder().encode(decoded).length > 512 || /[%?#\\]/.test(decoded)
      || Array.from(decoded).some(character => { const point = character.codePointAt(0)!; return point <= 31 || point >= 127 && point <= 159 || point >= 0xd800 && point <= 0xdfff; })) return false;
    return decoded.slice(1).split('/').every(part => part !== '' && part !== '.' && part !== '..');
  } catch { return false; }
}
