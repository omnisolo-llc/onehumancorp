import { proxyBackendRequest } from "@/lib/auth/backendTransport";

type EdgeContext = { params: Promise<{ tenantId: string; siteId: string }> };
const SAFE_ID = /^[A-Za-z0-9._-]{1,128}$/;

export async function GET(request: Request, context: EdgeContext) {
  const { tenantId, siteId } = await context.params;
  if (!SAFE_ID.test(tenantId) || !SAFE_ID.test(siteId)) {
    return Response.json({ error: "invalid builder identifier" }, { status: 400 });
  }
  return proxyBackendRequest(request, `/api/v1/builder/edge/${tenantId}/${siteId}`, {
    suppressRequestBody: true,
  });
}
