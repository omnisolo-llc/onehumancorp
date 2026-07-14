"use client";

import { useRouter } from "next/navigation";
import { useState } from "react";

export function LogoutButton() {
  const router = useRouter();
  const [pending, setPending] = useState(false);
  const [error, setError] = useState(false);

  async function logout() {
    if (pending) return;
    setPending(true);
    setError(false);
    try {
      const response = await fetch("/api/auth/logout", { method: "POST" });
      if (!response.ok) throw new Error("logout failed");
      router.replace("/login");
      router.refresh();
    } catch {
      setError(true);
      setPending(false);
    }
  }

  return (
    <div className="relative">
      <button
        className="app-button min-h-[44px]"
        disabled={pending}
        onClick={logout}
        type="button"
      >
        {pending ? "Logging out…" : "Log out"}
      </button>
      {error && (
        <span
          className="absolute right-0 top-full z-20 mt-2 w-56 rounded-lg border border-red-300 bg-red-50 p-2 text-xs text-red-800 shadow-lg dark:border-red-900 dark:bg-red-950 dark:text-red-200"
          role="alert"
        >
          Logout failed. Please try again.
        </span>
      )}
    </div>
  );
}
