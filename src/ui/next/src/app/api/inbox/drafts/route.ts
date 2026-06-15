import { NextResponse, NextRequest } from 'next/server';
import { proxyBackendGet } from "../../../backendProxy";

export async function GET(req: NextRequest) {
  return proxyBackendGet(req, "/api/v1/inbox/drafts");
}
