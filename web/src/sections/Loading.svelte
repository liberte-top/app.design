<script lang="ts">
  import {
    Card,
    Cluster,
    Inline,
    PageHeader,
    Spinner,
    Stack,
    Text,
  } from "@liberte/svelte-components";
</script>

<Card>
  <Stack gap="md">
    <PageHeader
      eyebrow="Atoms · loading"
      title="Spinner"
      summary="Pure-CSS rotor. Inherits color via currentColor so it adapts to whichever atom hosts it; sizes pair with sm/md/lg button heights."
      level={2}
      size="md"
    />

    <Cluster gap="md">
      <Inline gap="sm" align="center">
        <Spinner size="sm" />
        <Text size="sm" tone="tertiary">sm · current</Text>
      </Inline>
      <Inline gap="sm" align="center">
        <Spinner size="md" />
        <Text size="sm" tone="tertiary">md · current</Text>
      </Inline>
      <Inline gap="sm" align="center">
        <Spinner size="lg" />
        <Text size="sm" tone="tertiary">lg · current</Text>
      </Inline>
    </Cluster>

    <Cluster gap="md">
      <Inline gap="sm" align="center">
        <Spinner size="md" tone="muted" />
        <Text size="sm" tone="tertiary">muted</Text>
      </Inline>
      <Inline gap="sm" align="center">
        <Spinner size="md" tone="accent" />
        <Text size="sm" tone="tertiary">accent (brand)</Text>
      </Inline>
      <Inline gap="sm" align="center">
        <Spinner size="md" tone="current" />
        <Text size="sm" weight="strong">current (inherits ink)</Text>
      </Inline>
    </Cluster>
  </Stack>
</Card>
