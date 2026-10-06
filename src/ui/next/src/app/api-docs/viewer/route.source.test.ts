import { createHash } from 'node:crypto';
import { describe, expect, it } from 'vitest';
import { GET } from './route';
import { VIEWER_SCRIPT } from '@/lib/swaggerViewerDocument';
import { classifyRequest } from '@/lib/auth/publicRoutes';

describe('authenticated Swagger viewer document', () => {
  it('serves a private document with an exact inline-script CSP hash', async () => {
    const response = GET();
    expect(response.headers.get('content-type')).toBe('text/html; charset=utf-8');
    expect(response.headers.get('cache-control')).toBe('private, no-store');
    expect(response.headers.get('content-security-policy')).toContain(`'sha256-${createHash('sha256').update(VIEWER_SCRIPT).digest('base64')}'`);
    expect(response.headers.get('content-security-policy')).toContain("frame-ancestors 'self'");
    expect(await response.text()).toContain(`<script>${VIEWER_SCRIPT}</script>`);
  });
  it.each(['/api-docs/viewer', '/vendor/swagger-ui/dist/swagger-ui-bundle.js', '/vendor/swagger-ui/dist/swagger-ui.css', '/vendor/swagger-ui/dist/oauth2-redirect.html'])('does not make %s a public auth exception', pathname => {
    expect(classifyRequest({ method: 'GET', pathname, invocation: 'page' }).access).not.toBe('public');
  });
});
