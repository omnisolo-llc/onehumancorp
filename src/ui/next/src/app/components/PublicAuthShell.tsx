import Link from "next/link";
import type { ReactNode } from "react";

export function PublicAuthShell({ children }: { children: ReactNode }) {
  return (
    <main
      className="min-h-screen bg-gray-50 px-4 py-8 font-outfit text-[#1D1D1F] dark:bg-gray-900 dark:text-[#F5F5F7] sm:px-6"
      data-auth-shell
    >
      <div className="mx-auto flex min-h-[calc(100vh-4rem)] w-full max-w-md flex-col justify-center gap-5">
        <Link
          aria-label="OHC Network sign in"
          className="mx-auto flex items-center gap-3 text-sm font-semibold text-gray-700 no-underline dark:text-gray-200"
          href="/login"
        >
          <span className="grid h-10 w-10 place-items-center rounded-lg bg-[#0066FF] text-base font-bold text-white shadow-sm">
            O
          </span>
          <span>OHC Network</span>
        </Link>
        <section className="auth-panel w-full rounded-lg border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800 sm:p-8">
          {children}
        </section>
      </div>
    </main>
  );
}
