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

	export let data;
    let rows;

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
        </ModalFooter>
    </Modal>
    <aside>
        {#if $rows}
            <SearchInput id={'customer-table'}/>
        {/if}
    </aside>

    <div>
        <Datatable {settings} {data} bind:dataRows={rows} id={'customer-table'}>
            <thead>
                <th></th>
                <th data-key="customer_id">#</th>
                <th data-key="(row) => row.first_name + ' ' + row.last_name">Name</th>
                <th data-key="email">Email</th>
                <th data-key="phone">Phone</th>
            </thead>
            <tbody>
            {#if rows}
                {#each $rows as row}
                <tr>
                    <td>
                        <Button size="sm" color="danger" on:click={() => deleteRow(row.customer_id)}><Icon name="trash" /></Button>
                        <Button size="sm" color="primary" on:click={() => showModal(row.customer_id)}><Icon name="pencil-square" /></Button>
                    </td>
                    <td>{ row.customer_id }</td>
                    <td>{ row.first_name } { row.last_name }</td>
                    <td>{ row.email }</td>
                    <td>{ row.phone }</td>
                </tr>
                {/each}
            {/if}
            </tbody>
        </Datatable>
    </div>
    
    <aside>
        {#if $rows}
            <PaginationButtons id={'customer-table'}/>
            <PaginationRowCount id={'customer-table'}/>
        {/if}
    </aside>
</section>

<style>
    section{padding:24px 48px;background:#eee;border-radius:8px;}
    aside{display:flex;justify-content:space-between;padding:8px 16px;background:#fff;border-radius:8px;margin:8px 0;}
    div{padding:8px 0;height:400px;background:#fff;border-radius:8px;}
    td{text-align:center;padding:4px 8px;white-space:nowrap;}
</style>
