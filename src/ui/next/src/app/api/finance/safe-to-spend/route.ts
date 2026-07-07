import { proxyBackendGet } from "../../ui/backendProxy";

export async function GET(req: Request) {
  return proxyBackendGet(req, "/api/v1/payments/ledger/api/finance/safe-to-spend");
}
