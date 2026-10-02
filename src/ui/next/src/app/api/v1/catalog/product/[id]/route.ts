import { proxyBackendRequest, stripBrowserIdentityJsonRequestBody } from '@/lib/auth/backendTransport';
type RouteContext = { params: Promise<{ id: string }> };
export async function PUT(request: Request, context: RouteContext): Promise<Response> {
  const { id } = await context.params;
  if (id === '.' || id === '..' || !/^[A-Za-z0-9._~-]{1,128}$/.test(id)) return Response.json({ error: 'invalid product id' }, { status: 400 });
  return proxyBackendRequest(request, `/api/v1/catalog/product/${id}`, { forwardQuery: false, transformRequestBody: stripBrowserIdentityJsonRequestBody });
}
