"use client";

import React, { useEffect, useState } from "react";
import { PipelineCard } from "./PipelineCard";

export interface Opportunity {
    id: string;
    tenant_id: string;
    lead_id?: string;
    title: string;
    stage: string;
    estimated_value: number;
    priority: string;
    created_at: string;
}

const STAGES = ["Qualified", "Proposal", "Negotiation", "Won", "Lost"];

export const PipelineBoard: React.FC = () => {
    const [opportunities, setOpportunities] = useState<Opportunity[]>([]);
    const [loading, setLoading] = useState(true);
    const tenantId = typeof window !== "undefined" ? localStorage.getItem("ohc_tenant_id") || "default" : "default";

    useEffect(() => {
        fetchOpportunities();
    }, []);

    const fetchOpportunities = async () => {
        try {
            const res = await fetch(`/api/v1/crm/opportunities/${tenantId}`);
            if (res.ok) {
                const data = await res.json();
                setOpportunities(data);
            }
        } catch (e) {
            console.error("Failed to fetch opportunities", e);
        } finally {
            setLoading(false);
        }
    };

    const handleDragStart = (e: React.DragEvent, id: string) => {
        e.dataTransfer.setData("opportunityId", id);
    };

    const handleDragOver = (e: React.DragEvent) => {
        e.preventDefault();
    };

    const handleDrop = async (e: React.DragEvent, stage: string) => {
        const id = e.dataTransfer.getData("opportunityId");
        if (!id) return;

        // Optimistic update
        setOpportunities(prev =>
            prev.map(opp => opp.id === id ? { ...opp, stage } : opp)
        );

        try {
            await fetch(`/api/v1/crm/opportunities/${tenantId}/${id}/stage`, {
                method: "PUT",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ stage }),
            });
        } catch (error) {
            console.error("Failed to update stage", error);
            fetchOpportunities(); // Revert on failure
        }
    };

    if (loading) {
        return <div className="text-gray-500">Loading Pipeline...</div>;
    }

    return (
        <div className="flex gap-6 min-w-max pb-4">
            {STAGES.map(stage => {
                const stageOpps = opportunities.filter(o => o.stage === stage);
                const totalValue = stageOpps.reduce((sum, o) => sum + o.estimated_value, 0);

                return (
                    <div
                        key={stage}
                        className="w-80 flex-shrink-0 bg-gray-50/50 rounded-xl p-4 border border-gray-200"
                        onDragOver={handleDragOver}
                        onDrop={(e) => handleDrop(e, stage)}
                    >
                        <div className="flex justify-between items-center mb-4">
                            <h3 className="font-semibold text-gray-700">{stage}</h3>
                            <span className="text-xs bg-gray-200 text-gray-600 px-2 py-1 rounded-full">
                                {stageOpps.length}
                            </span>
                        </div>
                        <div className="text-sm font-medium text-green-600 mb-4">
                            ${totalValue.toFixed(2)}
                        </div>

                        <div className="min-h-[200px]">
                            {stageOpps.map(opp => (
                                <PipelineCard
                                    key={opp.id}
                                    opportunity={opp}
                                    onDragStart={handleDragStart}
                                />
                            ))}
                            {stageOpps.length === 0 && (
                                <div className="text-gray-400 text-sm text-center italic mt-10">
                                    No opportunities
                                </div>
                            )}
                        </div>
                    </div>
                );
            })}
        </div>
    );
};
