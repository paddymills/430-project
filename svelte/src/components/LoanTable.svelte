<script lang="ts">
    import { Datatable, SearchInput, PaginationButtons, PaginationRowCount } from 'svelte-simple-datatables'
    import { Button, Icon, Modal, ModalHeader, ModalBody, ModalFooter } from 'sveltestrap';

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

    let rows;
	export let data;

    let editId;
    let editModalIsOpen = false;
    let modalTitle = "Edit Row";
    let modalText = "put form here";
    const toggleModal = () => (editModalIsOpen = !editModalIsOpen);

    function showModal(id: Number) {
        editId = id;
        modalText = "edit: " + id;
        toggleModal();
    }
    async function deleteRow(id: Number) {
        modalText = "delete: " + id;
        toggleModal();
    }
    async function updateRow() {
        modalText = "update: " + editId;
    }

</script>

<section>
    <Modal isOpen={editModalIsOpen} toggle={toggleModal}>
        <ModalHeader>{ modalTitle }</ModalHeader>
        <ModalBody>
            { modalText }
        </ModalBody>
        <ModalFooter>
            <Button color="primary" on:click={() => updateRow()}><Icon name="check-square" /> Submit</Button>
            <Button color="secondary" on:click={toggleModal}><Icon name="x-square" /> Cancel</Button>
            <!-- <Button color="primary" on:click={toggleModal}>Submit</Button>
            <Button color="primary" on:click={toggleModal}>Cancel</Button> -->
        </ModalFooter>
    </Modal>
    <aside>
        {#if $rows}
            <SearchInput id={'loan-table'}/>
        {/if}
    </aside>

    <div>
        <Datatable {settings} {data} bind:dataRows={rows} id={'loan-table'}>
            <thead>
                <th></th>
                <th data-key="loan_id">#</th>
                <th data-key="loan_amount">Loan Amount</th>
                <th data-key="start_date">Start Date</th>
            </thead>
            <tbody>
            {#if rows}
                {#each $rows as row}
                <tr>
                    <td>
                        <Button size="sm" color="danger" on:click={() => deleteRow(row.loan_id)}><Icon name="trash" /></Button>
                        <Button size="sm" color="primary" on:click={() => showModal(row.loan_id)}><Icon name="pencil-square" /></Button>
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
