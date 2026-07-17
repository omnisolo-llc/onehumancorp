import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export function DELETE(request: Request, { params }: { params: { id: string } }): Promise<Response> {
  return proxyBackendRequest(request, `/api/v1/tooltips/${params.id}`);
}
