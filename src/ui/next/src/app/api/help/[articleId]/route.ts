import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { helpArticleBackendPath, invalidArticleId } from "../helpBackend";

export async function GET(
  request: Request,
  context: { params: Promise<{ articleId: string }> },
): Promise<Response> {
  let path: string;
  try {
    path = helpArticleBackendPath((await context.params).articleId);
  } catch {
    return invalidArticleId();
  }
  return proxyBackendRequest(request, path);
}
