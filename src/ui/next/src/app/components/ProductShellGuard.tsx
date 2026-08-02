"use client";

import { usePathname } from "next/navigation";
import { AppShell } from "./AppShell";
import { resolveShellRoute } from "./shellRoutes";

export function ProductShellGuard({ children }: { children: React.ReactNode }) {
  const pathname = usePathname();
  const config = resolveShellRoute(pathname);

  if (config.owner === "page") return <>{children}</>;

  return (
    <AppShell title={config.title} subtitle={config.subtitle}>
      {children}
    </AppShell>
  );
}
