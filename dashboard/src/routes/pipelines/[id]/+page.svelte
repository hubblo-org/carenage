<script lang="ts">
  import type { PageData } from "./$types";
  import Pipeline from "$lib/components/pipeline.svelte";

  let { data }: { data: PageData } = $props();
</script>

<Pipeline pipeline={data.pipeline} />
