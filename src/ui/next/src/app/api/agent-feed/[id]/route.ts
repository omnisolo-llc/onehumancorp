import { proxyBackendPut } from "../../ui/backendProxy";
import { NextRequest } from "next/server";

export async function PUT(req: NextRequest, { params }: { params: Promise<{ id: string }> }) {
  const p = await params;
  const id = p.id;
  return proxyBackendPut(req, `/api/agent-feed/${id}/state`);
}
