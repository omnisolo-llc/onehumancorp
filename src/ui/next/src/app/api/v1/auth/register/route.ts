import { proxyBackendPost } from '@/lib/auth/publicBackendProxy';
import {
  publicAuthDependencies,
  registerAndSealSession,
  unavailableAuthenticationResponse,
} from "@/lib/auth/publicBackendProxy";

export async function POST(request: Request): Promise<Response> {
  try {
    return await registerAndSealSession(request, await publicAuthDependencies());
  } catch {
    return unavailableAuthenticationResponse();
  }
}

// The underlying proxy functions throw and return status: 503 on failure.
