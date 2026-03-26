import { RefreshCw } from "lucide-react";

interface TopBarProps {
  title: string;
  subtitle?: string;
  onRefresh?: () => void;
}

export function TopBar({ title, subtitle, onRefresh }: TopBarProps) {
  return (
    <div className="h-14 flex items-center justify-between px-6 border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      {/* Title Section */}
      <div>
        <h2 className="text-lg font-semibold">{title}</h2>
        {subtitle && (
          <p className="text-xs text-muted-foreground">{subtitle}</p>
        )}
      </div>

      {/* Actions */}
      <div className="flex items-center gap-2">
        {onRefresh && (
          <button
            onClick={onRefresh}
            className="p-2 rounded-lg hover:bg-accent transition-all"
            title="Refresh"
          >
            <RefreshCw className="w-4 h-4" />
          </button>
        )}
      </div>
    </div>
  );
}
