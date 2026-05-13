import React from 'react';
import { WizardState } from './types';
export default function BrandColorsStep({ state, update, next, prev }: { state: WizardState, update: (k: keyof WizardState, v: any) => void, next: () => void, prev: () => void }) {
    return (
        <div className="glass-panel p-8 rounded-2xl border border-gray-700">
            <h2 className="text-3xl font-bold mb-8">Brand colors & logo</h2>
            <div className="grid grid-cols-5 gap-4 mb-8">
                <div onClick={() => update('colors', ['#FF0000', '#000000'])} className={`h-16 rounded cursor-pointer bg-gradient-to-r from-red-500 to-black ${state.colors.length ? 'ring-2 ring-white' : ''}`}></div>
                <div onClick={() => update('colors', ['#00FF00', '#FFFFFF'])} className={`h-16 rounded cursor-pointer bg-gradient-to-r from-green-500 to-white`}></div>
            </div>

            <div className="flex flex-col gap-4 mb-4">
                <input type="file" onChange={() => update('logo', 'uploaded')} className="text-white" />
                <button className="px-4 py-2 bg-purple-600/30 text-purple-300 rounded border border-purple-500 w-64 hover:bg-purple-600/50">✨ AI Background Removal</button>
                <button className="px-4 py-2 bg-purple-600/30 text-purple-300 rounded border border-purple-500 w-64 hover:bg-purple-600/50">✨ Generate Logo with AI</button>
            </div>

            <div className="flex justify-between mt-8">
                <button onClick={prev} className="px-6 py-2 border border-gray-600 rounded-lg">Back</button>
                <button onClick={next} className="px-6 py-2 bg-blue-500 rounded-lg">Next</button>
            </div>
        </div>
    );
}