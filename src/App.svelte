<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import FeedList from "./lib/FeedList.svelte";



  type FeedItem = { title: string; link: string; date: string };
  type FeedState = { feeditems: FeedItem[]; message: string };
  type RssUrls = { [key: string]: string[] };
  type SearchToken = {
    selected_genre: string;
    rss_urls: RssUrls;
    search_word: string;
  };

  let feedlist: FeedList;
  let rss_urls: RssUrls = { "": [""] };
  let search_word: string = "";
  let genres = Object.keys(rss_urls);
  let selected_genre: string = genres[0];
  let feeditems: FeedItem[] = [];
  let status_message = "";

  async function get_urls() {
    // Url設定ファイルからRss Urlとジャンルを読み出し
    rss_urls = await invoke("get_urls", {});
    genres = Object.keys(rss_urls);
    selected_genre = "News";
  }

  async function get_feeds(myselected_genre:string) {
    // 現在選択されているジャンルのフィードを取得する
    let searchToken: SearchToken = {
      selected_genre:myselected_genre,
      rss_urls,
      search_word,
    };

  
    let result: FeedState = await invoke("get_feeds", {
      searchToken,
    });
    feeditems = result.feeditems;
    status_message = result.message;
  }

  async function listhandler() {
    await get_feeds(selected_genre);
    feedlist.gotop();

  }
  async function initfunction() {
    await get_urls();
    get_feeds(selected_genre);
  }
  // 初回一回だけ実行
  initfunction();
</script>

<main class="container">
  
    <div class="row">
      <select bind:value={selected_genre} on:change={listhandler}>
        {#each genres as genr}
          <option value={genr}>
            {genr}
          </option>
        {/each}
      </select>

      <form on:submit|preventDefault={listhandler}>
        <input
          type="text"
          id="searchword"
          placeholder="input search word"
          bind:value={search_word}
        />
      </form>

      <!-- row end -->
    </div>

    <FeedList feeditems= {feeditems} bind:this={feedlist} />

</main>

<style>
  :root {
    font-family: Arial, Helvetica, sans-serif;
    font-size: 16px;
    line-height: 24px;

    color: #110101;
    background-color: #f4f8eb;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }
  .row {
    display: flex;
    justify-content: center;
  }
  input,
  select {
    border-radius: 3px;
    /* border: 1px solid transparent; */
    padding: 0.1em 0.8em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #534e4e;
    /* transition: border-color 0.25s; */
  }
</style>
