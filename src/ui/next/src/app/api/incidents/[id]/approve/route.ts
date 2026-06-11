import { proxyBackendPost } from "../../../backendProxy";

export async function POST(req: Request, { params }: { params: { id: string } }) {
  const url = new URL(req.url);
  const searchParams = url.searchParams.toString();
  const path = `/api/incidents/${params.id}/approve${searchParams ? \`?\${searchParams}\` : ''}`;
  return proxyBackendPost(req, path);
}
