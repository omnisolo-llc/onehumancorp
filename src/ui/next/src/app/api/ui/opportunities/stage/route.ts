<<<<<<< HEAD
import { proxyBackendPost } from "../../backendProxy";
=======
import { proxyBackendPost } from "../../../backendProxy";
>>>>>>> 359e384d (feat(memory): Implement AgentMemoryService for tenant-isolated episodic memory)

export async function POST(req: Request) {
  return proxyBackendPost(req, "/api/ui/opportunities/stage");
}
