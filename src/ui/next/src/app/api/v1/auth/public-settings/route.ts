import {
  publicAuthDependencies,
  proxyPublicAuthentication,
  unavailableAuthenticationResponse,
} from "@/lib/auth/publicBackendProxy";

export async function GET(request: Request): Promise<Response> {
  try {
    return await proxyPublicAuthentication(
      request,
      await publicAuthDependencies(),
      "/api/v1/auth/public-settings",
      "GET",
    );
  } catch {
    return unavailableAuthenticationResponse();
  }
}
