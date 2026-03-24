import { ThemeProvider } from "@/components/theme-provider";
import { TitleBar } from "@/components/layout/TitleBar";
import { Sidebar } from "@/components/layout/Sidebar";
import { TopBar } from "@/components/layout/TopBar";
import { StatusBar } from "@/components/layout/StatusBar";
import { ContainersPage } from "@/pages/ContainersPage";
import { useAppStore, type Screen } from "@/store/app.store";
import { useContainersResponse } from "@/hooks/useContainers";

const DEFAULT_QUERY = { all: true } as const;

const SCREEN_TITLES: Record<Screen, { title: string; subtitle: string }> = {
  containers: { title: "Containers", subtitle: "Manage lifecycle, inspect status, and act on your workloads." },
  images: { title: "Images", subtitle: "Browse local images and clean up what you no longer need." },
  volumes: { title: "Volumes", subtitle: "Track durable storage attached to your environments." },
  builds: { title: "Builds", subtitle: "Review recent builds and prepare for the next phase." },
  dockerHub: { title: "Docker Hub", subtitle: "Registry integration is queued for a later phase." },
  scout: { title: "Docker Scout", subtitle: "Security and insight surfaces will land in a later phase." },
  kubernetes: { title: "Kubernetes", subtitle: "Cluster management is planned after the core Docker flows." },
  models: { title: "Models", subtitle: "Model runtimes and discovery will follow after container parity." },
  toolkit: { title: "MCP Toolkit", subtitle: "Toolkit integration is reserved for a later phase." },
  extensions: { title: "Extensions", subtitle: "Extension management will arrive in a future phase." },
  suggestions: { title: "Suggestions", subtitle: "Optimization recommendations will appear here." },
  settings: { title: "Settings", subtitle: "Preferences, theme controls, and daemon configuration." },
};

function PlaceholderScreen({ title, subtitle }: { title: string; subtitle: string }) {
  return (
    <div className="flex h-full items-center justify-center px-10">
      <div className="max-w-xl rounded-[28px] border border-white/8 bg-white/3 px-12 py-14 text-center shadow-[0_24px_80px_rgba(0,0,0,0.26)] backdrop-blur-xl">
        <p className="mb-3 text-[11px] font-semibold uppercase tracking-[0.28em] text-[rgb(var(--workspace-muted))]">
          Coming Next
        </p>
        <h2 className="text-[32px] font-semibold tracking-[-0.04em] text-[rgb(var(--workspace-foreground))]">
          {title}
        </h2>
        <p className="mt-3 text-[15px] leading-7 text-[rgb(var(--workspace-muted))]">
          {subtitle}
        </p>
      </div>
    </div>
  );
}

function AppContent() {
  const { screen } = useAppStore();
  const { data } = useContainersResponse(DEFAULT_QUERY);
  const meta = SCREEN_TITLES[screen];
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

  const containerSubtitle =
    screen === "containers"
      ? `${listResponse.overview.running} running · ${listResponse.total_count} total`
      : meta.subtitle;

  return (
    <div className="flex h-screen w-screen flex-col overflow-hidden bg-[rgb(var(--workspace-bg))] text-[rgb(var(--workspace-foreground))]">
      <TitleBar />

      <div className="flex min-h-0 flex-1">
        <Sidebar />

        <div className="flex min-w-0 flex-1 flex-col overflow-hidden bg-[rgb(var(--workspace-bg))]">
          <TopBar title={meta.title} subtitle={containerSubtitle} />

          <main className="min-h-0 flex-1 overflow-hidden">
            {screen === "containers" ? (
              <ContainersPage />
            ) : (
              <PlaceholderScreen title={meta.title} subtitle={meta.subtitle} />
            )}
          </main>
        </div>
      </div>

      <StatusBar />
    </div>
  );
}

export default function App() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="dockerlens-theme">
      <AppContent />
    </ThemeProvider>
  );
}
