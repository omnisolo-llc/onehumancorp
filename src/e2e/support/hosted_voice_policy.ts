import { isRuntimeReadUnavailable, runtimeUnavailableMessage } from './runtime_policy';

type SmokeActor = { userId: string; tenantId: string };
const savingsPath = '/api/v1/growth/time-savings';

function isMeasuredSavingsRequest(input: {
  url: string; origin: string; method: string; status: number; headers: Record<string, string>;
}, actor?: SmokeActor | null): boolean {
  try {
    const url = new URL(input.url);
    return !!actor?.userId && !!actor.tenantId
      && input.headers['x-ohc-expected-user'] === actor.userId
      && input.headers['x-ohc-expected-tenant'] === actor.tenantId
      && url.origin === new URL(input.origin).origin && url.pathname === savingsPath
      && !url.search && !url.hash && input.method === 'GET' && input.status === 501;
  } catch { return false; }
}

function isMeasuredSavingsUnavailableBody(body: unknown): boolean {
  if (!body || typeof body !== 'object' || Array.isArray(body)) return false;
  const value = body as Record<string, unknown>;
  return Object.keys(value).sort().join(',') === 'capability,code,message,success'
    && value.success === false && value.code === 'capability_unavailable'
    && value.capability === 'measured_time_savings'
    && value.message === 'No verified result is available. This capability is not implemented; no action was completed.';
}

/** Only the resource diagnostic belonging to the verified savings receipt. */
export function isVerifiedSavingsPolicyDiagnostic(text: string, url: string, verifiedUrls: Set<string>): boolean {
  try {
    const parsed = new URL(url);
    return verifiedUrls.has(url) && parsed.pathname === savingsPath && !parsed.search && !parsed.hash
      && /^Failed to load resource: the server responded with a status of 501(?: \(Not Implemented\))?$/.test(text);
  } catch { return false; }
}

/** The real hosted deployment denies instance-global voice authority. */
export function isHostedVoiceUnavailable(input: {
  url: string; origin: string; method: string; status: number; body: unknown;
}): boolean {
  try {
    const url = new URL(input.url);
    if (url.origin !== new URL(input.origin).origin || url.pathname !== '/api/v1/settings/voice' || url.search || url.hash || input.method !== 'GET' || input.status !== 403) return false;
    const body = input.body;
    if (!body || typeof body !== 'object' || Array.isArray(body)) return false;
    const value = body as Record<string, unknown>;
    return Object.keys(value).sort().join(',') === 'error,provisioning_available,provisioning_block_reason,success'
      && value.success === false && value.provisioning_available === false
      && value.error === 'hosted_global_provisioning_unavailable'
      && value.provisioning_block_reason === value.error;
  } catch { return false; }
}

/** Suppress only the browser resource diagnostic for a verified policy response. */
export function isVerifiedVoicePolicyDiagnostic(text: string, url: string, verifiedUrls: Set<string>): boolean {
  return verifiedUrls.has(url) && /^Failed to load resource: the server responded with a status of 403(?: \(Forbidden\))?$/.test(text);
}

/** Record the actual browser response; caller awaits the returned body checks. */
export function recordSmokeHttpResponse(response: {
  status(): number; url(): string; request(): { method(): string; headers?(): Record<string, string> }; json(): Promise<unknown>;
}, origin: string, results: { failures: string[]; httpFailures: string[]; verifiedPolicyUrls: Set<string>; policyChecks: Promise<void>[] }, actor?: SmokeActor | null): void {
  if (response.status() >= 500) {
    const input = { url: response.url(), origin, method: response.request().method(), status: response.status() };
    const savingsCandidate = isMeasuredSavingsRequest({ ...input, headers: response.request().headers?.() ?? {} }, actor);
    if (!savingsCandidate && !isRuntimeReadUnavailable({ ...input, body: { error: runtimeUnavailableMessage } })) {
      results.failures.push(`${response.status()} ${response.url()}`);
      return;
    }
    results.policyChecks.push((async () => {
      const body: unknown = await response.json().catch(() => null);
      if ((savingsCandidate && isMeasuredSavingsUnavailableBody(body)) || isRuntimeReadUnavailable({ ...input, body })) results.verifiedPolicyUrls.add(response.url());
      else results.failures.push(`${response.status()} ${response.url()}`);
    })());
  }
  else if (response.status() >= 400 && !response.url().includes('e2e-route-record')) {
    let candidate = false;
    try {
      const url = new URL(response.url());
      candidate = url.origin === new URL(origin).origin && url.pathname === '/api/v1/settings/voice'
        && !url.search && !url.hash && response.request().method() === 'GET' && response.status() === 403;
    } catch { /* An invalid response URL is an ordinary HTTP failure. */ }
    if (!candidate) {
      results.httpFailures.push(`${response.status()} ${response.url()}`);
      return;
    }
    results.policyChecks.push((async () => {
      const body: unknown = await response.json().catch(() => null);
      if (isHostedVoiceUnavailable({ url: response.url(), origin, method: response.request().method(), status: response.status(), body })) {
        results.verifiedPolicyUrls.add(response.url());
      } else {
        results.httpFailures.push(`${response.status()} ${response.url()}`);
      }
    })());
  }
}
