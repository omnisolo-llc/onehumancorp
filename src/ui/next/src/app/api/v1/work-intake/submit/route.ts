import { proxyBackendRequest } from '@/lib/auth/backendTransport';

function field(formData: FormData, name: string, maximum: number): string | null {
  const value = formData.get(name);
  if (typeof value !== 'string') return null;
  const trimmed = value.trim();
  return trimmed && trimmed.length <= maximum && !trimmed.includes('\0') ? trimmed : null;
}

function validEmail(value: string): boolean {
  const [local, domain, ...rest] = value.split('@');
  return rest.length === 0 && Boolean(local) && Boolean(domain?.includes('.')) && !/\s/.test(value);
}

export async function POST(request: Request) {
  try {
    const formData = await request.formData();
    const name = field(formData, 'name', 200);
    const email = field(formData, 'email', 320);
    const details = field(formData, 'request', 10_000) ?? field(formData, 'details', 10_000);
    if (!name || !email || !validEmail(email) || !details) {
      return Response.json({ success: false, error: 'invalid request' }, { status: 400 });
    }

    const backendRequest = new Request(request.url, {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({
        source: 'work_intake',
        message: details,
        customer_name: name,
        customer_email: email,
      }),
      signal: request.signal,
    });
    const backendResponse = await proxyBackendRequest(backendRequest, '/api/v1/agents/webhook', {
      forwardQuery: false,
      requestContentType: 'application/json',
    });
    if (!backendResponse.ok) {
      return Response.json({ success: false, error: 'submission unavailable' }, { status: backendResponse.status });
    }
    const headers = new Headers({ 'cache-control': 'no-store' });
    const contentType = backendResponse.headers.get('content-type');
    if (contentType) headers.set('content-type', contentType);
    return new Response(backendResponse.body, { status: backendResponse.status, headers });
  } catch {
    return Response.json({ success: false, error: 'invalid request' }, { status: 400 });
  }
}
