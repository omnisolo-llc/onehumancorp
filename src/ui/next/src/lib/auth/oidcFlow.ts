import {
  EncryptJWT,
  createRemoteJWKSet,
  jwtDecrypt,
  jwtVerify,
  type JWTPayload,
} from "jose";
import { publicAuthDependencies, type PublicAuthDependencies } from "./publicBackendProxy";
import { sealSession } from "./sessionCodec";
import { cookieForSession, serializeSessionCookie, sessionCodecContext } from "./sessionCookie";
import type { WebSession } from "./sessionTypes";
import { safeReturnPath } from "./url";

const STATE_COOKIE = "__Host-ohc_oidc_state";
const LOCAL_STATE_COOKIE = "ohc_oidc_state";
const STATE_SECONDS = 600;
const MAX_PROVIDER_RESPONSE = 65_536;

type ProviderConfig = Readonly<{
  key: "google" | "keycloak";
  issuer: string;
  clientId: string;
  clientSecret: string;
}>;

type Discovery = Readonly<{
  issuer: string;
  authorization_endpoint: string;
  token_endpoint: string;
  jwks_uri: string;
}>;

type OidcState = JWTPayload & Readonly<{
  provider: ProviderConfig["key"];
  state: string;
  nonce: string;
  verifier: string;
  redirectUri: string;
  returnTo: string;
}>;

function privateJson(status: number, message: string): Response {
  return Response.json(
    { error: message },
    { status, headers: { "cache-control": "private, no-store", pragma: "no-cache" } },
  );
}

function privateRedirect(location: URL, cookies: readonly string[] = []): Response {
  const headers = new Headers({
    location: location.href,
    "cache-control": "private, no-store",
    pragma: "no-cache",
  });
  for (const cookie of cookies) headers.append("set-cookie", cookie);
  return new Response(null, { status: 302, headers });
}

function base64url(bytes: Uint8Array): string {
  return Buffer.from(bytes).toString("base64url");
}

function randomValue(bytes = 32): string {
  return base64url(crypto.getRandomValues(new Uint8Array(bytes)));
}

async function boundedJson(response: Response): Promise<unknown> {
  const declared = response.headers.get("content-length");
  if (declared !== null && (!/^\d+$/.test(declared) || Number(declared) > MAX_PROVIDER_RESPONSE)) {
    throw new Error("provider response too large");
  }
  if (response.body === null) throw new Error("provider response missing");
  const reader = response.body.getReader();
  const chunks: Uint8Array[] = [];
  let total = 0;
  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      total += value.byteLength;
      if (total > MAX_PROVIDER_RESPONSE) throw new Error("provider response too large");
      chunks.push(value);
    }
  } finally {
    reader.releaseLock();
  }
  const body = new Uint8Array(total);
  let offset = 0;
  for (const chunk of chunks) {
    body.set(chunk, offset);
    offset += chunk.length;
  }
  return JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(body));
}

function providerConfig(key: string): ProviderConfig | null {
  if (key === "google") {
    const clientId = process.env.OHC_OIDC_GOOGLE_CLIENT_ID;
    const clientSecret = process.env.OHC_OIDC_GOOGLE_CLIENT_SECRET;
    return clientId && clientSecret
      ? { key: "google", issuer: "https://accounts.google.com", clientId, clientSecret }
      : null;
  }
  if (key === "keycloak") {
    const issuer = process.env.OHC_OIDC_KEYCLOAK_ISSUER?.replace(/\/$/, "");
    const clientId = process.env.OHC_OIDC_KEYCLOAK_CLIENT_ID;
    const clientSecret = process.env.OHC_OIDC_KEYCLOAK_CLIENT_SECRET;
    return issuer && clientId && clientSecret
      ? { key: "keycloak", issuer, clientId, clientSecret }
      : null;
  }
  return null;
}

function secureProviderUrl(value: unknown): string {
  if (typeof value !== "string" || value.length === 0 || value.length > 2_048) {
    throw new Error("invalid provider URL");
  }
  const url = new URL(value);
  if (url.protocol !== "https:" || url.username !== "" || url.password !== "") {
    throw new Error("invalid provider URL");
  }
  return url.href;
}

async function discovery(provider: ProviderConfig, dependencies: PublicAuthDependencies): Promise<Discovery> {
  const response = await dependencies.fetchImpl(
    `${provider.issuer}/.well-known/openid-configuration`,
    { redirect: "manual", cache: "no-store", signal: AbortSignal.timeout(5_000) },
  );
  if (!response.ok || response.status >= 300 && response.status < 400) {
    throw new Error("provider discovery unavailable");
  }
  const value = await boundedJson(response);
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("invalid provider discovery");
  }
  const document = value as Record<string, unknown>;
  if (document.issuer !== provider.issuer) throw new Error("provider issuer mismatch");
  return {
    issuer: provider.issuer,
    authorization_endpoint: secureProviderUrl(document.authorization_endpoint),
    token_endpoint: secureProviderUrl(document.token_endpoint),
    jwks_uri: secureProviderUrl(document.jwks_uri),
  };
}

async function providerIsEnabled(provider: ProviderConfig, dependencies: PublicAuthDependencies): Promise<boolean> {
  const response = await dependencies.fetchImpl(
    new URL("/api/v1/auth/public-settings", `${dependencies.config.backendOrigin}/`),
    { redirect: "manual", cache: "no-store", signal: AbortSignal.timeout(5_000) },
  );
  if (!response.ok) return false;
  const value = await boundedJson(response);
  if (value === null || typeof value !== "object" || Array.isArray(value)) return false;
  const providers = (value as { providers?: unknown }).providers;
  return Array.isArray(providers) && providers.some((candidate) =>
    candidate !== null && typeof candidate === "object" &&
    (candidate as { key?: unknown }).key === provider.key
  );
}

function stateCookieName(dependencies: PublicAuthDependencies): string {
  return dependencies.config.secureCookie ? STATE_COOKIE : LOCAL_STATE_COOKIE;
}

function readCookie(request: Request, name: string): string | null {
  const header = request.headers.get("cookie");
  if (header === null || header.length > 8_192) return null;
  const values = header.split(";").map((pair) => pair.trim()).filter((pair) => pair.startsWith(`${name}=`));
  if (values.length !== 1) return null;
  const value = values[0].slice(name.length + 1);
  return /^[A-Za-z0-9._-]{1,4096}$/.test(value) ? value : null;
}

function serializeStateCookie(name: string, value: string, secure: boolean): string {
  return [
    `${name}=${value}`,
    "Path=/api/v1/auth/oidc",
    `Max-Age=${STATE_SECONDS}`,
    "HttpOnly",
    secure ? "Secure" : "",
    "SameSite=Lax",
  ].filter(Boolean).join("; ");
}

function deleteStateCookie(name: string, secure: boolean): string {
  return [
    `${name}=`,
    "Path=/api/v1/auth/oidc",
    "Max-Age=0",
    "Expires=Thu, 01 Jan 1970 00:00:00 GMT",
    "HttpOnly",
    secure ? "Secure" : "",
    "SameSite=Lax",
  ].filter(Boolean).join("; ");
}

async function sealState(state: OidcState, dependencies: PublicAuthDependencies): Promise<string> {
  return new EncryptJWT(state)
    .setProtectedHeader({ alg: "dir", enc: "A256GCM", kid: dependencies.ring.active.id, typ: "JWT" })
    .setIssuer(dependencies.config.canonicalOrigin)
    .setAudience("onehumancorp-oidc-state")
    .setIssuedAt(dependencies.now())
    .setExpirationTime(dependencies.now() + STATE_SECONDS)
    .encrypt(dependencies.ring.active.key);
}

async function openState(compact: string, dependencies: PublicAuthDependencies): Promise<OidcState> {
  const { payload } = await jwtDecrypt(
    compact,
    async (header) => {
      if (header.kid === dependencies.ring.active.id) return dependencies.ring.active.key;
      if (header.kid === dependencies.ring.previous?.id) return dependencies.ring.previous.key;
      throw new Error("unknown state key");
    },
    {
      issuer: dependencies.config.canonicalOrigin,
      audience: "onehumancorp-oidc-state",
      clockTolerance: 5,
    },
  );
  const state = payload as Partial<OidcState>;
  if (
    (state.provider !== "google" && state.provider !== "keycloak") ||
    typeof state.state !== "string" || state.state.length > 128 ||
    typeof state.nonce !== "string" || state.nonce.length > 128 ||
    typeof state.verifier !== "string" || state.verifier.length > 128 ||
    typeof state.redirectUri !== "string" || state.redirectUri.length > 2_048 ||
    typeof state.returnTo !== "string" || state.returnTo.length > 2_048
  ) throw new Error("invalid OIDC state");
  return state as OidcState;
}

export async function startOidc(request: Request, providerKey: string): Promise<Response> {
  try {
    const dependencies = await publicAuthDependencies();
    const provider = providerConfig(providerKey);
    if (provider === null || !await providerIsEnabled(provider, dependencies)) {
      return privateJson(404, "OIDC provider unavailable");
    }
    const metadata = await discovery(provider, dependencies);
    const stateValue = randomValue();
    const nonce = randomValue();
    const verifier = randomValue();
    const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(verifier));
    const challenge = base64url(new Uint8Array(digest));
    const redirectUri = `${dependencies.config.canonicalOrigin}/api/v1/auth/oidc/callback`;
    const returnTo = safeReturnPath(new URL(request.url).searchParams.get("next"));
    const compact = await sealState(
      { provider: provider.key, state: stateValue, nonce, verifier, redirectUri, returnTo },
      dependencies,
    );
    const authorization = new URL(metadata.authorization_endpoint);
    authorization.search = new URLSearchParams({
      response_type: "code",
      client_id: provider.clientId,
      redirect_uri: redirectUri,
      scope: "openid email profile",
      state: stateValue,
      nonce,
      code_challenge: challenge,
      code_challenge_method: "S256",
    }).toString();
    return new Response(null, {
      status: 302,
      headers: {
        location: authorization.href,
        "cache-control": "private, no-store",
        "set-cookie": serializeStateCookie(
          stateCookieName(dependencies),
          compact,
          dependencies.config.secureCookie,
        ),
      },
    });
  } catch {
    console.error("auth.oidc.start_unavailable");
    return privateJson(503, "OIDC unavailable");
  }
}

function parseBackendLogin(value: unknown, now: number): Readonly<{
  token: string;
  expires_at: number;
  user: { id: string; username: string; roles: string[]; organization_id: string };
}> | null {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return null;
  const candidate = value as Record<string, unknown>;
  const user = candidate.user;
  if (
    typeof candidate.token !== "string" || candidate.token.length > 2_048 || /\s/.test(candidate.token) ||
    !Number.isSafeInteger(candidate.expires_at) || (candidate.expires_at as number) <= now ||
    user === null || typeof user !== "object" || Array.isArray(user)
  ) return null;
  const identity = user as Record<string, unknown>;
  if (
    typeof identity.id !== "string" || identity.id.length > 128 ||
    typeof identity.username !== "string" || identity.username.length > 254 ||
    typeof identity.organization_id !== "string" || identity.organization_id.length > 128 ||
    !Array.isArray(identity.roles) || !identity.roles.every((role) => typeof role === "string" && role.length <= 64)
  ) return null;
  return {
    token: candidate.token,
    expires_at: candidate.expires_at as number,
    user: {
      id: identity.id,
      username: identity.username,
      organization_id: identity.organization_id,
      roles: identity.roles as string[],
    },
  };
}

export async function finishOidc(request: Request): Promise<Response> {
  let dependencies: PublicAuthDependencies | undefined;
  try {
    dependencies = await publicAuthDependencies();
    const cookieName = stateCookieName(dependencies);
    const compact = readCookie(request, cookieName);
    if (compact === null) throw new Error("missing state cookie");
    const state = await openState(compact, dependencies);
    const query = new URL(request.url).searchParams;
    const code = query.get("code");
    const returnedState = query.get("state");
    if (
      code === null || code.length === 0 || code.length > 4_096 ||
      returnedState === null || returnedState !== state.state
    ) throw new Error("state mismatch");
    const provider = providerConfig(state.provider);
    if (provider === null || !await providerIsEnabled(provider, dependencies)) {
      throw new Error("provider disabled");
    }
    const metadata = await discovery(provider, dependencies);
    const tokenResponse = await dependencies.fetchImpl(metadata.token_endpoint, {
      method: "POST",
      headers: { "content-type": "application/x-www-form-urlencoded" },
      body: new URLSearchParams({
        grant_type: "authorization_code",
        code,
        redirect_uri: state.redirectUri,
        client_id: provider.clientId,
        client_secret: provider.clientSecret,
        code_verifier: state.verifier,
      }),
      redirect: "manual",
      cache: "no-store",
      signal: AbortSignal.timeout(5_000),
    });
    if (!tokenResponse.ok) throw new Error("token exchange denied");
    const tokenValue = await boundedJson(tokenResponse);
    if (tokenValue === null || typeof tokenValue !== "object" || Array.isArray(tokenValue)) {
      throw new Error("invalid token response");
    }
    const idToken = (tokenValue as { id_token?: unknown }).id_token;
    if (typeof idToken !== "string" || idToken.length === 0 || idToken.length > 16_384) {
      throw new Error("missing id token");
    }
    const verified = await jwtVerify(idToken, createRemoteJWKSet(new URL(metadata.jwks_uri)), {
      issuer: provider.issuer,
      audience: provider.clientId,
      algorithms: ["RS256"],
      requiredClaims: ["sub", "email", "email_verified", "nonce"],
    });
    if (verified.payload.nonce !== state.nonce || verified.payload.email_verified !== true) {
      throw new Error("unverified OIDC identity");
    }

    const backendResponse = await dependencies.fetchImpl(
      new URL("/api/v1/auth/oidc/session", `${dependencies.config.backendOrigin}/`),
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ provider: provider.key, id_token: idToken }),
        redirect: "manual",
        cache: "no-store",
        signal: AbortSignal.timeout(5_000),
      },
    );
    if (!backendResponse.ok) {
      const location = backendResponse.status === 403
        ? "/login?error=registration-closed"
        : backendResponse.status === 409
          ? "/login?error=link-required"
          : "/login?error=oidc-denied";
      return privateRedirect(
        new URL(location, dependencies.config.canonicalOrigin),
        [deleteStateCookie(cookieName, dependencies.config.secureCookie)],
      );
    }
    const backend = parseBackendLogin(await boundedJson(backendResponse), dependencies.now());
    if (backend === null) throw new Error("invalid backend session");
    const expiresAt = Math.min(backend.expires_at, dependencies.now() + 86_400);
    const session: WebSession = {
      version: 1,
      iat: dependencies.now(),
      exp: expiresAt,
      accessToken: backend.token,
      user: {
        id: backend.user.id,
        username: backend.user.username,
        roles: backend.user.roles,
        organizationId: backend.user.organization_id,
      },
    };
    const sealed = await sealSession(
      session,
      dependencies.ring,
      sessionCodecContext(dependencies.config),
      { now: dependencies.now(), backendExpiresAt: backend.expires_at },
    );
    return privateRedirect(
      new URL(safeReturnPath(state.returnTo), dependencies.config.canonicalOrigin),
      [
        serializeSessionCookie(cookieForSession(dependencies.config, sealed, dependencies.now(), expiresAt)),
        deleteStateCookie(cookieName, dependencies.config.secureCookie),
      ],
    );
  } catch {
    console.error("auth.oidc.callback_denied");
    const origin = dependencies?.config.canonicalOrigin ?? "https://cloud.omnisolo.co";
    const cookies = dependencies === undefined
      ? []
      : [deleteStateCookie(stateCookieName(dependencies), dependencies.config.secureCookie)];
    return privateRedirect(new URL("/login?error=oidc-denied", origin), cookies);
  }
}
