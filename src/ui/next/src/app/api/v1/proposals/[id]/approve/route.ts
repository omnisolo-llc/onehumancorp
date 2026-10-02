import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export const runtime = "nodejs";

export async function POST(request: Request, { params }: { params: Promise<{ id: string }> }): Promise<Response> {
  const resolvedParams = await params;
  return proxyBackendRequest(request, `/api/v1/proposals/${resolvedParams.id}/approve`, {
    forwardQuery: false,
  });
}
