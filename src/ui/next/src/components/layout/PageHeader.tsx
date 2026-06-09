export function PageHeader({ title, description, subtitle }: { title: string; description?: string; subtitle?: string }) {
  return (
    <div className="mb-6">
      <h1 className="text-2xl font-bold">{title}</h1>
      {description && <p className="text-gray-500">{description}</p>}
      {subtitle && <p className="text-gray-500">{subtitle}</p>}
    </div>
  );
}
