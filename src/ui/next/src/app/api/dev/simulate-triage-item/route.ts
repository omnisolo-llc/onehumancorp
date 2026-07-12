import { proxyBackendPost } from "../../ui/backendProxy";
import { NextResponse } from "next/server";

export async function POST(req: Request) {
  return proxyBackendPost(req, "/api/dev/simulate-triage-item");
}
