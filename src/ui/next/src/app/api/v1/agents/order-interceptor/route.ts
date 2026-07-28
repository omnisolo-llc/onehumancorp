import { proxyBackendPost } from "../../ui/backendProxy";\n\nexport async function POST(req: Request) {\n  return proxyBackendPost(req, "/api/v1/agents/order-interceptor");\n}
