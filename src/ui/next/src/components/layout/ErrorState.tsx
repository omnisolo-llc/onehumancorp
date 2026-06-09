export function ErrorState({ title, message }: { title: string; message: string }) {
  return (
    <div className="p-4 bg-red-50 text-red-900 border border-red-200 rounded-md">
      <h3 className="font-bold">{title}</h3>
      <p>{message}</p>
    </div>
  );
}
