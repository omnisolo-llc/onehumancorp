import { proxyBackendGet } from "../../backendProxy";

export async function GET(req: Request) {
  const url = new URL(req.url);
  const forwardQuery = url.searchParams.has("tenant_id") || url.searchParams.has("mobile_optimized");
  return proxyBackendGet(req, "/api/v1/ui/dashboard/unified-feed", forwardQuery ? { forwardQuery: true } : undefined);
}
