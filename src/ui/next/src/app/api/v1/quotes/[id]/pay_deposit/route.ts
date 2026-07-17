import { NextRequest } from "next/server";
import { proxyBackendRequest } from "../../../../../../lib/api/proxy";
import { quoteBackendPath } from "../../quoteBackend";

export async function POST(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  const backendUrl = `${quoteBackendPath(params.id)}/pay_deposit`;
  return proxyBackendRequest(request, backendUrl, {
    method: "POST",
  });
}
