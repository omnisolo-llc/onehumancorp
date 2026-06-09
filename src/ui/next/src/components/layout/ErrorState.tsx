import React from 'react';

export function ErrorState({ title, message }: { title: string; message: string }) {
  return (
    <div className="rounded-md bg-red-50 dark:bg-red-900/20 p-4 my-4 border border-red-200 dark:border-red-800">
      <div className="flex">
        <div className="ml-3">
          <h3 className="text-sm font-medium text-red-800 dark:text-red-200">{title}</h3>
          <div className="mt-2 text-sm text-red-700 dark:text-red-300">
            <p>{message}</p>
          </div>
        </div>
      </div>
    </div>
  );
}
