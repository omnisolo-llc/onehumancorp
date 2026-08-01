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
