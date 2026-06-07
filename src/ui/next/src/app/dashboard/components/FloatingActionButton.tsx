import Link from 'next/link';

export function FloatingActionButton() {
  return (
    <Link
      href="/offerings/new"
      className="fixed bottom-6 right-6 w-14 h-14 bg-blue-600 text-white rounded-full shadow-lg flex items-center justify-center text-3xl font-bold hover:bg-blue-700 transition-transform hover:scale-105 z-50 focus:outline-none focus:ring-4 focus:ring-blue-300"
      aria-label="Create new offering"
    >
      +
    </Link>
  );
}
