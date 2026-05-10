<script lang="ts">
  import {
    Card,
    Field,
    Input,
    PageHeader,
    Split,
    Stack,
  } from "@liberte/svelte-components";
</script>

<Card>
  <Stack gap="md">
    <PageHeader
      eyebrow="Atoms · form"
      title="Field & Input"
      summary="Field wraps any control with a label / hint / error / optional badge; Input is the canonical text control with brand-tinted focus ring."
      level={2}
      size="md"
    />

    <Split gap="md" min="md">
      <Field label="Email" required hint="We will send a magic link.">
        <Input type="email" placeholder="you@example.com" />
      </Field>
      <Field label="Display name" optional="Optional">
        <Input placeholder="A friendly name" />
      </Field>
    </Split>

    <Split gap="md" min="md">
      <Field label="With value">
        <Input value="op-12ab34cd" />
      </Field>
      <Field label="Disabled">
        <Input value="locked" disabled />
      </Field>
    </Split>

    <Split gap="md" min="md">
      <Field label="Validation error" error="Email format looks invalid.">
        <Input type="email" value="not-an-email" />
      </Field>
      <Field label="Password" required>
        <Input type="password" value="secret-token-123" />
      </Field>
    </Split>

    <Split gap="md" min="md">
      <Field label="Size md (default)">
        <Input size="md" placeholder="medium control height" />
      </Field>
      <Field label="Size lg">
        <Input size="lg" placeholder="large control height" />
      </Field>
    </Split>
  </Stack>
</Card>
