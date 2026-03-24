import { Activity, Cpu, Database, HardDrive, TerminalSquare } from "lucide-react";
import { useAppStore } from "@/store/app.store";
import { useContainersOverview } from "@/hooks/useContainers";
import { useTheme } from "@/components/theme-provider";

function formatBytes(value: number) {
  if (!value) {
    return "0 B";
  }

  const units = ["B", "KB", "MB", "GB", "TB"];
  const index = Math.min(Math.floor(Math.log(value) / Math.log(1024)), units.length - 1);
  const scaled = value / 1024 ** index;

  return `${scaled.toFixed(index === 0 ? 0 : 2)} ${units[index]}`;
}

export function StatusBar() {
  const { daemonRunning } = useAppStore();
  const { theme } = useTheme();
  const { data: overview } = useContainersOverview();

  const memoryUsage = overview?.total_memory_usage_bytes ?? 0;
  const memoryLimit = overview?.total_memory_limit_bytes ?? 0;

  return (
    <footer className="flex min-h-12 flex-wrap items-center justify-between gap-3 border-t border-white/10 bg-[rgb(var(--statusbar-bg))] px-4 py-3 text-[12px] text-[rgb(var(--statusbar-foreground))] sm:px-6">
      <div className="flex flex-wrap items-center gap-4 sm:gap-6">
        <div className="flex items-center gap-2 text-[rgb(var(--statusbar-accent))]">
          <Activity className="h-4 w-4" />
          <span className="font-semibold">
            {daemonRunning ? "Engine running" : "Engine unavailable"}
          </span>
        </div>

        <div className="flex items-center gap-2">
          <Cpu className="h-4 w-4 text-[rgb(var(--statusbar-muted))]" />
          <span>CPU {overview?.total_cpu_percent.toFixed(2) ?? "0.00"}%</span>
        </div>

        <div className="flex items-center gap-2">
          <Database className="h-4 w-4 text-[rgb(var(--statusbar-muted))]" />
          <span>
            RAM {formatBytes(memoryUsage)}
            {memoryLimit ? ` / ${formatBytes(memoryLimit)}` : ""}
          </span>
        </div>

        <div className="hidden items-center gap-2 md:flex">
          <HardDrive className="h-4 w-4 text-[rgb(var(--statusbar-muted))]" />
          <span>{overview?.total ?? 0} containers</span>
        </div>
      </div>

      <div className="flex items-center gap-4">
        <span className="uppercase tracking-[0.18em] text-[rgb(var(--statusbar-muted))]">
          {theme}
        </span>
        <div className="hidden items-center gap-2 text-[rgb(var(--statusbar-accent))] sm:flex">
          <TerminalSquare className="h-4 w-4" />
          <span className="font-medium">Terminal</span>
        </div>
      </div>
    </footer>
  );
}
