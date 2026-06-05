"use client";
import React from "react";
import dynamic from "next/dynamic";
import "swagger-ui-react/swagger-ui.css";

// Dynamic import with SSR disabled to prevent hydration errors and type mismatches
const SwaggerUI = dynamic(() => import("swagger-ui-react").then((mod) => mod.default as any), { ssr: false });

export default function SwaggerUIWrapper({ spec }: { spec: any }) {
  return React.createElement(SwaggerUI as any, { spec });
}
