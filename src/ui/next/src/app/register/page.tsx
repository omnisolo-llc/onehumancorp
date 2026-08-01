"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { type FormEvent, useEffect, useState } from "react";

type PublicSettings = Readonly<{
  registration_mode: "closed" | "open" | "invite_only";
  registration_available: boolean;
  email_verification_required: true;
}>;

const CHALLENGE_STORAGE_KEY = "ohc-registration-challenge";

export default function RegisterPage() {
  const router = useRouter();
  const [settings, setSettings] = useState<PublicSettings | null>(null);
  const [email, setEmail] = useState("");
  const [invitationToken, setInvitationToken] = useState("");
  const [pending, setPending] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let active = true;
    fetch("/api/v1/auth/public-settings", { cache: "no-store" })
      .then(async (response) => {
        if (!response.ok) throw new Error("settings unavailable");
        return response.json() as Promise<PublicSettings>;
      })
      .then((value) => {
        if (active) setSettings(value);
      })
      .catch(() => {
        if (active) setError("Registration settings are temporarily unavailable.");
      });
    return () => {
      active = false;
    };
  }, []);

  async function submit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (pending || !settings?.registration_available) return;
    setPending(true);
    setError(null);
    try {
      const response = await fetch("/api/v1/auth/registration/email/start", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({
          email,
          ...(settings.registration_mode === "invite_only"
            ? { invitation_token: invitationToken }
            : {}),
        }),
      });
      const body: unknown = await response.json();
      const challengeId =
        body !== null && typeof body === "object" && "challenge_id" in body
          ? (body as { challenge_id?: unknown }).challenge_id
          : undefined;
      if (!response.ok || typeof challengeId !== "string" || challengeId.length > 64) {
        throw new Error("verification unavailable");
      }
      sessionStorage.setItem(
        CHALLENGE_STORAGE_KEY,
        JSON.stringify({ challengeId, email: email.trim().toLowerCase() }),
      );
      router.push("/verify-email");
    } catch {
      setError("We could not send a verification code. Check your details and try again.");
    } finally {
      setPending(false);
    }
  }

  const closed = settings?.registration_mode === "closed";

  return (
    <main className="min-h-screen bg-gray-50 p-4 font-outfit dark:bg-gray-900 sm:p-6">
      <div className="mx-auto flex min-h-[calc(100vh-2rem)] w-full max-w-md items-center justify-center">
        <section className="glassmorphism w-full rounded-[24px] border border-white/20 p-6 shadow-2xl sm:p-10">
          <div className="mb-8 text-center">
            <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-2xl bg-[#0066FF] text-xl font-bold text-white">O</div>
            <h1 className="text-3xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">Create your account</h1>
            <p className="mt-2 text-sm text-gray-600 dark:text-gray-300">Verify your email before choosing a username or password.</p>
          </div>

          {settings === null && error === null && <p role="status">Checking registration availability…</p>}
          {closed && (
            <div className="rounded-xl border border-amber-300 bg-amber-50 p-4 text-sm text-amber-900" role="status">
              Registration is currently closed. An administrator can enable it in Settings.
            </div>
          )}
          {!closed && settings !== null && (
            <form className="flex flex-col gap-5" onSubmit={submit}>
              <label className="flex flex-col gap-2 text-sm font-semibold text-gray-700 dark:text-gray-200">
                Email address
                <input
                  autoComplete="email"
                  autoFocus
                  className="glassmorphism min-h-[52px] rounded-xl px-4 text-base"
                  disabled={pending || !settings.registration_available}
                  maxLength={254}
                  onChange={(event) => setEmail(event.target.value)}
                  required
                  type="email"
                  value={email}
                />
              </label>
              {settings.registration_mode === "invite_only" && (
                <label className="flex flex-col gap-2 text-sm font-semibold text-gray-700 dark:text-gray-200">
                  Invitation code
                  <input
                    className="glassmorphism min-h-[52px] rounded-xl px-4 text-base"
                    disabled={pending}
                    maxLength={128}
                    onChange={(event) => setInvitationToken(event.target.value)}
                    required
                    value={invitationToken}
                  />
                </label>
              )}
              <button className="min-h-[54px] rounded-xl bg-[#1D1D1F] p-4 font-bold text-white disabled:opacity-60" disabled={pending || !settings.registration_available} type="submit">
                {pending ? "Sending code…" : "Verify email"}
              </button>
            </form>
          )}
          {error && <p className="mt-4 rounded-xl border border-red-300 bg-red-50 p-3 text-sm text-red-800" role="alert">{error}</p>}
          <p className="mt-6 text-center text-sm text-gray-600 dark:text-gray-300">
            Already have an account? <Link className="font-semibold text-[#0066FF]" href="/login">Sign in</Link>
          </p>
        </section>
      </div>
    </main>
  );
}
