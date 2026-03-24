import {
  Bot,
  Box,
  Boxes,
  Database,
  HardDrive,
  Image,
  Layers3,
  Network,
  PackageSearch,
  Puzzle,
  Settings,
  ShieldCheck,
  Wrench,
} from "lucide-react";
import { useAppStore, type Screen } from "@/store/app.store";
import { useContainersResponse } from "@/hooks/useContainers";
import { cn } from "@/lib/utils";

type NavItem = {
  id: Screen;
  label: string;
  icon: typeof Box;
  badge?: string;
};

const PRIMARY_ITEMS: NavItem[] = [
  { id: "containers", label: "Containers", icon: Box },
  { id: "images", label: "Images", icon: Image },
  { id: "volumes", label: "Volumes", icon: HardDrive },
  { id: "builds", label: "Builds", icon: Wrench },
  { id: "dockerHub", label: "Docker Hub", icon: PackageSearch },
  { id: "scout", label: "Docker Scout", icon: ShieldCheck },
  { id: "kubernetes", label: "Kubernetes", icon: Network },
  { id: "models", label: "Models", icon: Layers3 },
  { id: "toolkit", label: "MCP Toolkit", icon: Bot, badge: "Beta" },
];

const SECONDARY_ITEMS: NavItem[] = [
  { id: "extensions", label: "Extensions", icon: Puzzle },
  { id: "suggestions", label: "Suggestions", icon: Boxes },
  { id: "settings", label: "Settings", icon: Settings },
];

function SidebarItem({
  item,
  active,
  onClick,
}: {
  item: NavItem;
  active: boolean;
  onClick: () => void;
}) {
  const Icon = item.icon;

  return (
    <button
      type="button"
      onClick={onClick}
      className={cn(
        "group flex h-12 w-full items-center justify-center gap-4 rounded-2xl px-0 text-left text-[15px] font-medium transition xl:justify-start xl:px-5",
        active
          ? "bg-[rgb(var(--sidebar-active))] text-white shadow-[inset_0_0_0_1px_rgba(95,142,255,0.16)]"
          : "text-[rgb(var(--sidebar-foreground))] hover:bg-white/5 hover:text-white",
      )}
    >
      <Icon className={cn("h-5 w-5", active ? "text-[rgb(var(--sidebar-accent))]" : "text-[rgb(var(--sidebar-muted))]")} />
      <span className="hidden flex-1 truncate xl:block">{item.label}</span>
      {item.badge ? (
        <span className="hidden rounded-full bg-[rgb(var(--shell-blue-end))] px-2.5 py-1 text-[10px] font-bold uppercase tracking-[0.14em] text-white xl:inline-flex">
          {item.badge}
        </span>
      ) : null}
    </button>
  );
}

export function Sidebar() {
  const { screen, setScreen } = useAppStore();
  const { data } = useContainersResponse({ all: true });
  const listResponse = data ?? {
    items: [],
    overview: {
      total: 0,
      running: 0,
      paused: 0,
      exited: 0,
      total_cpu_percent: 0,
      total_memory_usage_bytes: 0,
      total_memory_limit_bytes: 0,
      generated_at_ms: 0,
    },
    total_count: 0,
    filtered_count: 0,
    generated_at_ms: 0,
  };

  return (
    <aside className="flex w-[88px] shrink-0 flex-col border-r border-white/8 bg-[rgb(var(--sidebar-bg))] px-3 py-5 text-white xl:w-[320px] xl:px-5 xl:py-7">
      <div className="mb-6 rounded-[24px] border border-white/6 bg-white/3 px-4 py-4 xl:flex xl:items-center xl:justify-between xl:px-5">
        <div className="hidden xl:block">
          <div className="text-[13px] font-semibold text-white/88">Ask Gordon</div>
          <div className="mt-1 text-[12px] text-[rgb(var(--sidebar-muted))]">
            AI-assisted workflow suggestions
          </div>
        </div>
        <span className="mt-3 inline-flex rounded-full bg-[rgb(var(--shell-blue-end))] px-3 py-1 text-[10px] font-bold uppercase tracking-[0.14em] text-white lg:mt-0">
          Beta
        </span>
      </div>

      <nav className="space-y-1.5">
        {PRIMARY_ITEMS.map((item) => (
          <SidebarItem
            key={item.id}
            item={item}
            active={screen === item.id}
            onClick={() => setScreen(item.id)}
          />
        ))}
      </nav>

      <div className="my-6 h-px bg-white/8" />

      <nav className="space-y-1.5">
        {SECONDARY_ITEMS.map((item) => (
          <SidebarItem
            key={item.id}
            item={item}
            active={screen === item.id}
            onClick={() => setScreen(item.id)}
          />
        ))}
      </nav>

      <div className="mt-auto rounded-[24px] border border-white/6 bg-white/4 p-4 lg:p-5">
        <div className="flex items-center gap-3">
          <div className="flex h-10 w-10 items-center justify-center rounded-2xl bg-[rgb(var(--sidebar-active))] text-[rgb(var(--sidebar-accent))]">
            <Database className="h-5 w-5" />
          </div>
          <div className="hidden xl:block">
            <div className="text-[14px] font-semibold text-white">Container inventory</div>
            <div className="mt-1 text-[12px] text-[rgb(var(--sidebar-muted))]">
              {listResponse.overview.running} running · {listResponse.total_count} total
            </div>
          </div>
        </div>
      </div>
    </aside>
  );
}
