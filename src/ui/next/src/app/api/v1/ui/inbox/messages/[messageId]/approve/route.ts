import { proxyBackendPost } from "../../../../backendProxy";

export async function POST(
  req: Request,
  { params }: { params: { messageId: string } }
) {
  const { messageId } = await params;
  return proxyBackendPost(req, `/api/v1/ui/inbox/messages/${messageId}/approve`);
}
