<script lang="ts">
    import { Table } from 'sveltestrap';

	let data = [
        [123, "Name1", "Open"],
        [456, "Name2", "Closed"],
    ]

</script>

<Table hover>
    <thead>
        <th>#</th>
        <th>Name</th>
        <th>Value</th>
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
