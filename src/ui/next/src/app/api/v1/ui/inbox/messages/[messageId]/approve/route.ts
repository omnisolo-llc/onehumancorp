import { proxyBackendPost } from "@/app/api/v1/ui/backendProxy";

export async function POST(
  req: Request,
  { params }: { params: { messageId: string } }
) {
  const { messageId } = params;
  return proxyBackendPost(req, `/api/v1/ui/inbox/messages/${messageId}/approve`);
}
