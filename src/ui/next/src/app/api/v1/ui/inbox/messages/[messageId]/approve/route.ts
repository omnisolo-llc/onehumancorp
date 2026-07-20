import { proxyBackendPost } from "@/app/api/v1/ui/backendProxy";

export async function POST(
  req: Request,
  { params }: { params: Promise<{ messageId: string }> }
) {
  const { messageId } = await params;
  return proxyBackendPost(req, `/api/v1/ui/inbox/messages/${messageId}/approve`);
}
