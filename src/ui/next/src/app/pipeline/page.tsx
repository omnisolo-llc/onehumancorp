"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";
import { PipelineBoard } from "./PipelineBoard";

export default function PipelinePage() {
    return (
        <AppShell title="Pipeline" subtitle="Manage leads and opportunities across stages">
            <div className="p-4 overflow-x-auto">
                <PipelineBoard />
            </div>
        </AppShell>
    );
}
