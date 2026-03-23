import type { ContainerSummary } from "@/types/docker";

export function OverviewTab({ container }: { container: ContainerSummary }) {
    return (
        <div className="p-6 space-y-6">
            {/* Image */}
            <Section title="Image">
                <InfoRow label="Name" value={container.image} mono />
                <InfoRow label="ID" value={container.id} mono />
            </Section>

            {/* Status */}
            <Section title="Status">
                <InfoRow label="State" value={container.state} />
                <InfoRow label="Status" value={container.status} />
                <InfoRow label="Created" value={formatTimestamp(container.created)} />
            </Section>

            {/* Ports */}
            <Section title="Ports">
                {container.ports.length === 0 ? (
                    <div className="text-[12px]" style={{ color: "var(--text-muted)" }}>
                        No ports exposed
                    </div>
                ) : (
                    <div className="space-y-2">
                        {container.ports.map((port, i) => (
                            <div
                                key={i}
                                className="flex items-center gap-3 px-3 py-2 rounded-[7px]"
                                style={{ background: "var(--bg)", border: "1px solid var(--border)" }}
                            >
                                <span className="mono text-[12px]" style={{ color: "var(--text-primary)" }}>
                                    {port.host_port || "—"}
                                </span>
                                <span style={{ color: "var(--text-muted)" }}>→</span>
                                <span className="mono text-[12px]" style={{ color: "var(--text-primary)" }}>
                                    {port.container_port}
                                </span>
                                <span
                                    className="text-[10px] font-semibold px-2 py-[2px] rounded-[6px] uppercase ml-auto"
                                    style={{ background: "var(--blue-dim)", color: "var(--blue)" }}
                                >
                                    {port.protocol}
                                </span>
                            </div>
                        ))}
                    </div>
                )}
            </Section>
        </div>
    );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
    return (
        <div>
            <h3
                className="text-[10px] font-semibold uppercase mb-3"
                style={{ color: "var(--text-muted)", letterSpacing: "0.5px" }}
            >
                {title}
            </h3>
            <div className="space-y-2">{children}</div>
        </div>
    );
}

function InfoRow({ label, value, mono }: { label: string; value: string; mono?: boolean }) {
    return (
        <div className="flex items-center justify-between py-2">
            <span className="text-[12px] font-medium" style={{ color: "var(--text-secondary)" }}>
                {label}
            </span>
            <span
                className={`text-[12px] ${mono ? "mono" : ""}`}
                style={{ color: "var(--text-primary)" }}
            >
                {value}
            </span>
        </div>
    );
}

function formatTimestamp(unix: number): string {
    const date = new Date(unix * 1000);
    const now = Date.now();
    const diff = now - date.getTime();

    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);

    if (days > 0) return `${days} day${days > 1 ? "s" : ""} ago`;
    if (hours > 0) return `${hours} hour${hours > 1 ? "s" : ""} ago`;
    if (minutes > 0) return `${minutes} minute${minutes > 1 ? "s" : ""} ago`;
    return "Just now";
}
