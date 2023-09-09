<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import FeedList from "./lib/FeedList.svelte";
  import { selected_genre } from "./lib/stores";
  import Selecter from "./lib/Selecter.svelte";

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
  let display_word: string = "";
  $: search_tags = display_word.split(" ");
  let genres = Object.keys(rss_urls);
  let feeditems: FeedItem[] = [];
  let status_message = "";

  let is_stock = true;

  async function get_urls() {
    // Url設定ファイルからRss Urlとジャンルを読み出し
    rss_urls = await invoke("get_urls", {});
    genres = Object.keys(rss_urls);
    $selected_genre = "News";
  }

  async function get_all_feeds() {
    await invoke("get_all_feeds", {});
  }

  async function is_today() {
    is_stock = true;
    await getFeeds();

    let lastday = new Date(feeditems[0].date);
    let todayst = new Date();
    return lastday.toLocaleDateString() === todayst.toLocaleDateString();
  }

  async function getFeeds() {
    // 現在選択されているジャンルのフィードを取得する
    // isGetallがtrueの時、ローカルに保存しているデータのみを返す.
    let searchToken: SearchToken = {
      selected_genre: $selected_genre,
      rss_urls,
      search_word,
    };
    display_word = search_word;

    let result: FeedState = await invoke("get_feeds", {
      searchToken,
      fromStock: is_stock,
    });

    feeditems = result.feeditems;
    status_message = result.message;
  }

  function reload_feed() {
    // 現在選択されているジャンルのフィードを取得する
    // 必ずNetからFeedを取得する
    is_stock = false;
    getFeeds().catch((err) => {
      status_message = err;
    });
  }

  function listhandler() {
    feedlist.gotop();
    status_message =
      search_word === "" ? status_message : "Search Word:" + search_word;
    search_word = "";
  }

  function select_handler() {
    is_stock = false;
    getFeeds().then(() => {
      listhandler();
    });
  }

  function search_handler() {
    is_stock = true;
    getFeeds().then(() => {
      listhandler();
    });
  }

  async function initfunction() {
    // 初回に一度だけ処理したい工程を集めた関数
    get_urls().then(async () => {
      let stockistoday = await is_today();
      if (!stockistoday) {
        await get_all_feeds();
        console.log("all gets")
      } else {

        await getFeeds();
      }
    });
  }
</script>

{#await initfunction()}
  <p id="waiting">更新を待っています</p>
{:then}
  <main class="container">
    <div class="row" id="operate">
      <form on:change|preventDefault={select_handler}>
        <Selecter {genres} />
      </form>

      <form on:submit|preventDefault={search_handler}>
        <input
          type="text"
          id="searchword"
          placeholder="input search word"
          bind:value={search_word}
        />
      </form>

      <button on:click|preventDefault={reload_feed}>Reload</button>

      {#each search_tags as stag}
        <div id="displayword">
          {stag}
        </div>
      {/each}

      <!-- row end -->
    </div>

    <FeedList {feeditems} bind:this={feedlist} />

    {#if status_message !== undefined}
      <p style="margin-left: 3rem;">{status_message}</p>
    {/if}
  </main>
{/await}

<style>
  :root {
    font-family: Arial, Helvetica, sans-serif;
    font-size: 16px;
    line-height: 24px;

    color: #ece1e1;
    background-color: whitesmoke;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }
  .row {
    display: flex;
    justify-content: center;
    align-items: center;
  }
  /* input {
    border-radius: 3px;
    padding: 0.2em 0.5em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color:black;
    text-align: center;
    border-style: ridge;
    border-color: gray;
    transition: border-color 0.25s;
    margin-left: 1em;
  } */
  button,input {
    font-size: 1em;
    font-family: inherit;
    background-color: white;
    border-radius: 5px;
    border-style:ridge;
    padding: 0.25em 0.5em;
    margin-left: 1em;
  }
  button:hover {
    background-color: lightskyblue
  }
  #waiting {
    font-size: large;
    text-align: center;
  }
  #displayword {
    background-color:darkslateblue;
    color: white;
    padding-inline: 0.2rem;
    margin-inline-start: 1em;
    border-radius: 0.2rem;
  }
</style>
