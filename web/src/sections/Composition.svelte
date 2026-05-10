<script lang="ts">
  import {
    Button,
    Card,
    CardHeader,
    Heading,
    LinkButton,
    PageHeader,
    Pill,
    SectionLabel,
    Stack,
    Text,
  } from "@liberte/svelte-components";
</script>

<Card>
  <Stack gap="md">
    <PageHeader
      eyebrow="Composition"
      title="Card, CardHeader, PageHeader, AppShell"
      summary="Compositions assemble primitives into recurring product shapes. Card / CardHeader live below; AppShell wraps this entire page; PageHeader heads each section card you are looking at right now."
      level={2}
      size="md"
    />

    <Stack gap="md">
      <SectionLabel>Card padding — sm / md / lg</SectionLabel>
      <Card padding="sm">
        <Text size="sm" tone="tertiary">padding=sm</Text>
        <Text>Tight, dense rows.</Text>
      </Card>
      <Card padding="md">
        <Text size="sm" tone="tertiary">padding=md (default)</Text>
        <Text>Most application cards.</Text>
      </Card>
      <Card padding="lg">
        <Text size="sm" tone="tertiary">padding=lg</Text>
        <Text>Hero / feature cards with breathing room.</Text>
      </Card>
    </Stack>

    <Stack gap="md">
      <SectionLabel>Card with header / footer slots</SectionLabel>
      <Card>
        {#snippet header()}
          <CardHeader>
            <SectionLabel>Header slot</SectionLabel>
            <Heading level={3} size="md">A card with both a header and a footer</Heading>
            <Text tone="secondary" size="sm">
              The header band carries the eyebrow / title / summary; the body
              carries action content; the footer carries primary CTAs.
            </Text>
          </CardHeader>
        {/snippet}

        <Text>
          Body content. The Stack the consumer wraps the children in supplies
          rhythm; the Card itself only owns the surface, padding, and divider
          rules between header / body / footer.
        </Text>

        {#snippet footer()}
          <Stack gap="sm">
            <LinkButton block href="#">Primary CTA</LinkButton>
            <Button block variant="secondary">Secondary CTA</Button>
          </Stack>
        {/snippet}
      </Card>
    </Stack>

    <Stack gap="md">
      <SectionLabel>PageHeader — eyebrow + title + summary + actions</SectionLabel>
      <Card>
        <PageHeader
          eyebrow="Token rotation"
          title="Pending quarterly key rotation"
          summary="The next scheduled rotation lands on 2026-06-01. Acknowledge once you have rotated downstream consumers."
          level={3}
          size="md"
        >
          {#snippet actions()}
            <Pill tone="warning">due in 21 days</Pill>
            <Button size="sm">Acknowledge</Button>
          {/snippet}
        </PageHeader>
      </Card>
    </Stack>
  </Stack>
</Card>
