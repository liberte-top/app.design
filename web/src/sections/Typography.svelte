<script lang="ts">
  import {
    Card,
    Heading,
    PageHeader,
    SectionLabel,
    Stack,
    Text,
  } from "@liberte/svelte-components";
</script>

<Card>
  <Stack gap="md">
    <PageHeader
      eyebrow="Primitives · typography"
      title="Heading, Text, SectionLabel"
      summary="Heading covers display through h6; Text covers body / caption tones; SectionLabel is the uppercase eyebrow that sits above each card."
      level={2}
      size="md"
    />

    <Stack gap="md">
      <Stack gap="sm">
        <SectionLabel>Heading — sm / md / lg / xl / display</SectionLabel>
        <Heading level={1} size="display">Display</Heading>
        <Heading level={1} size="xl">Extra large</Heading>
        <Heading level={2} size="lg">Large</Heading>
        <Heading level={3} size="md">Medium</Heading>
        <Heading level={4} size="sm">Small</Heading>
        <Heading level={3} size="md" tone="muted">Muted tone</Heading>
      </Stack>

      <Stack gap="sm">
        <SectionLabel>Text — size × weight × tone</SectionLabel>
        <Text size="lg">Large body — comfortable for short paragraphs.</Text>
        <Text size="base">Base body — primary reading size.</Text>
        <Text size="md">Md body — slightly tighter.</Text>
        <Text size="sm">Sm body — captions and metadata.</Text>
        <Text size="xs">Xs body — labels and footnotes.</Text>
        <Text weight="medium">Medium weight</Text>
        <Text weight="strong">Strong weight</Text>
        <Text tone="secondary">Secondary tone</Text>
        <Text tone="tertiary">Tertiary tone</Text>
        <Text tone="success">Success tone</Text>
        <Text tone="warning">Warning tone</Text>
        <Text tone="danger">Danger tone</Text>
        <Text tone="info">Info tone</Text>
      </Stack>

      <Stack gap="sm">
        <SectionLabel>Section labels (this row's eyebrow is one)</SectionLabel>
        <SectionLabel>section · label</SectionLabel>
        <SectionLabel>another · eyebrow</SectionLabel>
      </Stack>
    </Stack>
  </Stack>
</Card>
