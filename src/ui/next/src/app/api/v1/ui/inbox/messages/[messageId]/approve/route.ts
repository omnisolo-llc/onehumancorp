import { proxyBackendPost } from "../../../../backendProxy";

export async function POST(
  req: Request,
  context: { params: Promise<{ messageId: string }> }
) {
  const { messageId } = await context.params;
  return proxyBackendPost(req, `/api/v1/ui/inbox/messages/${messageId}/approve`);
}
