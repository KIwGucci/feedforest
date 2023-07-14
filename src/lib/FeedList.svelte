<script lang="ts">
  export let feeditems: FeedItem[];
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

<ul id="feedlist">
  {#each feeditems as feed}
    <li>
      <a href={feed.link} target="_blank">{feed.title} {diffdate(feed.date)}</a>
    </li>
  {/each}
</ul>

<style>
  #feedlist {
    font-size: 1.1em;
    min-height: 100px;
    max-height: 500px;
    overflow: scroll;
  }
  a {
    font-weight: 500;
    color: rgb(18, 5, 54);
    text-decoration: none;
  }

  a:hover {
    color: rgb(18, 5, 54);
    background-color: rgba(190, 214, 154, 0.826);
    border-radius: 3px;
    padding: 0.3em;
  }
</style>
