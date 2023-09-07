<script lang="ts">
  export let feeditems: FeedItem[];
  import ListItem from "./ListItem.svelte";
  let feedlist: HTMLUListElement;

  export function gotop() {
    feedlist.scrollTo(0, 0);
  }
  type FeedItem = { title: string; link: string; date: string };
</script>

<ul bind:this={feedlist} id="feedlist">
  {#each feeditems as feed, index}
    <ListItem mfeed={feed} mindex={index} />
  {/each}
</ul>

<style>
  #feedlist {
    flex: 1;
    min-height: 100px;
    max-height: 500px;
    overflow: scroll;
  }
  
</style>
