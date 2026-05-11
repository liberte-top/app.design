<script lang="ts">
  import {
    Breadcrumb,
    Inline,
    Mark,
    NavGroup,
    NavItem,
    PageHeader,
    Pill,
    ProLayout,
    Stack,
    Text,
    ThemeToggle,
    Wordmark,
    applyTheme,
    theme,
    toggleTheme,
    type BreadcrumbItem,
  } from "@liberte/svelte-components";
  import { onMount } from "svelte";
  import { config } from "./lib/config";

  import Buttons from "./sections/Buttons.svelte";
  import Inputs from "./sections/Inputs.svelte";
  import Pills from "./sections/Pills.svelte";
  import Feedback from "./sections/Feedback.svelte";
  import Identity from "./sections/Identity.svelte";
  import Layout from "./sections/Layout.svelte";
  import Typography from "./sections/Typography.svelte";
  import Data from "./sections/Data.svelte";
  import Composition from "./sections/Composition.svelte";
  import Tokens from "./sections/Tokens.svelte";

  type Route = {
    /** Path fragment without leading '#/'. Use empty string for index. */
    path: string;
    label: string;
    group: string;
    Component: typeof Buttons;
    summary: string;
  };

  const routes: Route[] = [
    {
      path: "atoms/buttons",
      label: "Buttons",
      group: "Atoms",
      Component: Buttons,
      summary: "Button, LinkButton, IconButton — every variant, size, and the in-flight Spinner combination.",
    },
    {
      path: "atoms/inputs",
      label: "Inputs",
      group: "Atoms",
      Component: Inputs,
      summary: "Field and Input — labelled controls with required / optional / hint / error / disabled states.",
    },
    {
      path: "atoms/pills",
      label: "Pills",
      group: "Atoms",
      Component: Pills,
      summary: "Pill — tone × variant × size, the inline status indicator.",
    },
    {
      path: "atoms/feedback",
      label: "Feedback",
      group: "Atoms",
      Component: Feedback,
      summary: "Alert + Spinner — block-level status banners and the pure-CSS rotor for in-flight actions.",
    },
    {
      path: "atoms/identity",
      label: "Identity",
      group: "Atoms",
      Component: Identity,
      summary: "Mark, Wordmark, Avatar — the three brand-presence atoms.",
    },
    {
      path: "primitives/layout",
      label: "Layout",
      group: "Primitives",
      Component: Layout,
      summary: "Stack, Inline, Cluster, Split, Surface — the composition vocabulary.",
    },
    {
      path: "primitives/typography",
      label: "Typography",
      group: "Primitives",
      Component: Typography,
      summary: "Heading, Text, SectionLabel — display through caption.",
    },
    {
      path: "primitives/data",
      label: "Data",
      group: "Primitives",
      Component: Data,
      summary: "Code, Pre, Divider, InfoList, InfoRow — for inline tokens and key/value display.",
    },
    {
      path: "composition",
      label: "Composition",
      group: "Composition",
      Component: Composition,
      summary: "Card, CardHeader, PageHeader, AppShell — assembled product shapes.",
    },
    {
      path: "tokens",
      label: "Tokens",
      group: "Tokens",
      Component: Tokens,
      summary: "Brand scale, surfaces, radii, spacing, shadows — the single source of truth.",
    },
  ];

  const groups = Array.from(new Set(routes.map((r) => r.group)));

  let path = $state(readHashPath());

  function readHashPath(): string {
    if (typeof window === "undefined") return routes[0].path;
    const raw = window.location.hash.replace(/^#\/?/, "");
    return routes.find((r) => r.path === raw)?.path ?? routes[0].path;
  }

  const current = $derived(routes.find((r) => r.path === path) ?? routes[0]);

  const breadcrumbItems = $derived<BreadcrumbItem[]>([
    { label: "design.liberte.top", href: "#/" },
    { label: current.group, href: `#/${current.path.split("/")[0]}` },
    { label: current.label },
  ]);

  onMount(() => {
    applyTheme();
    const onHashChange = () => {
      path = readHashPath();
      window.scrollTo({ top: 0, behavior: "instant" });
    };
    window.addEventListener("hashchange", onHashChange);
    return () => window.removeEventListener("hashchange", onHashChange);
  });
</script>

<svelte:head>
  <title>{current.label} · design · liberte.top</title>
</svelte:head>

<ProLayout siderWidth="md">
  {#snippet header()}
    <Inline gap="md" align="center">
      <Mark size="md" />
      <Wordmark href="#/" size="md">liberte.top / design</Wordmark>
    </Inline>
  {/snippet}

  {#snippet headerActions()}
    <Pill variant="outline" tone="neutral">{config.envLabel}</Pill>
    <ThemeToggle current={$theme} onToggle={toggleTheme} />
  {/snippet}

  {#snippet sider()}
    {#each groups as group}
      <NavGroup label={group}>
        {#each routes.filter((r) => r.group === group) as route}
          <NavItem href="#/{route.path}" active={route.path === current.path}>
            {route.label}
          </NavItem>
        {/each}
      </NavGroup>
    {/each}
  {/snippet}

  {#snippet breadcrumb()}
    <Breadcrumb items={breadcrumbItems} />
  {/snippet}

  {#snippet footer()}
    <Inline gap="sm" align="center" justify="between" wrap>
      <Text size="sm" tone="tertiary">
        @liberte/svelte-components reference · light + dark
      </Text>
      <Text size="sm" tone="tertiary">design.liberte.top</Text>
    </Inline>
  {/snippet}

  <Stack gap="md">
    <PageHeader
      eyebrow={current.group}
      title={current.label}
      summary={current.summary}
      size="xl"
    />
  </Stack>

  <current.Component />
</ProLayout>
