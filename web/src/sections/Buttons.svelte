<script lang="ts">
  import {
    Button,
    Card,
    Cluster,
    IconButton,
    LinkButton,
    PageHeader,
    Spinner,
    Stack,
  } from "@liberte/svelte-components";
</script>

<Card>
  <Stack gap="md">
    <PageHeader
      eyebrow="Atoms · interactive"
      title="Buttons"
      summary="Button (form-bound), LinkButton (navigation), IconButton (compact actions). All variants, sizes, and the in-flight Spinner combination."
      level={2}
      size="md"
    />

    <Stack gap="sm">
      <Cluster gap="sm">
        <Button variant="primary">Primary</Button>
        <Button variant="secondary">Secondary</Button>
        <Button variant="ghost">Ghost</Button>
        <Button variant="primary" disabled>Disabled primary</Button>
        <Button variant="secondary" disabled>Disabled secondary</Button>
      </Cluster>
      <Cluster gap="sm">
        <Button size="sm">Small</Button>
        <Button size="md">Medium</Button>
        <Button size="lg">Large</Button>
        <Button size="md" block>Block (fills row)</Button>
      </Cluster>
      <Cluster gap="sm">
        <Button>
          <Spinner size="sm" />
          Pending
        </Button>
        <Button variant="secondary">
          <Spinner size="sm" tone="muted" />
          Loading
        </Button>
        <Button variant="ghost">
          <Spinner size="sm" tone="muted" />
          Refresh
        </Button>
      </Cluster>
    </Stack>

    <Stack gap="sm">
      <Cluster gap="sm">
        <LinkButton href="#" variant="primary">Link primary</LinkButton>
        <LinkButton href="#" variant="secondary">Link secondary</LinkButton>
        <LinkButton href="#" variant="ghost">Link ghost</LinkButton>
      </Cluster>
      <Cluster gap="sm">
        <LinkButton href="#" size="sm">sm</LinkButton>
        <LinkButton href="#" size="md">md</LinkButton>
        <LinkButton href="#" size="lg">lg</LinkButton>
      </Cluster>
    </Stack>

    <Cluster gap="sm">
      <IconButton variant="ghost" size="sm" aria-label="Refresh">
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          aria-hidden="true"
        >
          <path d="M21 12a9 9 0 1 1-3-6.7L21 8" />
          <path d="M21 3v5h-5" />
        </svg>
      </IconButton>
      <IconButton variant="ghost" size="md" aria-label="Refresh">
        <svg
          width="18"
          height="18"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          aria-hidden="true"
        >
          <path d="M21 12a9 9 0 1 1-3-6.7L21 8" />
          <path d="M21 3v5h-5" />
        </svg>
      </IconButton>
      <IconButton variant="secondary" size="sm" aria-label="Refresh secondary">
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          aria-hidden="true"
        >
          <path d="M21 12a9 9 0 1 1-3-6.7L21 8" />
          <path d="M21 3v5h-5" />
        </svg>
      </IconButton>
      <IconButton variant="secondary" size="md" aria-label="Refresh secondary">
        <svg
          width="18"
          height="18"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          aria-hidden="true"
        >
          <path d="M21 12a9 9 0 1 1-3-6.7L21 8" />
          <path d="M21 3v5h-5" />
        </svg>
      </IconButton>
    </Cluster>
  </Stack>
</Card>
