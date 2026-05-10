<script lang="ts">
  import {
    Avatar,
    Card,
    Cluster,
    Divider,
    Inline,
    Mark,
    PageHeader,
    Split,
    Stack,
    Text,
    Wordmark,
  } from "@liberte/svelte-components";
</script>

<Card>
  <Stack gap="md">
    <PageHeader
      eyebrow="Atoms · identity"
      title="Mark, Wordmark, Avatar"
      summary="The three brand-presence atoms. Mark is the canonical glyph; Wordmark pairs it with a configurable text; Avatar fills user-specific spots with image or initials."
      level={2}
      size="md"
    />

    <Split gap="md" min="md">
      <Stack gap="sm">
        <Text size="sm" tone="tertiary" weight="medium">Mark — sm / md / lg</Text>
        <Inline gap="md" align="center">
          <Mark size="sm" />
          <Mark size="md" />
          <Mark size="lg" />
        </Inline>
      </Stack>
      <Stack gap="sm">
        <Text size="sm" tone="tertiary" weight="medium">Wordmark — sm / md / lg</Text>
        <Stack gap="sm">
          <Wordmark size="sm">liberte.top</Wordmark>
          <Wordmark size="md">liberte.top</Wordmark>
          <Wordmark size="lg">liberte.top</Wordmark>
        </Stack>
      </Stack>
    </Split>

    <Divider />

    <Stack gap="sm">
      <Text size="sm" tone="tertiary" weight="medium">Wordmark with leading Mark glyph</Text>
      <Stack gap="sm">
        <Wordmark size="sm" mark>liberte.top</Wordmark>
        <Wordmark size="md" mark>liberte.top</Wordmark>
        <Wordmark size="lg" mark>liberte.top</Wordmark>
      </Stack>
    </Stack>

    <Divider />

    <Stack gap="sm">
      <Text size="sm" tone="tertiary" weight="medium">Avatar — initials inferred from name</Text>
      <Cluster gap="md">
        <Avatar name="Liberte Operator" size="sm" />
        <Avatar name="Liberte Operator" size="md" />
        <Avatar name="Liberte Operator" size="lg" />
        <Avatar name="A" size="md" />
        <Avatar name="ada lovelace" size="md" />
        <Avatar size="md" />
      </Cluster>
    </Stack>
  </Stack>
</Card>
