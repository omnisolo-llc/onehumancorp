import { proxyBackendPost } from "../../backendProxy";

export async function POST(req: Request) {
  return proxyBackendPost(req, "/api/v1/ui/opportunities/stage");
}
