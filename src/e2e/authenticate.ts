import type { APIRequestContext } from '@playwright/test';

const MAX_ERROR_BODY_BYTES = 1024;

export type E2ECredentials = {
  username: string;
  password: string;
  organizationId?: string;
};

function safeErrorBody(body: Buffer): string {
  const bounded = body.subarray(0, MAX_ERROR_BODY_BYTES).toString('utf8');
  return bounded
    .replace(
      /("(?:access_token|refresh_token|token|cookie|authorization)"\s*:\s*)"[^"]*"/gi,
      '$1"[redacted]"',
    )
    .replace(/\bBearer\s+[^\s",}]+/gi, 'Bearer [redacted]');
}

export async function authenticateRequest(
  request: APIRequestContext,
  credentials: E2ECredentials,
  origin: string,
): Promise<void> {
  const response = await request.post('/api/v1/auth/login', {
    headers: {
      origin,
      'sec-fetch-site': 'same-origin',
    },
    data: {
      username: credentials.username,
      password: credentials.password,
      ...(credentials.organizationId
        ? { organization_id: credentials.organizationId }
        : {}),
    },
  });

  if (!response.ok()) {
    const body = safeErrorBody(await response.body());
    console.error(`auth error: `, `auth error: `,
      `E2E authentication failed with HTTP ${response.status()}: ${body || '<empty response>'}`,
    );
  }
}
