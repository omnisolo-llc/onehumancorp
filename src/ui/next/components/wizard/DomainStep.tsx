import React from 'react';
import { WizardState } from './types';
export default function DomainStep({ state, update, next, prev }: { state: WizardState, update: (k: keyof WizardState, v: string) => void, next: () => void, prev: () => void }) {
    const doms = ['Use a free OHC subdomain (mybusiness.ohc.app)', 'Use my own domain', 'Buy a domain'];
    return (
        <div className="glass-panel p-8 rounded-2xl border border-gray-700">
            <h2 className="text-3xl font-bold mb-8">Connect a domain</h2>
            <div className="flex flex-col gap-4 mb-8">
                {doms.map(d => (
                    <button key={d} onClick={() => update('domain', d)} className={`p-6 border rounded-xl text-left text-lg ${state.domain === d ? 'border-green-500 bg-green-500/10' : 'border-gray-600 hover:border-gray-400'}`}>
                        {d}
                    </button>
                ))}
            </div>
            <div className="flex justify-between">
                <button onClick={prev} className="px-6 py-2 border border-gray-600 rounded-lg">Back</button>
                <button onClick={next} disabled={!state.domain} className="px-6 py-2 bg-blue-500 rounded-lg disabled:opacity-50">Next</button>
            </div>
        </div>
    );
}