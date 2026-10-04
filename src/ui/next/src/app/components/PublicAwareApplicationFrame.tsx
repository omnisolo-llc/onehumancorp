"use client";

import { usePathname } from "next/navigation";
import { isPublicPagePath } from "../../lib/auth/publicRoutes";
import { ProductShellGuard } from "./ProductShellGuard";

export function PublicAwareApplicationFrame({
  children,
  applicationWidgets,
}: {
  children: React.ReactNode;
  applicationWidgets: React.ReactNode;
}) {
  const pathname = usePathname();

  // The share preview keeps server metadata and a redirect/fallback link only.
  // Starting the shell here races its fetches and queue work against document
  // replacement. The destination mounts the normal application frame.
  if (pathname === "/share-card") return <>{children}</>;
  if (isPublicPagePath(pathname)) return <>{children}</>;

  return (
    <>
      <ProductShellGuard>{children}</ProductShellGuard>
      {applicationWidgets}
    </>
  );
}
