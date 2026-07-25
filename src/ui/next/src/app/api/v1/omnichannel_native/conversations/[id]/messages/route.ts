import { proxyBackendRequest } from "@/lib/auth/backendTransport";
export const runtime = "nodejs";

export function GET(
  request: Request,
  { params }: { params: { id: string } }
): Promise<Response> {
  return proxyBackendRequest(request, `/api/v1/omnichannel_native/conversations/${params.id}/messages`, {
    forwardQuery: true,
    suppressRequestBody: true,
  });
}
