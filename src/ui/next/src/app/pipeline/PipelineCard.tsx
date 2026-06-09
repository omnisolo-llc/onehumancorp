"use client";

import React from "react";
import { Opportunity } from "./PipelineBoard";

interface PipelineCardProps {
    opportunity: Opportunity;
    onDragStart: (e: React.DragEvent, id: string) => void;
}

export const PipelineCard: React.FC<PipelineCardProps> = ({ opportunity, onDragStart }) => {
    return (
        <div
            draggable
            onDragStart={(e) => onDragStart(e, opportunity.id)}
            className="app-card cursor-move mb-4 hover:shadow-lg transition-shadow duration-200 border-l-4 border-l-blue-500"
            style={{
                background: 'rgba(255, 255, 255, 0.7)',
                backdropFilter: 'blur(10px)',
                WebkitBackdropFilter: 'blur(10px)'
            }}
        >
            <div className="font-semibold text-gray-800 truncate mb-1">{opportunity.title}</div>
            <div className="text-sm text-gray-500 flex justify-between items-center">
                <span>{opportunity.priority} Priority</span>
                <span className="font-medium text-green-600">${opportunity.estimated_value.toFixed(2)}</span>
            </div>
            <div className="text-xs text-gray-400 mt-2 flex justify-between">
                <span>ID: {opportunity.id.substring(0, 8)}</span>
            </div>
        </div>
    );
};
