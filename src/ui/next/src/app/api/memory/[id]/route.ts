import { proxyBackendDelete } from "../../ui/backendProxy";
import { NextRequest } from "next/server";

export async function DELETE(req: NextRequest, context: { params: Promise<{ id: string }> }) {
  const p = await context.params;
  const id = p.id;
  return proxyBackendDelete(req, `/api/memory/${id}`);
}
