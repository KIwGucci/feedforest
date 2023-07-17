<script lang="ts">
  export let feeditems: FeedItem[];
  let feedlist: HTMLUListElement;

  export function gotop() {
    feedlist.scrollTo(0, 0);
  }

  type FeedItem = { title: string; link: string; date: string };

  function diffdate(date: string) {
    // 引数で渡した日付から何時間経っているか返す。
    // 24時間以上の場合は何日と返す。
    let now = Date.now();
    let pasttime = Date.parse(date);
    let hour = Math.floor((now - pasttime) / (1000 * 60 * 60));

    if (hour < 1) {
      return "";
    } else if (hour < 24) {
      return " +" + hour.toString() + " 時間";
    } else {
      return " +" + Math.floor(hour / 24).toString() + " 日";
    }
  }
</script>

<ul bind:this={feedlist} id="feedlist">
  {#each feeditems as feed}
    <article>
      <a href={feed.link} target="_blank"
        >{feed.title.slice(0, 48)} {diffdate(feed.date)}</a
      >
    </article>
  {/each}
</ul>

<style>
  #feedlist {
    min-height: 100px;
    max-height: 500px;
    overflow: scroll;
  }
  a {
    color: rgb(18, 5, 54);
    text-decoration: none;
  }
  article {
    font-size: 1.1em;
    position: relative;
    padding-top: 0.2em;
    padding-bottom: 0.2em;
    border-bottom: 1px solid rgb(152, 151, 151);
  }

  article:hover {
    color: rgb(18, 5, 54);
    background-color: rgba(190, 214, 154, 0.826);
    border-radius: 3px;
  }
</style>
