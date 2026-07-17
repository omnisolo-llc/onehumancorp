import { proxyBackendPost } from "../../ui/backendProxy";
import { NextResponse } from "next/server";

export async function POST(req: Request) {
  return proxyBackendPost(req, "/api/v1/dev/simulate-triage-item");
}
