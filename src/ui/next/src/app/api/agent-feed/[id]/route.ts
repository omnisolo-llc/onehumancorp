import { proxyBackendPatch } from "../../ui/backendProxy";
import { NextRequest } from "next/server";

export async function PATCH(req: NextRequest, { params }: { params: Promise<{ id: string }> }) {
  const p = await params;
  return proxyBackendPatch(req, `/api/agent-feed/${p.id}`);
}
