import { startOidc } from "@/lib/auth/oidcFlow";

export async function GET(
  request: Request,
  context: { params: Promise<{ provider: string }> },
): Promise<Response> {
  const { provider } = await context.params;
  return startOidc(request, provider);
}
