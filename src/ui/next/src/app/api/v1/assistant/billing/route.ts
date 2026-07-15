import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { privateJson } from "../assistantBackend";

export const dynamic = "force-dynamic";

function finiteNumber(value: unknown): number | null {
  return typeof value === "number" && Number.isFinite(value) ? value : null;
}

export async function GET(request: Request): Promise<Response> {
  const response = await proxyBackendRequest(request, "/api/v1/billing/my-plan");
  if (!response.ok) return response;
  const value = await response.json().catch(() => null);
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    return privateJson(502, { error: "invalid billing response" });
  }
  const data = value as Record<string, unknown>;
  const used = finiteNumber(data.ai_actions_used);
  const limit = finiteNumber(data.ai_actions_limit);
  const storage = finiteNumber(data.storage_used_bytes);
  const storageLimit = finiteNumber(data.storage_limit_bytes);
  const nextBill = finiteNumber(data.next_bill_estimated);
  if (
    typeof data.current_plan !== "string" ||
    used === null ||
    limit === null ||
    storage === null ||
    storageLimit === null ||
    nextBill === null
  ) {
    return privateJson(502, { error: "invalid billing response" });
  }
  return privateJson(200, {
    plan: data.current_plan,
    aiActionsUsed: used,
    aiActionsLimit: limit,
    storageUsedGB: Number((storage / 1_073_741_824).toFixed(2)),
    storageLimitGB: Number((storageLimit / 1_073_741_824).toFixed(2)),
    estimatedNextBill: nextBill / 100,
  });
}
