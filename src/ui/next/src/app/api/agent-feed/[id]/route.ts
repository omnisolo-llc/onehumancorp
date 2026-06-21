import { proxyBackendPut } from "../../ui/backendProxy";

export async function PUT(req: Request, { params }: { params: { id: string } }) {
  // It seems the backend expects `{id}/state`
  const { id } = params;
  return proxyBackendPut(req, `/api/agent-feed/${id}/state`);
}
