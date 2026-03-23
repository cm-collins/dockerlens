import { ThemeProvider } from "@/components/theme-provider";
import { useAppStore } from "@/store/app.store";
import { TitleBar } from "@/components/layout/TitleBar";
import { TopBar } from "@/components/layout/TopBar";
import { Sidebar } from "@/components/layout/Sidebar";
import { ContainersPage } from "@/pages/ContainersPage";
import { useContainers } from "@/hooks/useContainers";

const SCREEN_TITLES: Record<string, { title: string; subtitle: string }> = {
  containers: { title: "Containers", subtitle: "" },
  images: { title: "Images", subtitle: "Coming in Phase 3" },
  volumes: { title: "Volumes", subtitle: "Coming in Phase 4" },
  networks: { title: "Networks", subtitle: "Coming in Phase 4" },
  suggestions: { title: "Suggestions", subtitle: "Optimization recommendations" },
  settings: { title: "Settings", subtitle: "Preferences & daemon control" },
};

function AppContent() {
  const { screen } = useAppStore();
  const { data: containers = [], refetch } = useContainers();
  
  const running = containers.filter((c) => c.state.toLowerCase() === "running").length;
  const meta = SCREEN_TITLES[screen] ?? { title: screen, subtitle: "" };

  const subtitle =
    screen === "containers"
      ? `${running} running · ${containers.length} total`
      : meta.subtitle;

  // Wrap refetch to match expected signature
  const handleRefresh = screen === "containers" ? () => { refetch(); } : undefined;
  const refreshProps = handleRefresh ? { onRefresh: handleRefresh } : {};

  return (
    <div className="w-screen h-screen flex flex-col overflow-hidden fade-in">
      <TitleBar title={`DockerLens — ${meta.title}`} />

      <div className="flex flex-1 overflow-hidden">
        <Sidebar />

        <div className="flex flex-col flex-1 overflow-hidden">
          <TopBar 
            title={meta.title} 
            subtitle={subtitle}
            {...refreshProps}
          />

          <main className="flex-1 overflow-hidden">
            {screen === "containers" && <ContainersPage />}

            {screen !== "containers" && (
              <div className="h-full flex items-center justify-center">
                <div className="text-center space-y-3">
                  <div className="text-5xl opacity-20">🔜</div>
                  <h3 className="text-lg font-semibold">{meta.title}</h3>
                  <p className="text-sm text-muted-foreground">{meta.subtitle}</p>
                </div>
              </div>
            )}
          </main>
        </div>
      </div>
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
