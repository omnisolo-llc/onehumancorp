import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

export default function MemoryConsolidatorRoot() {
    const [metrics, setMetrics] = useState<any>(null);
    const [records, setRecords] = useState<any[]>([]);

    useEffect(() => {
        invoke('api_memory_get_metrics').then(setMetrics);
        invoke('api_memory_get_records').then(setRecords);
    }, []);

    return (
        <div className="p-8 text-white min-h-screen bg-neutral-900 font-sans">
            <h1 className="text-3xl font-bold mb-6 text-purple-400">🧠 Core Memory Consolidator</h1>

            {metrics && (
                <div className="grid grid-cols-4 gap-6 mb-8">
                    <div className="bg-neutral-800 p-6 rounded-xl border border-neutral-700 shadow-lg">
                        <div className="text-sm text-neutral-400">Total Vectors</div>
                        <div className="text-3xl font-mono mt-2">{metrics.total_records}</div>
                    </div>
                    <div className="bg-neutral-800 p-6 rounded-xl border border-orange-900 shadow-lg">
                        <div className="text-sm text-orange-400">Active Conflicts</div>
                        <div className="text-3xl font-mono mt-2 text-orange-300">{metrics.active_conflicts}</div>
                    </div>
                    <div className="bg-neutral-800 p-6 rounded-xl border border-neutral-700 shadow-lg">
                        <div className="text-sm text-neutral-400">Pending Prunes</div>
                        <div className="text-3xl font-mono mt-2">{metrics.pending_prunes}</div>
                    </div>
                    <div className="bg-neutral-800 p-6 rounded-xl border border-green-900 shadow-lg">
                        <div className="text-sm text-green-400">Resolved Anomalies</div>
                        <div className="text-3xl font-mono mt-2 text-green-300">{metrics.resolved_anomalies}</div>
                    </div>
                </div>
            )}

            <div className="bg-neutral-800 rounded-xl border border-neutral-700 overflow-hidden">
                <div className="p-4 border-b border-neutral-700 bg-neutral-800/50 flex justify-between items-center">
                    <h2 className="font-semibold text-lg">Cross-Department Context Stream</h2>
                    <button
                        onClick={() => invoke('api_memory_trigger_sync').then(() => alert('Sync Triggered'))}
                        className="px-4 py-2 bg-purple-600 hover:bg-purple-500 rounded-lg text-sm font-medium transition-colors"
                    >
                        Trigger Vector Consolidation
                    </button>
                </div>
                <table className="w-full text-left">
                    <thead className="bg-neutral-900/50 text-neutral-400 text-sm">
                        <tr>
                            <th className="p-4 font-medium">Context ID</th>
                            <th className="p-4 font-medium">Extracted Fact</th>
                            <th className="p-4 font-medium">Source Dept</th>
                            <th className="p-4 font-medium">Confidence</th>
                            <th className="p-4 font-medium">Timestamp</th>
                        </tr>
                    </thead>
                    <tbody className="divide-y divide-neutral-700">
                        {records.map(record => (
                            <tr key={record.id} className="hover:bg-neutral-700/30 transition-colors">
                                <td className="p-4 font-mono text-xs text-neutral-400">{record.id}</td>
                                <td className="p-4 text-sm">{record.context}</td>
                                <td className="p-4">
                                    <span className="px-2 py-1 bg-neutral-700 rounded text-xs">
                                        {record.department}
                                    </span>
                                </td>
                                <td className="p-4 text-sm font-mono">{(record.confidence * 100).toFixed(1)}%</td>
                                <td className="p-4 text-xs text-neutral-500">{new Date(record.timestamp).toLocaleString()}</td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    );
}
