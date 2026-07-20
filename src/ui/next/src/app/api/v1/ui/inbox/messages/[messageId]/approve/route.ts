import { proxyBackendPost } from "../../../../backendProxy";

export async function POST(
  req: Request,
  context: { params: Promise<{ messageId: string }> }
) {
  const params = await context.params;
  const { messageId } = params;
  return proxyBackendPost(req, `/api/v1/ui/inbox/messages/${messageId}/approve`);
}
