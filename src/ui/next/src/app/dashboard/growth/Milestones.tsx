'use client';
import React, { useEffect, useState } from 'react';

export default function Milestones() {
    const [milestones, setMilestones] = useState([]);

    useEffect(() => {
        fetch('/api/v1/growth/milestones/check')
            .then(res => res.json())
            .then(data => setMilestones(data.milestones || []));
    }, []);

    return (
        <div className="bg-white shadow rounded-lg p-6">
            <h2 className="text-xl font-semibold mb-4">Success Milestones</h2>
            <ul className="space-y-3">
                {milestones.map((m: any) => (
                    <li key={m.id} className="flex items-center space-x-3 bg-gray-50 p-2 rounded">
                        <span className={m.reached ? 'text-yellow-500 text-xl' : 'text-gray-300 text-xl'}>
                            {m.reached ? 'M' : 'W'}
                        </span>
                        <div>
                            <p className="font-medium text-gray-800">{m.title}</p>
                            <p className="text-sm text-gray-500">{m.description}</p>
                        </div>
                    </li>
                ))}
            </ul>
        </div>
    );
}
