import {
  publicAuthDependencies,
  proxyPublicAuthentication,
  unavailableAuthenticationResponse,
} from "@/lib/auth/publicBackendProxy";

export async function POST(request: Request): Promise<Response> {
  try {
    return await proxyPublicAuthentication(
      request,
      await publicAuthDependencies(),
      "/api/v1/auth/registration/email/verify",
      "POST",
    );
  } catch {
    return unavailableAuthenticationResponse();
  }
}
