import { parseAuthRuntimeConfig, type AuthRuntimeConfig } from "./runtimeConfig";
import { openSession } from "./sessionCodec";
import { parseSessionKeyRing, type SessionKeyRing } from "./sessionKeys";
import { parseSessionCookieHeader, sessionCodecContext } from "./sessionCookie";
import type { WebSession } from "./sessionTypes";

export type ServerSessionDependencies = Readonly<{
  config: AuthRuntimeConfig;
  ring: SessionKeyRing;
  now: () => number;
}>;

export async function readServerSession(
  request: Request,
  dependencies: ServerSessionDependencies,
): Promise<WebSession | null> {
  const parsed = parseSessionCookieHeader(
    request.headers.get("cookie"),
    dependencies.config,
  );
  if (parsed.invalid || parsed.value === null) return null;
  try {
    return await openSession(
      parsed.value,
      dependencies.ring,
      sessionCodecContext(dependencies.config),
      dependencies.now(),
    );
  } catch {
    return null;
  }
}

let liveDependencies: Promise<ServerSessionDependencies> | undefined;

async function dependenciesFromEnvironment(): Promise<ServerSessionDependencies> {
  const config = parseAuthRuntimeConfig(process.env);
  const ring = await parseSessionKeyRing(process.env);
  return { config, ring, now: () => Math.floor(Date.now() / 1_000) };
}

export function liveServerSessionDependencies(): Promise<ServerSessionDependencies> {
  liveDependencies ??= dependenciesFromEnvironment();
  return liveDependencies;
}
