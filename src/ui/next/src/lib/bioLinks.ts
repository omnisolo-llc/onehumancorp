/** The private profile editor supports absolute web links, without URL rewriting. */
export function isSupportedBioUrl(value: string): boolean {
  if (!/^https?:\/\//i.test(value) || /[\s\p{Cc}\\]/u.test(value)) return false;
  try {
    const url = new URL(value);
    return (url.protocol === 'http:' || url.protocol === 'https:') && url.hostname.length > 0;
  } catch {
    return false;
  }
}
