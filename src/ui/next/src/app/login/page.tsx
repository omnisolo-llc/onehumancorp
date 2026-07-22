"use client";

import Link from "next/link";
import { useRouter, useSearchParams } from "next/navigation";
import { Suspense, type FormEvent, useState } from "react";
import { safeReturnPath } from "@/lib/auth/url";

const GENERIC_ERROR = "We couldn't sign you in. Check your details and try again.";

function LoginForm() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const [identifier, setIdentifier] = useState("");
  const [password, setPassword] = useState("");
  const [organization, setOrganization] = useState("");
  const [pending, setPending] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function submit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (pending) return;
    setPending(true);
    setError(null);
    const next = safeReturnPath(searchParams.get("next"));
    try {
      const response = await fetch(`/api/v1/auth/login?next=${encodeURIComponent(next)}`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({
          username: identifier,
          password,
          ...(organization.trim() === "" ? {} : { organization_id: organization }),
        }),
      });
      const body = await response.json();
      if (!response.ok || typeof body?.next !== "string") throw new Error("login denied");
      router.replace(safeReturnPath(body.next));
    } catch {
      setError(GENERIC_ERROR);
    } finally {
      setPending(false);
    }
  }

  return (
    <main className="min-h-screen bg-gray-50 p-4 font-outfit dark:bg-gray-900 sm:p-6">
      <div className="mx-auto flex min-h-[calc(100vh-2rem)] w-full max-w-md items-center justify-center sm:min-h-[calc(100vh-3rem)]">
        <section
          aria-labelledby="login-title"
          className="glassmorphism w-full rounded-[24px] border border-white/20 p-6 shadow-2xl sm:p-10"
        >
          <div className="mb-8 text-center">
            <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-2xl bg-[#0066FF] text-xl font-bold text-white shadow-lg">
              O
            </div>
            <h1 id="login-title" className="text-3xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">
              Sign in to OHC
            </h1>
            <p className="mt-2 text-sm text-gray-600 dark:text-gray-300">
              Access your business workspace securely.
            </p>
          </div>

          <form className="flex flex-col gap-5" onSubmit={submit}>
            <label className="flex flex-col gap-2 text-sm font-semibold text-gray-700 dark:text-gray-200">
              Email or username
              <input
                autoComplete="username"
                autoFocus
                className="glassmorphism min-h-[52px] w-full rounded-xl px-4 text-base text-[#1D1D1F] outline-none transition focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 dark:text-[#F5F5F7]"
                disabled={pending}
                maxLength={254}
                onChange={(event) => setIdentifier(event.target.value)}
                required
                value={identifier}
              />
            </label>

            <label className="flex flex-col gap-2 text-sm font-semibold text-gray-700 dark:text-gray-200">
              Password
              <input
                autoComplete="current-password"
                className="glassmorphism min-h-[52px] w-full rounded-xl px-4 text-base text-[#1D1D1F] outline-none transition focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 dark:text-[#F5F5F7]"
                disabled={pending}
                maxLength={1024}
                onChange={(event) => setPassword(event.target.value)}
                required
                type="password"
                value={password}
              />
            </label>

            <label className="flex flex-col gap-2 text-sm font-semibold text-gray-700 dark:text-gray-200">
              Organization <span className="font-normal text-gray-500">(optional for standalone)</span>
              <input
                autoComplete="organization"
                className="glassmorphism min-h-[52px] w-full rounded-xl px-4 text-base text-[#1D1D1F] outline-none transition focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 dark:text-[#F5F5F7]"
                disabled={pending}
                maxLength={128}
                onChange={(event) => setOrganization(event.target.value)}
                value={organization}
              />
            </label>

            <div aria-live="polite" className="min-h-6">
              {error && (
                <p role="alert" className="rounded-xl border border-red-300 bg-red-50 px-4 py-3 text-sm text-red-800 dark:border-red-900 dark:bg-red-950/40 dark:text-red-200">
                  {error}
                </p>
              )}
            </div>

            <button
              className="min-h-[54px] w-full rounded-xl bg-[#1D1D1F] p-4 font-bold text-white shadow-[0_4px_14px_0_rgba(0,0,0,0.3)] transition hover:bg-black active:scale-[0.99] disabled:cursor-wait disabled:opacity-70 dark:bg-white dark:text-[#1D1D1F] dark:hover:bg-gray-200"
              disabled={pending}
              type="submit"
            >
              {pending ? "Signing in…" : "Log in"}
            </button>
          </form>

          <div className="min-h-6" />
        </section>
      </div>
    </main>
  );
}

export default function Login() {
  return (
    <Suspense
      fallback={
        <main className="flex min-h-screen items-center justify-center bg-gray-50 p-6 font-outfit dark:bg-gray-900">
          <p className="text-sm text-gray-600 dark:text-gray-300" role="status">
            Loading sign in…
          </p>
        </main>
      }
    >
      <LoginForm />
    </Suspense>
  );
}
