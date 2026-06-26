import { proxyBackendGet } from "@/app/api/ui/backendProxy";

export async function GET(request: Request) {
  return proxyBackendGet(request, "/api/ohc_job_queue");
}
