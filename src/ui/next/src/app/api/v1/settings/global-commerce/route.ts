import { proxyBackendGet, proxyBackendPut } from "@/app/api/v1/ui/backendProxy";

export async function GET(request: Request) {
  return proxyBackendGet(request, "/api/v1/settings/global-commerce");
}

export async function PUT(request: Request) {
  return proxyBackendPut(request, "/api/v1/settings/global-commerce");
}
