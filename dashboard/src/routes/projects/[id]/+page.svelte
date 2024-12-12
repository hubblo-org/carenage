<script lang="ts">
  import type { PageData } from "./$types";
  import Project from "$lib/components/project.svelte";

  let { data }: { data: PageData } = $props();
</script>

<Project project={data.project} />
