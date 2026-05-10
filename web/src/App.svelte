<script lang="ts">
  import {
    AppShell,
    Cluster,
    Inline,
    LinkButton,
    PageHeader,
    Pill,
    Stack,
    Text,
    ThemeToggle,
    Wordmark,
    applyTheme,
    theme,
    toggleTheme,
  } from "@liberte/svelte-components";
  import { onMount } from "svelte";
  import { config } from "./lib/config";

  import Buttons from "./sections/Buttons.svelte";
  import Inputs from "./sections/Inputs.svelte";
  import Pills from "./sections/Pills.svelte";
  import Alerts from "./sections/Alerts.svelte";
  import Identity from "./sections/Identity.svelte";
  import Loading from "./sections/Loading.svelte";
  import Layout from "./sections/Layout.svelte";
  import Typography from "./sections/Typography.svelte";
  import Data from "./sections/Data.svelte";
  import Composition from "./sections/Composition.svelte";
  import Tokens from "./sections/Tokens.svelte";

  type SectionEntry = {
    id: string;
    label: string;
    Component: typeof Buttons;
  };

  const sections: SectionEntry[] = [
    { id: "buttons", label: "Buttons", Component: Buttons },
    { id: "inputs", label: "Inputs", Component: Inputs },
    { id: "pills", label: "Pills", Component: Pills },
    { id: "alerts", label: "Alerts", Component: Alerts },
    { id: "identity", label: "Identity", Component: Identity },
    { id: "loading", label: "Loading", Component: Loading },
    { id: "layout", label: "Layout", Component: Layout },
    { id: "typography", label: "Typography", Component: Typography },
    { id: "data", label: "Data", Component: Data },
    { id: "composition", label: "Composition", Component: Composition },
    { id: "tokens", label: "Tokens", Component: Tokens },
  ];

  onMount(() => {
    applyTheme();
  });
</script>

<AppShell width="lg">
  {#snippet header()}
    <Wordmark href="#" size="lg" mark>liberte.top</Wordmark>
  {/snippet}
  {#snippet aside()}
    <Pill variant="outline" tone="neutral">{config.envLabel}</Pill>
    <Pill variant="outline" tone="accent">design system</Pill>
    <ThemeToggle current={$theme} onToggle={toggleTheme} />
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
      eyebrow="design system"
      title="@liberte/svelte-components"
      summary="A stateless, exhaustive reference of every primitive, atom, and composition shipped by the design package. Toggle the theme to see tokens repaint everything in place."
      size="xl"
    />

    <Cluster gap="xs">
      {#each sections as section}
        <LinkButton href="#{section.id}" variant="ghost" size="sm">
          {section.label}
        </LinkButton>
      {/each}
    </Cluster>
  </Stack>

  {#each sections as section}
    <span id={section.id} class="lt-anchor"></span>
    <section.Component />
  {/each}
</AppShell>

<style>
  /* Anchor offset so #fragment scrolls to the section title, not under the
   * sticky-ish header. The element itself is invisible. */
  .lt-anchor {
    display: block;
    height: 0;
    scroll-margin-top: var(--lt-space-6);
  }
</style>
