# React + TypeScript — Best Practices

> **Applies to:** `src/` — all React frontend code
> **Versions:** React 18 · TypeScript 5 · Vite 5
> **Last reviewed:** March 2026
> **References:** Vercel React Best Practices · Snyk Security Guide · Sitepoint TypeScript Guide

---

## Table of Contents

1. [TypeScript Configuration](#1-typescript-configuration)
2. [Type Safety](#2-type-safety)
3. [Component Architecture](#3-component-architecture)
4. [State Management](#4-state-management)
5. [Performance](#5-performance)
6. [Security](#6-security)
7. [Hooks](#7-hooks)
8. [Tauri Integration](#8-tauri-integration)
9. [Testing](#9-testing)
10. [Code Quality Checklist](#10-code-quality-checklist)

---

## 1. TypeScript Configuration

### `tsconfig.json` — strict mode is non-negotiable
```json
{
  "compilerOptions": {
    "target": "ES2020",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "moduleResolution": "bundler",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true,
    "skipLibCheck": true,
    "jsx": "react-jsx"
  }
}
```

Key settings explained:

| Option | Why |
|---|---|
| `"strict": true` | Enables `strictNullChecks`, `strictFunctionTypes`, `noImplicitAny` and more |
| `"noUnusedLocals"` | Catches dead code at compile time |
| `"noUncheckedIndexedAccess"` | Array/object access returns `T \| undefined` — forces null checks |
| `"exactOptionalPropertyTypes"` | Distinguishes `{ a?: string }` from `{ a: string \| undefined }` |

---

## 2. Type Safety

### Never use `any` — use `unknown` instead
```typescript
// ❌ any disables all type checking
function processPayload(data: any) {
    data.whatever(); // no error — unsafe
}

// ✅ unknown forces you to narrow the type before use
function processPayload(data: unknown) {
    if (typeof data === 'object' && data !== null && 'containers' in data) {
        // now TypeScript knows the shape
    }
}
```

### Define explicit types for all Tauri command responses
```typescript
// src/types/docker.ts

export interface ContainerSummary {
    id: string;
    names: string[] | null;
    image: string;
    status: 'running' | 'stopped' | 'paused' | 'restarting' | 'exited';
    cpu_percent: number;
    memory_mb: number;
    ports: PortBinding[];
    created: number;
}

export interface PortBinding {
    host_port: string;
    container_port: string;
    protocol: 'tcp' | 'udp';
}

export interface DockerVolume {
    name: string;
    driver: string;
    mountpoint: string;
    size_bytes: number | null;
    created_at: string;
    labels: Record<string, string>;
}
```

### Type Tauri invoke wrappers — never use raw `invoke`
```typescript
// src/lib/tauri.ts
import { invoke } from '@tauri-apps/api/core';
import type { ContainerSummary, DockerVolume, DockerNetwork } from '@/types/docker';

export const docker = {
    listContainers: (): Promise<ContainerSummary[]> =>
        invoke('list_containers'),

    startContainer: (id: string): Promise<void> =>
        invoke('start_container', { id }),

    stopContainer: (id: string): Promise<void> =>
        invoke('stop_container', { id }),

    deleteContainer: (id: string, force?: boolean): Promise<void> =>
        invoke('delete_container', { id, force: force ?? false }),

    listVolumes: (): Promise<DockerVolume[]> =>
        invoke('list_volumes'),

    listNetworks: (): Promise<DockerNetwork[]> =>
        invoke('list_networks'),

    startDaemon: (): Promise<void> =>
        invoke('start_docker_daemon'),

    stopDaemon: (): Promise<void> =>
        invoke('stop_docker_daemon'),

    detectSocket: (): Promise<string | null> =>
        invoke('detect_socket_path'),
};
```

### Avoid type assertions (`as`) — use type guards instead
```typescript
// ❌ Type assertion — bypasses type checking, hides bugs
const container = response as ContainerSummary;

// ✅ Type guard — runtime check that TypeScript understands
function isContainerSummary(value: unknown): value is ContainerSummary {
    return (
        typeof value === 'object' &&
        value !== null &&
        'id' in value &&
        'status' in value
    );
}

if (isContainerSummary(response)) {
    // TypeScript now knows this is ContainerSummary
}
```

---

## 3. Component Architecture

### One component per file — always
```
src/components/containers/
├── ContainerList.tsx       ← list of containers
├── ContainerDetail.tsx     ← detail panel
├── LogsTab.tsx             ← logs tab content
├── TerminalTab.tsx         ← terminal tab content
├── StatsTab.tsx            ← stats tab content
├── OverviewTab.tsx         ← overview tab content
└── InspectTab.tsx          ← inspect tab content
```

### Props interfaces — always explicit, never implicit
```typescript
// ❌ Inline object type — hard to reuse, hard to document
const ContainerRow = ({ id, name, status }: { id: string; name: string; status: string }) => {};

// ✅ Named interface — reusable, clear, documentable
interface ContainerRowProps {
    /** The container's short ID */
    id: string;
    /** Human-readable container name */
    name: string;
    /** Current container lifecycle status */
    status: 'running' | 'stopped' | 'paused';
    /** Called when the user clicks this row */
    onSelect: (id: string) => void;
}

const ContainerRow = ({ id, name, status, onSelect }: ContainerRowProps) => {};
```

### Children typing
```typescript
import type { ReactNode } from 'react';

interface CardProps {
    title: string;
    children: ReactNode;
    className?: string;
}
```

### Separate smart (data) from dumb (presentational) components
```
// Smart — fetches data, manages state
ContainersPage.tsx  → useContainers() hook → renders ContainerList

// Dumb — pure UI, receives props
ContainerList.tsx   → receives containers[], renders rows
ContainerRow.tsx    → receives one container, renders a row
```

---

## 4. State Management

### Zustand — keep stores small and focused
```typescript
// src/store/app.store.ts
import { create } from 'zustand';

interface AppStore {
    screen: Screen;
    daemonRunning: boolean;
    socketPath: string;
    setScreen: (screen: Screen) => void;
    setDaemonRunning: (running: boolean) => void;
}

export const useAppStore = create<AppStore>((set) => ({
    screen: 'dashboard',
    daemonRunning: false,
    socketPath: '/var/run/docker.sock',
    setScreen: (screen) => set({ screen }),
    setDaemonRunning: (daemonRunning) => set({ daemonRunning }),
}));
```

### Use selectors to prevent unnecessary re-renders
```typescript
// ❌ Subscribes to entire store — re-renders on any change
const store = useAppStore();

// ✅ Subscribes only to what this component needs
const screen = useAppStore((state) => state.screen);
const daemonRunning = useAppStore((state) => state.daemonRunning);
```

### TanStack Query for all Docker API data
```typescript
// src/hooks/useContainers.ts
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { docker } from '@/lib/tauri';

export function useContainers() {
    return useQuery({
        queryKey: ['containers'],
        queryFn: docker.listContainers,
        refetchInterval: 5000,   // refresh every 5 seconds
        staleTime: 2000,         // consider fresh for 2 seconds
    });
}

export function useStartContainer() {
    const queryClient = useQueryClient();
    return useMutation({
        mutationFn: docker.startContainer,
        onSuccess: () => {
            // Invalidate container list — triggers a refetch
            queryClient.invalidateQueries({ queryKey: ['containers'] });
        },
    });
}
```

---

## 5. Performance

### Memo only when profiling shows it's needed
```typescript
// ❌ Premature memoisation adds complexity without guaranteed benefit
const ExpensiveComponent = React.memo(({ data }) => {
    return <div>{data.name}</div>; // trivial — memo not needed
});

// ✅ Memo for genuinely expensive renders
const ContainerStatsGraph = React.memo(({ stats }: { stats: StatsHistory[] }) => {
    // Recharts re-renders are expensive — memoising here is justified
    return <LineChart data={stats} />;
});
```

### `useCallback` for stable function references passed to children
```typescript
// ✅ Stable reference — ContainerRow won't re-render unless containers change
const handleSelect = useCallback((id: string) => {
    setSelectedId(id);
}, []); // empty deps = stable reference
```

### Lazy-load heavy components
```typescript
// ✅ Split xterm.js into its own chunk — loaded only when terminal tab opens
const TerminalTab = lazy(() => import('./TerminalTab'));

// In render:
<Suspense fallback={<div>Loading terminal…</div>}>
    <TerminalTab containerId={selectedId} />
</Suspense>
```

### Avoid cascading `useEffect` calls
```typescript
// ❌ Cascade — first effect sets state, second effect reacts to it
useEffect(() => {
    setContainers(data);
}, [data]);

useEffect(() => {
    updateBadgeCount(containers.length);
}, [containers]);

// ✅ Compute derived state in one pass
const runningCount = useMemo(
    () => containers.filter(c => c.status === 'running').length,
    [containers]
);
```

---

## 6. Security

### Never use `dangerouslySetInnerHTML` with Docker output

All data from Docker (container names, log lines, image names, environment variables) is untrusted user-controlled content. Never inject it raw into the DOM.
```typescript
// ❌ XSS risk
<div dangerouslySetInnerHTML={{ __html: container.name }} />

// ✅ React escapes text automatically
<div>{container.name}</div>

// ✅ For log lines with ANSI codes — strip them first
import stripAnsi from 'strip-ansi';
<div>{stripAnsi(logLine)}</div>
```

### Validate inputs before calling Tauri commands
```typescript
// ✅ Validate container ID before invoke
const handleStart = async (id: string) => {
    if (!id || !/^[a-zA-Z0-9_-]{1,64}$/.test(id)) {
        toast.error('Invalid container ID');
        return;
    }
    await docker.startContainer(id);
};
```

### Never expose Supabase service key in frontend code
```typescript
// ❌ Service key bypasses RLS — NEVER in frontend
const supabase = createClient(url, serviceKey); // bypasses all security

// ✅ Only the anon key goes in the frontend
const supabase = createClient(
    import.meta.env.VITE_SUPABASE_URL,
    import.meta.env.VITE_SUPABASE_ANON_KEY  // anon key only
);
```

### Sanitise URLs before opening in browser
```typescript
// ✅ Validate URL before calling Tauri's shell:open
import { open } from '@tauri-apps/plugin-shell';

const openContainerPort = async (hostPort: string) => {
    const url = `http://localhost:${hostPort}`;
    // Only open localhost URLs
    if (!url.startsWith('http://localhost:')) {
        console.error('Refusing to open non-localhost URL:', url);
        return;
    }
    await open(url);
};
```

---

## 7. Hooks

### Naming — custom hooks always start with `use`
```typescript
// src/hooks/useDockerStats.ts
export function useDockerStats(containerId: string) {
    // ...
}

// src/hooks/useTauriEvent.ts
export function useTauriEvent<T>(event: string, handler: (payload: T) => void) {
    useEffect(() => {
        const unlisten = listen<T>(event, (e) => handler(e.payload));
        return () => { unlisten.then(fn => fn()); };
    }, [event, handler]);
}
```

### Clean up Tauri event listeners
```typescript
// ✅ Always return the unlisten cleanup function
useEffect(() => {
    let unlisten: (() => void) | undefined;

    listen<DaemonStateEvent>('daemon_state', (event) => {
        setDaemonRunning(event.payload.running);
    }).then(fn => { unlisten = fn; });

    return () => {
        unlisten?.();  // ✅ Clean up on unmount
    };
}, []);
```

### Avoid `useEffect` for derived state
```typescript
// ❌ Unnecessary useEffect for computed value
const [runningCount, setRunningCount] = useState(0);
useEffect(() => {
    setRunningCount(containers.filter(c => c.status === 'running').length);
}, [containers]);

// ✅ Compute directly
const runningCount = containers.filter(c => c.status === 'running').length;
// Or with memo for expensive calculations:
const runningCount = useMemo(
    () => containers.filter(c => c.status === 'running').length,
    [containers]
);
```

---

## 8. Tauri Integration

### Always handle errors from `invoke`
```typescript
// ❌ Unhandled rejection
docker.startContainer(id);

// ✅ Handle success and error
try {
    await docker.startContainer(id);
    toast.success('Container started');
} catch (error) {
    toast.error(`Failed to start: ${error}`);
    console.error('start_container error:', error);
}
```

### Type Tauri events
```typescript
// src/types/events.ts
export interface LogLineEvent {
    container_id: string;
    line: string;
    timestamp: number;
    level: 'info' | 'warn' | 'error' | 'debug';
}

export interface DaemonStateEvent {
    running: boolean;
    socket_path: string | null;
}

export interface StatsUpdateEvent {
    container_id: string;
    cpu_percent: number;
    memory_mb: number;
    network_rx_bytes: number;
    network_tx_bytes: number;
}
```
```typescript
// src/hooks/useTauriEvents.ts
import { listen } from '@tauri-apps/api/event';
import type { LogLineEvent, DaemonStateEvent } from '@/types/events';

export function useLogStream(containerId: string, onLine: (event: LogLineEvent) => void) {
    useEffect(() => {
        const unlisten = listen<LogLineEvent>('log_line', (e) => {
            if (e.payload.container_id === containerId) {
                onLine(e.payload);
            }
        });
        return () => { unlisten.then(fn => fn()); };
    }, [containerId, onLine]);
}
```

---

## 9. Testing

### Test with Vitest + React Testing Library
```typescript
// src/components/containers/__tests__/ContainerRow.test.tsx
import { render, screen, fireEvent } from '@testing-library/react';
import { ContainerRow } from '../ContainerRow';

describe('ContainerRow', () => {
    const mockContainer = {
        id: 'abc123',
        name: 'nginx-proxy',
        status: 'running' as const,
        cpu: 0.4,
        mem: 128,
    };

    it('renders container name', () => {
        render(<ContainerRow container={mockContainer} onSelect={vi.fn()} />);
        expect(screen.getByText('nginx-proxy')).toBeInTheDocument();
    });

    it('calls onSelect with container id when clicked', () => {
        const onSelect = vi.fn();
        render(<ContainerRow container={mockContainer} onSelect={onSelect} />);
        fireEvent.click(screen.getByRole('button'));
        expect(onSelect).toHaveBeenCalledWith('abc123');
    });

    it('shows running badge for running containers', () => {
        render(<ContainerRow container={mockContainer} onSelect={vi.fn()} />);
        expect(screen.getByText('running')).toBeInTheDocument();
    });
});
```

### Mock Tauri invoke in tests
```typescript
// src/test/setup.ts
import { vi } from 'vitest';

// Mock Tauri IPC for tests
vi.mock('@tauri-apps/api/core', () => ({
    invoke: vi.fn(),
}));

vi.mock('@tauri-apps/api/event', () => ({
    listen: vi.fn(() => Promise.resolve(() => {})),
    emit: vi.fn(),
}));
```

---

## 10. Code Quality Checklist

### ESLint configuration
```json
// .eslintrc.json
{
  "extends": [
    "eslint:recommended",
    "plugin:@typescript-eslint/strict",
    "plugin:react-hooks/recommended"
  ],
  "rules": {
    "@typescript-eslint/no-explicit-any": "error",
    "@typescript-eslint/no-non-null-assertion": "error",
    "react-hooks/exhaustive-deps": "error",
    "no-console": ["warn", { "allow": ["warn", "error"] }]
  }
}
```

### Pre-commit checklist

- [ ] `pnpm tsc --noEmit` — zero TypeScript errors
- [ ] `pnpm lint` — zero ESLint errors
- [ ] `pnpm test` — all tests pass
- [ ] No `any` types introduced
- [ ] No `dangerouslySetInnerHTML` with Docker data
- [ ] All Tauri invoke calls have error handling
- [ ] Event listeners have cleanup functions
- [ ] New components have prop interfaces defined