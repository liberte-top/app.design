<script lang="ts">
  import {
    Card,
    Cluster,
    Code,
    Inline,
    PageHeader,
    SectionLabel,
    Split,
    Stack,
    Surface,
    Text,
  } from "@liberte/svelte-components";

  const brandSteps = [
    "50",
    "100",
    "200",
    "300",
    "400",
    "500",
    "600",
    "700",
    "800",
    "900",
  ] as const;

  const textTones = [
    ["primary", "Primary"],
    ["secondary", "Secondary"],
    ["tertiary", "Tertiary"],
    ["quaternary", "Quaternary"],
    ["disabled", "Disabled"],
  ] as const;

  const surfaces = [
    ["bg", "Background"],
    ["surface", "Surface"],
    ["surface-raised", "Surface raised"],
    ["surface-sunken", "Surface sunken"],
    ["surface-muted", "Surface muted"],
    ["surface-strong", "Surface strong"],
  ] as const;

  const radii = [
    ["xs", "4 px"],
    ["sm", "6 px"],
    ["md", "10 px"],
    ["lg", "14 px"],
    ["xl", "18 px"],
    ["pill", "9999 px"],
  ] as const;

  const spacings = [
    ["1", "4 px"],
    ["2", "8 px"],
    ["3", "12 px"],
    ["4", "16 px"],
    ["5", "20 px"],
    ["6", "24 px"],
    ["8", "32 px"],
    ["10", "40 px"],
    ["12", "48 px"],
    ["16", "64 px"],
  ] as const;
</script>

<Card>
  <Stack gap="md">
    <PageHeader
      eyebrow="Tokens"
      title="Theme variables"
      summary="The single source of truth in default.css. Every value here repaints the entire product when the token shifts."
      level={2}
      size="md"
    />

    <Stack gap="lg">
      <Stack gap="sm">
        <SectionLabel>Brand scale — anchor 500 = #3551d6</SectionLabel>
        <Cluster gap="sm">
          {#each brandSteps as step}
            <Stack gap="xs" align="center">
              <span class="lt-token-swatch" style="background: var(--lt-color-brand-{step});"></span>
              <Text size="xs" tone="tertiary">{step}</Text>
            </Stack>
          {/each}
        </Cluster>
      </Stack>

      <Split gap="md" min="md">
        <Stack gap="sm">
          <SectionLabel>Text tones</SectionLabel>
          <Stack gap="xs">
            {#each textTones as [token, label]}
              <Inline gap="sm" align="center">
                <Code>--lt-color-text-{token}</Code>
                <Text size="md" style="color: var(--lt-color-text-{token});">{label}</Text>
              </Inline>
            {/each}
          </Stack>
        </Stack>

        <Stack gap="sm">
          <SectionLabel>Surface tokens</SectionLabel>
          <Stack gap="xs">
            {#each surfaces as [token, label]}
              <Inline gap="sm" align="center">
                <span class="lt-token-swatch lt-token-swatch--bordered" style="background: var(--lt-color-{token});"></span>
                <Code>--lt-color-{token}</Code>
                <Text size="sm" tone="tertiary">{label}</Text>
              </Inline>
            {/each}
          </Stack>
        </Stack>
      </Split>

      <Stack gap="sm">
        <SectionLabel>Radius scale</SectionLabel>
        <Cluster gap="md">
          {#each radii as [token, px]}
            <Stack gap="xs" align="center">
              <span class="lt-token-radius" style="border-radius: var(--lt-radius-{token});"></span>
              <Text size="xs" tone="tertiary">{token} · {px}</Text>
            </Stack>
          {/each}
        </Cluster>
      </Stack>

      <Stack gap="sm">
        <SectionLabel>Spacing scale (4-step base)</SectionLabel>
        <Stack gap="xs">
          {#each spacings as [token, px]}
            <Inline gap="sm" align="center">
              <Code>--lt-space-{token}</Code>
              <span class="lt-token-spacing" style="width: var(--lt-space-{token});"></span>
              <Text size="xs" tone="tertiary">{px}</Text>
            </Inline>
          {/each}
        </Stack>
      </Stack>

      <Stack gap="sm">
        <SectionLabel>Shadows</SectionLabel>
        <Split gap="md" min="md">
          <Surface tone="default" padding="md">
            <span class="lt-token-shadow lt-token-shadow--xs">xs</span>
          </Surface>
          <Surface tone="default" padding="md">
            <span class="lt-token-shadow lt-token-shadow--sm">sm</span>
          </Surface>
          <Surface tone="default" padding="md">
            <span class="lt-token-shadow lt-token-shadow--md">md</span>
          </Surface>
          <Surface tone="default" padding="md">
            <span class="lt-token-shadow lt-token-shadow--lg">lg</span>
          </Surface>
          <Surface tone="default" padding="md">
            <span class="lt-token-shadow lt-token-shadow--card">card</span>
          </Surface>
        </Split>
      </Stack>
    </Stack>
  </Stack>
</Card>

<style>
  .lt-token-swatch {
    width: 28px;
    height: 28px;
    border-radius: var(--lt-radius-sm);
    display: inline-block;
  }
  .lt-token-swatch--bordered {
    border: 1px solid var(--lt-color-border);
  }
  .lt-token-radius {
    width: 36px;
    height: 36px;
    background: var(--lt-color-brand-500);
    display: inline-block;
  }
  .lt-token-spacing {
    height: 8px;
    background: var(--lt-color-brand-500);
    display: inline-block;
    border-radius: var(--lt-radius-xs);
    flex-shrink: 0;
  }
  .lt-token-shadow {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 12px 16px;
    border-radius: var(--lt-radius-md);
    background: var(--lt-color-surface);
    font-size: var(--lt-text-sm);
    font-weight: var(--lt-font-weight-medium);
    color: var(--lt-color-text-secondary);
  }
  .lt-token-shadow--xs {
    box-shadow: var(--lt-shadow-xs);
  }
  .lt-token-shadow--sm {
    box-shadow: var(--lt-shadow-sm);
  }
  .lt-token-shadow--md {
    box-shadow: var(--lt-shadow-md);
  }
  .lt-token-shadow--lg {
    box-shadow: var(--lt-shadow-lg);
  }
  .lt-token-shadow--card {
    box-shadow: var(--lt-shadow-card);
  }
</style>
