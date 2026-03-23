import { useAppStore } from "@/store/app.store";
import { useContainers } from "@/hooks/useContainers";
import { cn } from "@/lib/utils";
import { 
  Box, 
  Image, 
  Database, 
  Network, 
  Settings, 
  Lightbulb,
  Activity
} from "lucide-react";

// Navigation items following SOLID principles (Single Responsibility)
const NAV_ITEMS = [
  { id: "containers", label: "Containers", icon: Box },
  { id: "images", label: "Images", icon: Image },
  { id: "volumes", label: "Volumes", icon: Database },
  { id: "networks", label: "Networks", icon: Network },
] as const;

const BOTTOM_ITEMS = [
  { id: "suggestions", label: "Suggestions", icon: Lightbulb },
  { id: "settings", label: "Settings", icon: Settings },
] as const;

export function Sidebar() {
  const { screen, setScreen } = useAppStore();
  const { data: containers = [] } = useContainers();
  
  // Calculate stats (DRY: reusable logic)
  const runningCount = containers.filter(c => c.state.toLowerCase() === "running").length;

  return (
    <aside className="w-[240px] flex flex-col border-r bg-card/50 backdrop-blur-sm">
      {/* Logo & Status */}
      <div className="p-4 border-b">
        <div className="flex items-center gap-3 mb-3">
          <div className="w-8 h-8 rounded-lg docker-gradient flex items-center justify-center">
            <Activity className="w-5 h-5 text-white" />
          </div>
          <div>
            <h1 className="text-sm font-semibold">DockerLens</h1>
            <p className="text-xs text-muted-foreground">v0.1.0 · Linux</p>
          </div>
        </div>
        
        {/* Docker Status Badge */}
        <DockerStatusBadge running={runningCount} total={containers.length} />
      </div>

      {/* Main Navigation */}
      <nav className="flex-1 p-3 space-y-1">
        {NAV_ITEMS.map((item) => (
          <NavItem
            key={item.id}
            {...item}
            active={screen === item.id}
            onClick={() => setScreen(item.id)}
            {...(item.id === "containers" ? { count: containers.length } : {})}
          />
        ))}
      </nav>

      {/* Bottom Navigation */}
      <nav className="p-3 border-t space-y-1">
        {BOTTOM_ITEMS.map((item) => (
          <NavItem
            key={item.id}
            {...item}
            active={screen === item.id}
            onClick={() => setScreen(item.id)}
          />
        ))}
      </nav>
    </aside>
  );
}

// Extracted component following Single Responsibility Principle
function NavItem({ 
  icon: Icon, 
  label, 
  active, 
  onClick, 
  count 
}: { 
  icon: any; 
  label: string; 
  active: boolean; 
  onClick: () => void; 
  count?: number;
}) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-all",
        "hover:bg-accent",
        active && "bg-primary/10 text-primary hover:bg-primary/15"
      )}
    >
      <Icon className="w-4 h-4 flex-shrink-0" />
      <span className="flex-1 text-left">{label}</span>
      {count !== undefined && (
        <span className={cn(
          "text-xs px-2 py-0.5 rounded-full",
          active ? "bg-primary/20 text-primary" : "bg-muted text-muted-foreground"
        )}>
          {count}
        </span>
      )}
    </button>
  );
}

// Docker status badge component (DRY: reusable)
function DockerStatusBadge({ running, total }: { running: number; total: number }) {
  return (
    <div className="flex items-center gap-2 px-3 py-2 rounded-lg bg-muted/50">
      <div className={cn(
        "w-2 h-2 rounded-full",
        running > 0 ? "bg-green-500 pulse-dot" : "bg-muted-foreground"
      )} />
      <div className="flex-1">
        <p className="text-xs font-medium">
          {running > 0 ? "Docker Running" : "Docker Idle"}
        </p>
        <p className="text-[10px] text-muted-foreground">
          {running} running · {total} total
        </p>
      </div>
    </div>
  );
}
