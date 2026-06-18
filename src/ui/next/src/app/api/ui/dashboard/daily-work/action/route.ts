import { proxyBackendPost } from "../../../backendProxy";
import { NextResponse } from "next/server";

export async function POST(req: Request) {
  const { searchParams } = new URL(req.url);
  const id = searchParams.get("id");
  if (!id) {
    return NextResponse.json({ error: "Missing ID" }, { status: 400 });
  }
  return proxyBackendPost(req, `/api/ui/dashboard/daily-work/action/${id}`);
}
