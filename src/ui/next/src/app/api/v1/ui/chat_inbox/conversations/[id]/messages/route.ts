import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { NextRequest } from "next/server";

export const runtime = "nodejs";

export async function GET(request: NextRequest, { params }: { params: { id: string } }): Promise<Response> {
  const { id } = params;
  return proxyBackendRequest(request, `/api/v1/ui/chat/conversations/${id}/messages`, {
    forwardQuery: true,
    suppressRequestBody: true,
  });
}

export async function POST(request: NextRequest, { params }: { params: { id: string } }): Promise<Response> {
  const { id } = params;
  return proxyBackendRequest(request, `/api/v1/ui/chat/conversations/${id}/messages`);
}
