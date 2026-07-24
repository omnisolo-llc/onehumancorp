import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  return proxyBackendRequest(req, "/api/v1/omnichannel/integrations");
}
