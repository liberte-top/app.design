<script lang="ts">
  import {
    Card,
    Cluster,
    Code,
    Divider,
    InfoList,
    InfoRow,
    PageHeader,
    Pill,
    Pre,
    Stack,
    Text,
  } from "@liberte/svelte-components";
</script>

<Card>
  <Stack gap="md">
    <PageHeader
      eyebrow="Primitives · data"
      title="Code, Pre, Divider, InfoList"
      summary="Inline Code chips for tokens / scopes; Pre for multiline payloads; InfoList + InfoRow for label / value rows."
      level={2}
      size="md"
    />

    <Stack gap="md">
      <Text size="md">
        The
        <Code>liberte_session</Code>
        cookie carries the session token; deleting it via
        <Code>POST /api/v1/auth/logout</Code>
        clears the active state.
      </Text>

      <Pre
        text={`{
  "subject": "op-12ab34cd",
  "auth_type": "session",
  "scopes": ["notes:read", "notes:write", "registry:admin"]
}`}
      />

      <Divider />

      <InfoList>
        <InfoRow label="Subject"><Code>op-12ab34cd</Code></InfoRow>
        <InfoRow label="Auth type"><Text size="md">session</Text></InfoRow>
        <InfoRow label="Email status">
          <Pill tone="success">verified</Pill>
        </InfoRow>
        <InfoRow label="Scopes">
          <Cluster gap="xs">
            <Pill tone="success">notes:read</Pill>
            <Pill tone="success">notes:write</Pill>
            <Pill tone="success">registry:admin</Pill>
          </Cluster>
        </InfoRow>
        <InfoRow label="Created"><Text size="md">2026-03-12 03:21:32 UTC</Text></InfoRow>
      </InfoList>

      <Divider tone="default" />

      <Stack gap="sm">
        <Text size="sm" tone="tertiary">Divider — subtle (default) and default tone</Text>
        <Divider />
        <Divider tone="default" />
      </Stack>
    </Stack>
  </Stack>
</Card>
