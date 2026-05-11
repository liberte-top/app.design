<script lang="ts">
  import {
    Alert,
    Card,
    Cluster,
    Code,
    Inline,
    PageHeader,
    Pre,
    Spinner,
    Stack,
    Text,
  } from "@liberte/svelte-components";
</script>

<Stack gap="lg">
  <Card>
    <Stack gap="md">
      <PageHeader
        eyebrow="Atoms · feedback"
        title="Alert"
        summary="Block-level status feedback. Inline content (text + Code) flows naturally; nested Stack lets it carry secrets, code blocks, or multi-line copy."
        level={2}
        size="md"
      />

      <Stack gap="sm">
        <Alert tone="info">
          <Text weight="strong">Heads up.</Text>
          Inline content flows in a single block paragraph; trailing
          <Code>punctuation</Code>
          renders on the same line.
        </Alert>
        <Alert tone="success">
          <Text weight="strong">Action completed.</Text>
          Use this for confirmation toasts and post-submit banners.
        </Alert>
        <Alert tone="warning">
          <Text weight="strong">Attention required.</Text>
          Surface non-blocking risk: token approaching expiry, quota at 80%, etc.
        </Alert>
        <Alert tone="danger">
          <Text weight="strong">Something went wrong.</Text>
          Reserve danger for blocking failures the user can act on.
        </Alert>
      </Stack>

      <Alert tone="success">
        <Stack gap="xs">
          <Text weight="strong">Token created</Text>
          <Pre text="liberte_pat_abcdef0123456789…" />
          <Text tone="secondary" size="sm">
            Copy this now. You will not see it again.
          </Text>
        </Stack>
      </Alert>
    </Stack>
  </Card>

  <Card>
    <Stack gap="md">
      <PageHeader
        eyebrow="Atoms · feedback"
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
</Stack>
