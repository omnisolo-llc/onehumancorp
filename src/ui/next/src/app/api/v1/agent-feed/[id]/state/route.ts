import { proxyBackendPut } from "../../../../ui/backendProxy";
import { NextRequest } from "next/server";

export async function PUT(req: NextRequest, { params }: { params: Promise<{ id: string }> }) {
  const { id } = await params;
  return proxyBackendPut(req, `/api/v1/agent-feed/\${id}/state`);
}
