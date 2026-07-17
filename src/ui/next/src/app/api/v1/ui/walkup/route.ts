import { proxyBackendPost } from "../backendProxy";

export async function POST(req: Request) {
  // Use a dedicated walkup backend route
  return proxyBackendPost(req, "/api/v1/walkup");
}
