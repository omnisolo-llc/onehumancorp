"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { type FormEvent, useEffect, useState } from "react";

const CHALLENGE_STORAGE_KEY = "ohc-registration-challenge";
const TICKET_STORAGE_KEY = "ohc-registration-ticket";

type Challenge = Readonly<{ challengeId: string; email: string }>;

export default function VerifyEmailPage() {
  const router = useRouter();
  const [challenge, setChallenge] = useState<Challenge | null>(null);
  const [ticket, setTicket] = useState<string | null>(null);
  const [code, setCode] = useState("");
  const [username, setUsername] = useState("");
  const [organizationId, setOrganizationId] = useState("");
  const [password, setPassword] = useState("");
  const [pending, setPending] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    try {
      const stored = JSON.parse(sessionStorage.getItem(CHALLENGE_STORAGE_KEY) ?? "null") as Partial<Challenge> | null;
      if (
        stored !== null &&
        typeof stored.challengeId === "string" &&
        stored.challengeId.length <= 64 &&
        typeof stored.email === "string" &&
        stored.email.length <= 254
      ) {
        setChallenge({ challengeId: stored.challengeId, email: stored.email });
      }
    } catch {
      sessionStorage.removeItem(CHALLENGE_STORAGE_KEY);
    }
  }, []);

  async function verify(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (pending || challenge === null) return;
    setPending(true);
    setError(null);
    try {
      const response = await fetch("/api/v1/auth/registration/email/verify", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ challenge_id: challenge.challengeId, code }),
      });
      const body: unknown = await response.json();
      const registrationTicket =
        body !== null && typeof body === "object" && "registration_ticket" in body
          ? (body as { registration_ticket?: unknown }).registration_ticket
          : undefined;
      if (!response.ok || typeof registrationTicket !== "string" || registrationTicket.length > 128) {
        throw new Error("verification denied");
      }
      sessionStorage.setItem(TICKET_STORAGE_KEY, registrationTicket);
      setTicket(registrationTicket);
      setCode("");
    } catch {
      setError("The code is invalid or expired. Request a new code and try again.");
    } finally {
      setPending(false);
    }
  }

  async function createAccount(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const registrationTicket = ticket ?? sessionStorage.getItem(TICKET_STORAGE_KEY);
    if (pending || registrationTicket === null) return;
    setPending(true);
    setError(null);
    try {
      const response = await fetch("/api/v1/auth/register?next=%2Fonboarding", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ registration_ticket: registrationTicket, organization_id: organizationId, username, password }),
      });
      const body: unknown = await response.json();
      const next = body !== null && typeof body === "object" && "next" in body
        ? (body as { next?: unknown }).next
        : undefined;
      if (!response.ok || typeof next !== "string" || !next.startsWith("/")) {
        throw new Error("registration denied");
      }
      sessionStorage.removeItem(CHALLENGE_STORAGE_KEY);
      sessionStorage.removeItem(TICKET_STORAGE_KEY);
      router.replace(next);
    } catch {
      setError("We could not create the account. Review the username and password requirements and try again.");
    } finally {
      setPending(false);
    }
  }

  return (
    <main className="min-h-screen bg-gray-50 p-4 font-outfit dark:bg-gray-900 sm:p-6">
      <div className="mx-auto flex min-h-[calc(100vh-2rem)] w-full max-w-md items-center justify-center">
        <section className="glassmorphism w-full rounded-[24px] border border-white/20 p-6 shadow-2xl sm:p-10">
          <h1 className="text-3xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">
            {ticket === null ? "Verify your email" : "Create your account"}
          </h1>
          {challenge === null ? (
            <div className="mt-5">
              <p role="alert">No active email verification was found.</p>
              <Link className="mt-3 inline-block font-semibold text-[#0066FF]" href="/register">Start again</Link>
            </div>
          ) : ticket === null ? (
            <form className="mt-6 flex flex-col gap-5" onSubmit={verify}>
              <p className="text-sm text-gray-600">Enter the six-digit code sent to {challenge.email}.</p>
              <label className="flex flex-col gap-2 text-sm font-semibold">
                Verification code
                <input
                  autoComplete="one-time-code"
                  autoFocus
                  className="glassmorphism min-h-[52px] rounded-xl px-4 text-center text-xl tracking-[0.4em]"
                  inputMode="numeric"
                  maxLength={6}
                  onChange={(event) => setCode(event.target.value.replace(/\D/g, ""))}
                  pattern="[0-9]{6}"
                  required
                  value={code}
                />
              </label>
              <button className="min-h-[54px] rounded-xl bg-[#1D1D1F] p-4 font-bold text-white disabled:opacity-60" disabled={pending || code.length !== 6} type="submit">
                {pending ? "Verifying…" : "Verify email"}
              </button>
            </form>
          ) : (
            <form className="mt-6 flex flex-col gap-5" onSubmit={createAccount}>
              <p className="rounded-xl border border-green-300 bg-green-50 p-3 text-sm text-green-900" role="status">Email verified. Now choose your account credentials.</p>
              <label className="flex flex-col gap-2 text-sm font-semibold">
                Username
                <input autoComplete="username" className="glassmorphism min-h-[52px] rounded-xl px-4" maxLength={32} minLength={3} onChange={(event) => setUsername(event.target.value)} pattern="[A-Za-z0-9][A-Za-z0-9._-]{1,30}[A-Za-z0-9]" required value={username} />
              </label>
              <label className="flex flex-col gap-2 text-sm font-semibold">
                Workspace ID
                <input aria-describedby="workspace-help" autoComplete="organization" className="glassmorphism min-h-[52px] rounded-xl px-4" maxLength={48} minLength={3} onChange={(event) => setOrganizationId(event.target.value)} pattern="[A-Za-z0-9](?:[A-Za-z0-9-]{1,46}[A-Za-z0-9])?" required value={organizationId} />
              </label>
              <p className="text-xs text-gray-600" id="workspace-help">Use this memorable ID when signing in with a password, for example alice-shop.</p>
              <label className="flex flex-col gap-2 text-sm font-semibold">
                Password
                <input aria-describedby="password-help" autoComplete="new-password" className="glassmorphism min-h-[52px] rounded-xl px-4" maxLength={128} minLength={12} onChange={(event) => setPassword(event.target.value)} required type="password" value={password} />
              </label>
              <p className="text-xs text-gray-600" id="password-help">Use 12–128 characters. Avoid your username, email, and commonly used passwords.</p>
              <button className="min-h-[54px] rounded-xl bg-[#1D1D1F] p-4 font-bold text-white disabled:opacity-60" disabled={pending} type="submit">
                {pending ? "Creating account…" : "Create account"}
              </button>
            </form>
          )}
          {error && <p className="mt-4 rounded-xl border border-red-300 bg-red-50 p-3 text-sm text-red-800" role="alert">{error}</p>}
        </section>
      </div>
    </main>
  );
}
