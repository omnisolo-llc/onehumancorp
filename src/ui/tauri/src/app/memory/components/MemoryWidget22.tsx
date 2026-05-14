import React from 'react';

export interface MemoryWidget22Props {
    id: string;
    value: number;
}

export default function MemoryWidget22({ id, value }: MemoryWidget22Props) {
    return (
        <div className="widget-container bg-neutral-800 p-4 rounded-lg shadow-md border border-neutral-700 m-2">
            <h3 className="text-lg font-bold text-neutral-300">Widget 22 - {id}</h3>
            <div className="mt-2 text-2xl font-mono text-blue-400">{value.toFixed(2)}</div>
            <div className="mt-1 text-xs text-neutral-500">Cross-department semantic sync node</div>
            <div className="h-2 w-full bg-neutral-700 rounded mt-2 overflow-hidden">
                <div className="h-full bg-blue-500" style={{ width: `${Math.min(100, value * 10)}%` }}></div>
            </div>
            <button className="mt-3 px-3 py-1 bg-neutral-700 hover:bg-neutral-600 rounded text-xs w-full transition-colors">
                Inspect Context Node
            </button>
        </div>
    );
}
