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

  if (isPublicPagePath(pathname)) return <>{children}</>;

  return (
    <>
      <ProductShellGuard>{children}</ProductShellGuard>
      {applicationWidgets}
    </>
  );
}
