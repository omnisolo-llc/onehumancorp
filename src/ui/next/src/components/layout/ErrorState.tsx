import React from 'react';

export function ErrorState({ title, message, action }: { title: string, message?: string, action?: React.ReactNode }) {
    return (
        <div className="flex flex-col items-center justify-center py-12 text-center">
            <h3 className="text-lg font-medium text-gray-900 dark:text-white">{title}</h3>
            {message && <p className="mt-2 text-sm text-gray-500 max-w-sm">{message}</p>}
            {action && <div className="mt-6">{action}</div>}
        </div>
    );
}
