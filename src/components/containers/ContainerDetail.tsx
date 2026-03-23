import { useState } from "react";
import { useContainersStore } from "@/store/containers.store";
import { useContainers, useStartContainer, useStopContainer, useRestartContainer, usePauseContainer, useUnpauseContainer, useRemoveContainer } from "@/hooks/useContainers";
import { OverviewTab } from "./OverviewTab";

export function ContainerDetail() {
    const { selectedId } = useContainersStore();
    const { data: containers = [] } = useContainers();
    const [activeTab, setActiveTab] = useState<"overview" | "logs" | "inspect" | "terminal">("overview");

    const container = containers.find((c) => c.id === selectedId);

    if (!selectedId || !container) {
        return (
            <div className="flex-1 flex items-center justify-center" style={{ background: "var(--surface)" }}>
                <div className="text-center">
                    <div className="text-[40px] opacity-20 mb-3">◫</div>
                    <div className="text-[13px]" style={{ color: "var(--text-muted)" }}>
                        Select a container to view details
                    </div>
                </div>
            </div>
        );
    }

    return (
        <div className="flex-1 flex flex-col overflow-hidden" style={{ background: "var(--surface)" }}>
            <DetailHeader container={container} />
            
            <div className="flex" style={{ borderBottom: "1px solid var(--border)" }}>
                <TabButton label="Overview" active={activeTab === "overview"} onClick={() => setActiveTab("overview")} />
                <TabButton label="Logs" active={activeTab === "logs"} onClick={() => setActiveTab("logs")} />
                <TabButton label="Inspect" active={activeTab === "inspect"} onClick={() => setActiveTab("inspect")} />
                <TabButton label="Terminal" active={activeTab === "terminal"} onClick={() => setActiveTab("terminal")} />
            </div>

            <div className="flex-1 overflow-auto">
                {activeTab === "overview" && <OverviewTab container={container} />}
                {activeTab === "logs" && <ComingSoon feature="Logs" phase="Phase 6" />}
                {activeTab === "inspect" && <ComingSoon feature="Inspect" phase="Phase 2" />}
                {activeTab === "terminal" && <ComingSoon feature="Terminal" phase="Phase 6" />}
            </div>
        </div>
    );
}

function DetailHeader({ container }: { container: any }) {
    const startMutation = useStartContainer();
    const stopMutation = useStopContainer();
    const restartMutation = useRestartContainer();
    const pauseMutation = usePauseContainer();
    const unpauseMutation = useUnpauseContainer();
    const removeMutation = useRemoveContainer();

    const isRunning = container.state.toLowerCase() === "running";
    const isPaused = container.state.toLowerCase() === "paused";
    const isStopped = container.state.toLowerCase() === "exited";

    const handleStart = () => startMutation.mutate(container.id);
    const handleStop = () => stopMutation.mutate(container.id);
    const handleRestart = () => restartMutation.mutate(container.id);
    const handlePause = () => pauseMutation.mutate(container.id);
    const handleUnpause = () => unpauseMutation.mutate(container.id);
    const handleRemove = () => {
        if (confirm(`Delete container "${container.name}"? This action cannot be undone.`)) {
            removeMutation.mutate(container.id);
        }
    };

    return (
        <div className="px-6 py-4" style={{ borderBottom: "1px solid var(--border)" }}>
            <div className="flex items-center justify-between mb-4">
                <div className="flex items-center gap-3">
                    <StatusDot state={container.state} />
                    <h2 className="text-[18px] font-bold" style={{ color: "var(--text-primary)" }}>
                        {container.name}
                    </h2>
                    <StateBadge state={container.state} />
                </div>
            </div>

            <div className="flex gap-2">
                {isRunning && (
                    <>
                        <ActionButton variant="danger" onClick={handleStop} loading={stopMutation.isPending}>
                            Stop
                        </ActionButton>
                        <ActionButton variant="warning" onClick={handlePause} loading={pauseMutation.isPending}>
                            Pause
                        </ActionButton>
                        <ActionButton variant="ghost" onClick={handleRestart} loading={restartMutation.isPending}>
                            Restart
                        </ActionButton>
                    </>
                )}

                {isStopped && (
                    <>
                        <ActionButton variant="success" onClick={handleStart} loading={startMutation.isPending}>
                            Start
                        </ActionButton>
                        <ActionButton variant="danger" onClick={handleRemove} loading={removeMutation.isPending}>
                            Remove
                        </ActionButton>
                    </>
                )}

                {isPaused && (
                    <>
                        <ActionButton variant="success" onClick={handleUnpause} loading={unpauseMutation.isPending}>
                            Unpause
                        </ActionButton>
                        <ActionButton variant="danger" onClick={handleStop} loading={stopMutation.isPending}>
                            Stop
                        </ActionButton>
                    </>
                )}
            </div>
        </div>
    );
}

function TabButton({ label, active, onClick }: { label: string; active: boolean; onClick: () => void }) {
    return (
        <button
            onClick={onClick}
            className="px-4 py-3 text-[12px] font-medium transition-all"
            style={{
                background: "none",
                border: "none",
                borderBottom: `2px solid ${active ? "var(--blue)" : "transparent"}`,
                color: active ? "var(--blue)" : "var(--text-muted)",
                fontWeight: active ? 600 : 500,
                cursor: "pointer",
                fontFamily: "inherit",
            }}
        >
            {label}
        </button>
    );
}

function ActionButton({ 
    variant, 
    onClick, 
    loading, 
    children 
}: { 
    variant: "primary" | "success" | "danger" | "warning" | "ghost"; 
    onClick: () => void; 
    loading?: boolean;
    children: React.ReactNode;
}) {
    const styles = {
        primary: { bg: "var(--blue)", color: "#fff", border: "var(--blue)" },
        success: { bg: "var(--green-dim)", color: "var(--green)", border: "rgba(34, 212, 122, 0.3)" },
        danger: { bg: "var(--red-dim)", color: "var(--red)", border: "rgba(240, 82, 82, 0.3)" },
        warning: { bg: "var(--yellow-dim)", color: "var(--yellow)", border: "rgba(245, 166, 35, 0.3)" },
        ghost: { bg: "transparent", color: "var(--text-secondary)", border: "var(--border)" },
    };

    const style = styles[variant];

    return (
        <button
            onClick={onClick}
            disabled={loading}
            className="px-4 py-2 text-[13px] font-semibold rounded-[7px] transition-all"
            style={{
                background: style.bg,
                color: style.color,
                border: `1px solid ${style.border}`,
                cursor: loading ? "not-allowed" : "pointer",
                opacity: loading ? 0.6 : 1,
                fontFamily: "inherit",
            }}
        >
            {loading ? "..." : children}
        </button>
    );
}

function StatusDot({ state }: { state: string }) {
    const colors: Record<string, string> = {
        running: "var(--green)",
        exited: "var(--red)",
        paused: "var(--yellow)",
    };

    const color = colors[state.toLowerCase()] || "var(--text-muted)";
    const isRunning = state.toLowerCase() === "running";

    return (
        <span
            className="w-[10px] h-[10px] rounded-full flex-shrink-0"
            style={{
                background: color,
                animation: isRunning ? "pulse-dot 2s infinite" : "none",
                boxShadow: isRunning ? `0 0 5px ${color}` : "none",
            }}
        />
    );
}

function StateBadge({ state }: { state: string }) {
    const styles: Record<string, { bg: string; color: string }> = {
        running: { bg: "var(--green-dim)", color: "var(--green)" },
        exited: { bg: "var(--red-dim)", color: "var(--red)" },
        paused: { bg: "var(--yellow-dim)", color: "var(--yellow)" },
    };

    const style = styles[state.toLowerCase()] || { bg: "var(--border)", color: "var(--text-muted)" };

    return (
        <span
            className="text-[10px] font-semibold px-2 py-[2px] rounded-[10px] uppercase"
            style={style}
        >
            {state}
        </span>
    );
}

function ComingSoon({ feature, phase }: { feature: string; phase: string }) {
    return (
        <div className="flex-1 flex items-center justify-center">
            <div className="text-center">
                <div className="text-[32px] opacity-20 mb-2">🔜</div>
                <div className="text-[13px] font-semibold mb-1" style={{ color: "var(--text-secondary)" }}>
                    {feature}
                </div>
                <div className="text-[11px]" style={{ color: "var(--text-muted)" }}>
                    Coming in {phase}
                </div>
            </div>
        </div>
    );
}
