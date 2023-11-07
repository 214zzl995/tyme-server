<script>
  import SvelteMarkdown from "svelte-markdown";

  export const source = ``;

  /**
   * @param {{ detail: { tokens: any; }; }} event
   */
  function handleParsed(event) {
    console.log(event.detail.tokens);
  }
</script>

<SvelteMarkdown {source} on:parsed={handleParsed} />
