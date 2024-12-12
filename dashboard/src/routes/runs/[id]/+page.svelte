<script lang="ts">
  import type { PageData } from "./$types";
  import Run from "$lib/components/run.svelte";

  let { data }: { data: PageData } = $props();
</script>

<Run run={data.run} />
