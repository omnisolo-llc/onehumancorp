<<<<<<< HEAD
import { proxyBackendGet } from "../backendProxy";
=======
import { proxyBackendGet } from "../../backendProxy";
>>>>>>> 359e384d (feat(memory): Implement AgentMemoryService for tenant-isolated episodic memory)

export async function GET(req: Request) {
  return proxyBackendGet(req, "/api/ui/opportunities");
}
