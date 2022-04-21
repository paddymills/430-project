<script lang="ts">
    import { Datatable, SearchInput, PaginationButtons, PaginationRowCount } from 'svelte-simple-datatables';
    import { Button, Icon } from 'sveltestrap';
    import { Modal, ModalHeader, ModalBody, ModalFooter } from 'sveltestrap';
    import { Form, FormGroup, Input, Label } from 'sveltestrap';
    import { createEventDispatcher } from 'svelte';

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

    const dispatch = createEventDispatcher();

	export let data;
    let rows;
    let editrow;
    let addMode = false;

    let modals;

    const toggleUpdateModal = () => (modals.update = !modals.update);
    const toggleDeleteModal = () => (modals.delete = !modals.delete);
    const toggleEditModal = () => (modals.edit = !modals.edit);

    function resetModals() {
        addMode = false;
        modals = {
            edit: false,
            update: false,
            delete: false
        };
    }
    
    function showAddModal() {
        addMode = true;
        editrow = {
            first_name: null,
            last_name: null,
            email: null,
            phone: null
        };
        toggleEditModal();
    }
    function showEditModal(row: any) {
        editrow = row;
        toggleEditModal();
    }
    async function deleteRow(row: any) {
        // call delete on backend

        resetModals()

        alertMainPage("Customer Deleted.")
    }
    async function updateRow() {
        // call update on backend
        if (addMode) {
            // add
            alertMainPage("Customer Added.")
        } else {
            // update
            alertMainPage("Customer Updated.")
        }

        resetModals();
    }

    function alertMainPage(msg: string) {
        dispatch('alert', { message: msg });
    }

    resetModals();

</script>

<section>
    <Modal isOpen={modals.edit} toggle={toggleEditModal}>
        <ModalHeader>{ addMode === true ? 'Add' : 'Edit' } Customer</ModalHeader>
        <ModalBody>
            <Form>
                <FormGroup>
                    <Label for="firstName">First Name</Label>
                    <Input id="firstName" value={editrow.first_name} />
                </FormGroup>
                <FormGroup>
                    <Label for="lastName">Last Name</Label>
                    <Input id="lastName" value={editrow.last_name} />
                </FormGroup>
                <FormGroup>
                    <Label for="email">Email</Label>
                    <Input id="email" value={editrow.email} />
                </FormGroup>
                <FormGroup>
                    <Label for="phone">Phone</Label>
                    <Input id="phone" value={editrow.phone} />
                </FormGroup>
            </Form>
        </ModalBody>
        <ModalFooter>
            <Button color="primary" on:click={toggleUpdateModal}>Submit</Button>
            <Button color="secondary" on:click={resetModals}>Cancel</Button>
        </ModalFooter>
    </Modal>
    
    <!-- update confirmation -->
    <Modal size="sm" body header="Update customer?" isOpen={modals.update} toggle={toggleUpdateModal}>
        <Button color="primary" on:click={updateRow}>Yes</Button>
        <Button color="secondary" on:click={toggleUpdateModal}>No</Button>
    </Modal>

    <!-- delete confirmation -->
    <Modal size="sm" body header="Delete customer?" isOpen={modals.delete} toggle={toggleDeleteModal}>
        <Button color="primary" on:click={deleteRow}>Yes</Button>
        <Button color="secondary" on:click={toggleDeleteModal}>No</Button>
    </Modal>

    <aside>
        {#if $rows}
            <SearchInput id={'customer-table'}/>
        {/if}
        <Button size="sm" color="primary" on:click={showAddModal}><Icon name="plus-square" /> Add Customer</Button>
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
                        <Button size="sm" color="danger" on:click={toggleDeleteModal}><Icon name="trash" /></Button>
                        <Button size="sm" color="primary" on:click={() => showEditModal(row)}><Icon name="pencil-square" /></Button>
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
