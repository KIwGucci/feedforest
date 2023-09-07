<script lang="ts">
    import {selected_genre} from "./stores";
    export let genres: string[];
</script>

<select bind:value={$selected_genre}>
    {#each genres as genr}
        <option value={genr}>
            {genr}
        </option>
    {/each}
</select>

<style>
    select {
        border-radius: 3px;
        padding: 0.1em 0.8em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: #534e4e;
    }
</style>
