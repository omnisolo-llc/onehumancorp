import { proxyBackendRequest } from '@/lib/auth/backendTransport';

export async function GET(request: Request, context: { params: Promise<{ operationId: string }> }): Promise<Response> {
  const { operationId } = await context.params;
  if (!/^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$/.test(operationId)) {
    return Response.json({ error: 'invalid cash operation ID' }, { status: 400, headers: { 'cache-control': 'private, no-store' } });
  }
  return proxyBackendRequest(request, `/api/v1/payments/terminal/commit/${operationId}`, { forwardQuery: false });
}
