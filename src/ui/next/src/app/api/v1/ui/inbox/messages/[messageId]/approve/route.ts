<<<<<<< HEAD
import { proxyBackendPost } from "../../../../backendProxy";

export async function POST(
  req: Request,
  { params }: { params: Promise<{ messageId: string }> }
) {
  const { messageId } = await params;
=======
import { proxyBackendPost } from "../../../../../backendProxy";

export async function POST(
  req: Request,
  { params }: { params: { messageId: string } }
) {
  const { messageId } = params;
>>>>>>> 97cc191c1 (perf: tokio RwLock, Redis pool, SSE streaming, unified WS, backpressure, React hooks)
  return proxyBackendPost(req, `/api/v1/ui/inbox/messages/${messageId}/approve`);
}
