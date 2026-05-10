<script lang="ts">
  import {
    Card,
    Cluster,
    PageHeader,
    Pill,
    Stack,
  } from "@liberte/svelte-components";
</script>

<Card>
  <Stack gap="md">
    <PageHeader
      eyebrow="Atoms · status"
      title="Pill"
      summary="Tone × variant × size — the inline status indicator. Soft is the default; solid is for emphasis; outline is for auxiliary chips that should sit lighter on a busy row."
      level={2}
      size="md"
    />

    <Stack gap="sm">
      <Cluster gap="xs">
        <Pill tone="neutral">neutral</Pill>
        <Pill tone="info">info</Pill>
        <Pill tone="success">success</Pill>
        <Pill tone="warning">warning</Pill>
        <Pill tone="danger">danger</Pill>
        <Pill tone="accent">accent</Pill>
      </Cluster>
      <Cluster gap="xs">
        <Pill variant="solid" tone="neutral">neutral</Pill>
        <Pill variant="solid" tone="info">info</Pill>
        <Pill variant="solid" tone="success">success</Pill>
        <Pill variant="solid" tone="warning">warning</Pill>
        <Pill variant="solid" tone="danger">danger</Pill>
        <Pill variant="solid" tone="accent">accent</Pill>
      </Cluster>
      <Cluster gap="xs">
        <Pill variant="outline" tone="neutral">neutral</Pill>
        <Pill variant="outline" tone="info">info</Pill>
        <Pill variant="outline" tone="success">success</Pill>
        <Pill variant="outline" tone="warning">warning</Pill>
        <Pill variant="outline" tone="danger">danger</Pill>
        <Pill variant="outline" tone="accent">accent</Pill>
      </Cluster>
    </Stack>

    <Stack gap="sm">
      <Cluster gap="xs">
        <Pill size="sm" tone="success">sm</Pill>
        <Pill size="md" tone="success">md</Pill>
      </Cluster>
      <Cluster gap="xs">
        <Pill size="sm" variant="solid" tone="info">sm solid</Pill>
        <Pill size="md" variant="solid" tone="info">md solid</Pill>
        <Pill size="sm" variant="outline" tone="info">sm outline</Pill>
        <Pill size="md" variant="outline" tone="info">md outline</Pill>
      </Cluster>
    </Stack>
  </Stack>
</Card>
