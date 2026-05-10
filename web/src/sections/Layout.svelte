<script lang="ts">
  import {
    Card,
    Cluster,
    Inline,
    PageHeader,
    SectionLabel,
    Split,
    Stack,
    Surface,
    Text,
  } from "@liberte/svelte-components";
</script>

<Card>
  <Stack gap="md">
    <PageHeader
      eyebrow="Primitives · layout"
      title="Stack, Inline, Cluster, Split, Surface"
      summary="The composition vocabulary. Stack is vertical, Inline is horizontal, Cluster is wrap-friendly chip rows, Split is responsive grid, Surface is the bare tone box."
      level={2}
      size="md"
    />

    <Stack gap="lg">
      <Stack gap="sm">
        <SectionLabel>Stack — gap none / xs / sm / md / lg / xl</SectionLabel>
        <Surface tone="muted" padding="md">
          <Stack gap="md">
            {#each ["none", "xs", "sm", "md", "lg", "xl"] as gap}
              <Stack gap={gap as "none" | "xs" | "sm" | "md" | "lg" | "xl"}>
                <Text size="sm" tone="tertiary">gap = {gap}</Text>
                <Surface tone="default" padding="sm">
                  <Text size="sm">item one</Text>
                </Surface>
                <Surface tone="default" padding="sm">
                  <Text size="sm">item two</Text>
                </Surface>
              </Stack>
            {/each}
          </Stack>
        </Surface>
      </Stack>

      <Stack gap="sm">
        <SectionLabel>Inline — justify between / start / center / end / around</SectionLabel>
        <Surface tone="muted" padding="md">
          <Stack gap="md">
            {#each ["start", "center", "end", "between", "around"] as j}
              <Inline
                gap="sm"
                align="center"
                justify={j as "start" | "center" | "end" | "between" | "around"}
              >
                <Text size="sm" tone="tertiary">{j}</Text>
                <Surface tone="default" padding="sm">
                  <Text size="sm">a</Text>
                </Surface>
                <Surface tone="default" padding="sm">
                  <Text size="sm">b</Text>
                </Surface>
                <Surface tone="default" padding="sm">
                  <Text size="sm">c</Text>
                </Surface>
              </Inline>
            {/each}
          </Stack>
        </Surface>
      </Stack>

      <Stack gap="sm">
        <SectionLabel>Cluster — wrap-friendly chip rows</SectionLabel>
        <Surface tone="muted" padding="md">
          <Cluster gap="sm">
            {#each Array(12) as _, idx}
              <Surface tone="default" padding="sm">
                <Text size="sm">tag-{idx + 1}</Text>
              </Surface>
            {/each}
          </Cluster>
        </Surface>
      </Stack>

      <Stack gap="sm">
        <SectionLabel>Split — equal-share, collapses below min</SectionLabel>
        <Surface tone="muted" padding="md">
          <Stack gap="md">
            <Split gap="md" min="md">
              <Surface tone="default" padding="md"><Text size="sm">left</Text></Surface>
              <Surface tone="default" padding="md"><Text size="sm">right</Text></Surface>
            </Split>
            <Split gap="md" min="sm">
              <Surface tone="default" padding="md"><Text size="sm">a</Text></Surface>
              <Surface tone="default" padding="md"><Text size="sm">b</Text></Surface>
              <Surface tone="default" padding="md"><Text size="sm">c</Text></Surface>
            </Split>
          </Stack>
        </Surface>
      </Stack>

      <Stack gap="sm">
        <SectionLabel>Surface — tone × padding × radius × border</SectionLabel>
        <Split gap="sm" min="sm">
          <Surface tone="default" padding="md">
            <Text size="sm">default</Text>
          </Surface>
          <Surface tone="muted" padding="md">
            <Text size="sm">muted</Text>
          </Surface>
          <Surface tone="sunken" padding="md">
            <Text size="sm">sunken</Text>
          </Surface>
          <Surface tone="accent-soft" padding="md">
            <Text size="sm">accent-soft</Text>
          </Surface>
        </Split>
      </Stack>
    </Stack>
  </Stack>
</Card>
