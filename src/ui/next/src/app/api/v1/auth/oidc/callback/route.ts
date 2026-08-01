import { finishOidc } from "@/lib/auth/oidcFlow";

export async function GET(request: Request): Promise<Response> {
  return finishOidc(request);
}
