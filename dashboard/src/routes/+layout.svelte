<script lang="ts">
  import Header from "$lib/components/header.svelte";
  import type { LayoutData } from "./$types";
  import type { Snippet } from "svelte";

  let { data, children }: { data: LayoutData; children: Snippet } = $props();
</script>

<Header project={data.project} />

{@render children()}
