import { NextResponse, type NextRequest } from "next/server";
import { parseAuthRuntimeConfig } from "@/lib/auth/runtimeConfig";
import { parseSessionKeyRing } from "@/lib/auth/sessionKeys";
import { cookieDeletion, serializeSessionCookie } from "@/lib/auth/sessionCookie";
import {
  evaluateAuthMiddleware,
  isPublicFrameworkAsset,
  type MiddlewareDependencies,
} from "@/lib/auth/middlewareCore";

let liveDependencies: Promise<MiddlewareDependencies> | undefined;

export function _resetLiveDependencies(): void {
  liveDependencies = undefined;
}

async function dependenciesFromEnvironment(): Promise<MiddlewareDependencies> {
  // Edge bundles only expose environment variables that Next can identify as
  // static reads. Keep this allowlist explicit instead of passing process.env.
  const environment = {
    OHC_WEB_LOCAL_DEV: process.env.OHC_WEB_LOCAL_DEV,
    OHC_WEB_CANONICAL_ORIGIN: process.env.OHC_WEB_CANONICAL_ORIGIN,
    BACKEND_URL: process.env.BACKEND_URL,
    OHC_WEB_SESSION_KEY_ID: process.env.OHC_WEB_SESSION_KEY_ID,
    OHC_WEB_SESSION_SECRET: process.env.OHC_WEB_SESSION_SECRET,
    OHC_WEB_SESSION_PREVIOUS_KEY_ID: process.env.OHC_WEB_SESSION_PREVIOUS_KEY_ID,
    OHC_WEB_SESSION_PREVIOUS_SECRET: process.env.OHC_WEB_SESSION_PREVIOUS_SECRET,
  };
  const config = parseAuthRuntimeConfig(environment);
  const ring = await parseSessionKeyRing(environment);
  return { config, ring, now: () => Math.floor(Date.now() / 1_000) };
}

function copyHeaders(from: Headers, to: Headers): void {
  from.forEach((value, name) => to.set(name, value));
}

export async function middleware(request: NextRequest): Promise<NextResponse> {
  if (isPublicFrameworkAsset(request)) return NextResponse.next();

  let dependencies: MiddlewareDependencies;
  try {
    liveDependencies ??= dependenciesFromEnvironment();
    dependencies = await liveDependencies;
  } catch (error) {
    const msg = error instanceof Error ? error.stack || error.message : String(error);
    console.error("auth.middleware.configuration_unavailable: " + msg);
    return new NextResponse(JSON.stringify({ error: "authentication unavailable" }), {
      status: 503,
      headers: {
        "content-type": "application/json; charset=utf-8",
        "cache-control": "private, no-store",
        pragma: "no-cache",
      },
    });
  }

  const outcome = await evaluateAuthMiddleware(request, dependencies);
  let response: NextResponse;
  if (outcome.kind === "next") {
    response = NextResponse.next();
  } else if (outcome.kind === "redirect") {
    response = NextResponse.redirect(new URL(outcome.location, request.url));
  } else {
    response = new NextResponse(outcome.body, {
      status: outcome.status,
    });
  }
  copyHeaders(outcome.headers, response.headers);
  if (outcome.clearCookie) {
    response.headers.append(
      "set-cookie",
      serializeSessionCookie(cookieDeletion(dependencies.config)),
    );
  }
  return response;
}

export const config = {
  matcher: "/:path*",
};
