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
  const config = parseAuthRuntimeConfig(process.env);
  const ring = await parseSessionKeyRing(process.env);
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
  } catch (err: unknown) {
    console.error(
      "auth.middleware.configuration_unavailable: " +
        (err instanceof Error ? err.message : String(err)) +
        (err instanceof Error && err.stack ? `\n${err.stack}` : "")
    );
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
