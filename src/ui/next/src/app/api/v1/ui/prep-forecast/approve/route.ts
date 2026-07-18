import { NextResponse } from "next/server";
import { proxyBackendPost } from "../../backendProxy";

export async function POST(request: Request) {
  return proxyBackendPost(request, "/api/v1/ui/prep-forecast/approve");
}
