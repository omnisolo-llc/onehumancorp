import { proxyBackendPut } from "../../ui/backendProxy";
import { NextRequest } from "next/server";

export async function PUT(req: NextRequest, { params }: { params: Promise<{ id: string }> }) {
  const p = await params;
  return proxyBackendPut(req, `/api/agent-feed/${p.id}/state`);
}
