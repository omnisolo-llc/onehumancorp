import { proxyBackendGet } from "@/app/api/v1/ui/backendProxy";

export async function GET(request: Request) {
  return proxyBackendGet(request, "/api/v1/ohc_job_queue");
}
