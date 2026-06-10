"use client";

import { useSearchParams } from "next/navigation";
import { Suspense } from "react";

function EmbedContent() {
  const searchParams = useSearchParams();
  return <div>Share and Save Embed</div>;
}

export default function ShareAndSaveEmbed() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <EmbedContent />
    </Suspense>
  );
}
