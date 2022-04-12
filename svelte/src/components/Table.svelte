<script lang="ts">
    import { Table } from 'sveltestrap';

	export let data = []

</script>

<Table hover class="m-3">
    <thead>
        <th>#</th>
        <th>Amount</th>
        <th>Start Date</th>
    </thead>
    <tbody>
        {#each data as row}
            <tr>
                {#each row as col}
                    <td>{ col }</td>
                {/each}
            </tr>
        {/each}
    </tbody>
</Table>
