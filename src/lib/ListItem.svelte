<script lang="ts">
  type FeedItem = { title: string; link: string; date: string };
  // import { invoke } from "@tauri-apps/api/tauri";
  export let mfeed: FeedItem;
  export let mindex: number;

</script>

<article>
  <a href={mfeed.link} target="_blank">
    {mindex + 1}: {mfeed.title} | {mfeed.date.split('T')[0]}
  </a>
</article>

<style>
  a {
    color: rgb(18, 5, 54);
    text-decoration: none;
  }
  article {
    font-size: 1.1em;
    /* position: relative; */
    padding-top: 0.2em;
    padding-bottom: 0.2em;
    border-bottom: 1px solid rgb(152, 151, 151);
  }

  article:hover {
    color: chocolate;
    background-color:lemonchiffon;
    border-radius: 3px;
  }
  /* .datestring {
    float: right;
  } */
</style>
