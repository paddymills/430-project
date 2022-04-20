<script lang="ts">
    import { Datatable, SearchInput, PaginationButtons, PaginationRowCount } from 'svelte-simple-datatables'
    import { Toast, Button, Icon } from 'sveltestrap';

    const settings = {
        sortable: true,
        columnFilter: true,
        rowsPerPage: 25,
        blocks: {
            searchInput: false,
            paginationButtons: false,
            paginationRowCount: false,
        }
    }

    async function deleteRow(id: Number) {
        toastText = "delete row " + id;
        isOpen = true;
    }

    let rows;
	export let data;

    let isOpen = false;
    let toastText = null;

</script>

<section>
    <Toast autohide body header="Message" {isOpen} on:close={() => (isOpen = false)}>{toastText}</Toast>
    <aside>
        {#if $rows}
            <SearchInput id={'loan-table'}/>
        {/if}
    </aside>

    <div>
        <Datatable {settings} {data} bind:dataRows={rows} id={'loan-table'}>
            <thead>
                <th></th>   <!-- for icons -->
                <th data-key="id">#</th>
                <th data-key="amount">Loan Amount</th>
                <th data-key="start_date">Start Date</th>
            </thead>
            <tbody>
            {#if rows}
                {#each $rows as row}
                <tr>
                    <td>
                        {#if row.editing}
                            <Button size="sm" color="success" on:click={() => deleteRow(row.loan_id)}><Icon name="check-square" /></Button>
                            <Button size="sm" color="secondary" on:click={() => row.editing = false}><Icon name="x-square" /></Button>
                        {:else}
                            <Button size="sm" color="danger" on:click={() => deleteRow(row.loan_id)}><Icon name="trash" /></Button>
                            <Button size="sm" color="primary" on:click={() => row.editing = true}><Icon name="pencil-square" /></Button>
                        {/if}
                    </td>
                    <td>{row.loan_id}</td>
                    <td>{row.loan_amount}</td>
                    <td>{row.start_date}</td>
                </tr>
                {/each}
            {/if}
            </tbody>
        </Datatable>
    </div>
    
    <aside>
        {#if $rows}
            <PaginationButtons id={'loan-table'}/>
            <PaginationRowCount id={'loan-table'}/>
        {/if}
    </aside>
</section>

<style>
    section{padding:24px 48px;background:#eee;border-radius:8px;}
    aside{display:flex;justify-content:space-between;padding:8px 16px;background:#fff;border-radius:8px;margin:8px 0;}
    div{padding:8px 0;height:400px;background:#fff;border-radius:8px;}
    td{text-align:center;padding:4px 8px;white-space:nowrap;}
</style>
