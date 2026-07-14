export function isTrustedMutationOrigin(headers: Headers, canonicalOrigin: string): boolean {
  const origin = headers.get("origin");
  const fetchSite = headers.get("sec-fetch-site");
  if (origin === null || fetchSite !== "same-origin" || origin === "null") return false;
  if (origin !== canonicalOrigin) return false;
  try {
    const parsed = new URL(origin);
    return (
      parsed.origin === canonicalOrigin &&
      parsed.username === "" &&
      parsed.password === "" &&
      parsed.pathname === "/" &&
      parsed.search === "" &&
      parsed.hash === ""
    );
  } catch {
    return false;
  }
}
