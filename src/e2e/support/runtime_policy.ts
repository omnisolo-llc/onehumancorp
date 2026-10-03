export const runtimeUnavailableMessage = 'Agent runtime is not configured; no work was dispatched';

/** The SONA dashboard currently needs the explicitly configured tenant runtime. */
export function isSonaRuntimeUnavailable(input: { url: string; origin: string; method: string; status: number; body: unknown }): boolean {
  try {
    const url = new URL(input.url);
    if (url.origin !== new URL(input.origin).origin || url.pathname !== '/api/v1/sona' || url.search || url.hash
      || input.method !== 'GET' || input.status !== 503) return false;
    const body = input.body;
    return !!body && typeof body === 'object' && !Array.isArray(body)
      && Object.keys(body).join(',') === 'error' && 'error' in body && body.error === runtimeUnavailableMessage;
  } catch { return false; }
}

export function isVerifiedRuntimePolicyDiagnostic(text: string, url: string, verifiedUrls: Set<string>): boolean {
  return verifiedUrls.has(url) && new URL(url).pathname === '/api/v1/sona'
    && /^Failed to load resource: the server responded with a status of 503(?: \(Service Unavailable\))?$/.test(text);
}
