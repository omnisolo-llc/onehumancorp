export async function loginForVisualAudit({
  authOrigin,
  username,
  password,
  organizationId,
  fetchImpl = globalThis.fetch,
}) {
  if (!authOrigin || !username || !password || !organizationId) {
    throw new Error('visual audit backend origin, username, password, and organization id are required');
  }

  const authBase = new URL(`${authOrigin.replace(/\/$/, '')}/`);
  const loginUrl = new URL('/api/v1/auth/login', authBase).toString();
  let response;
  try {
    response = await fetchImpl(loginUrl, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
        origin: authBase.origin,
        'sec-fetch-site': 'same-origin',
      },
      body: JSON.stringify({ username, password, organization_id: organizationId }),
    });
  } catch (error) {
    throw new Error(`visual audit login request failed: ${error instanceof Error ? error.message : String(error)}`);
  }

  let payload = null;
  try {
    payload = await response.json();
  } catch {
    // Keep the status-specific error below when the backend returns an empty body.
  }
  if (!response.ok) {
    throw new Error(`visual audit login failed with HTTP ${response.status}`);
  }

  const accessToken = payload?.token ?? payload?.access_token ?? payload?.accessToken;
  const user = payload?.user;
  const expiresAt = payload?.expires_at ?? payload?.expiresAt;
  const organization = user?.organization_id ?? user?.organizationId;
  if (
    typeof accessToken !== 'string' || accessToken.length === 0 ||
    typeof expiresAt !== 'number' || !Number.isSafeInteger(expiresAt) ||
    typeof user?.id !== 'string' || user.id.length === 0 ||
    typeof user?.username !== 'string' || user.username.length === 0 ||
    !Array.isArray(user.roles) || user.roles.some((role) => typeof role !== 'string') ||
    typeof organization !== 'string' || organization.length === 0
  ) {
    throw new Error('visual audit login returned an invalid authentication payload');
  }

  return {
    accessToken,
    expiresAt,
    user: {
      id: user.id,
      username: user.username,
      roles: user.roles,
      organizationId: organization,
    },
  };
}
