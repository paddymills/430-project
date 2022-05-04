<script lang="ts">
    import { Datatable, SearchInput, PaginationButtons, PaginationRowCount } from 'svelte-simple-datatables';
    import { Button, Icon } from 'sveltestrap';
    import { Modal, ModalHeader, ModalBody, ModalFooter } from 'sveltestrap';
    import { Form, FormGroup, Input, Label } from 'sveltestrap';
    import { createEventDispatcher } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';

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
    function showDeleteModal(row: any) {
        editrow = row;
        toggleDeleteModal();
    }

    async function deleteRow() {
        // call delete on backend
        invoke('delete_customer', { id: editrow.customer_id })
            .then((result: [any]) => {
                alertMainPage("Customer Deleted.")
                dispatch('refresh');
			})
			.catch((error) => {
                console.log(error);
                dispatch('alert', { message: error, isError: true });
			});
            
        resetModals();
    }
    async function addRow() {
        invoke('add_customer', {
                fname: editrow.first_name,
                lname: editrow.last_name,
                email: editrow.email,
                phone: editrow.phone
            })
            .then((result: [any]) => {
                alertMainPage("Customer Added.")
                dispatch('refresh');
            })
            .catch((error) => {
                console.log(error);
                dispatch('alert', { message: error, isError: true });
            });


        resetModals();
        dispatch('refresh');
    }
    async function updateRow() {
        // call update on backend
        invoke('edit_customer', {
                id: editrow.customer_id,
                fname: editrow.first_name,
                lname: editrow.last_name,
                email: editrow.email,
                phone: editrow.phone
            })
            .then((result: [any]) => {
                alertMainPage("Customer Updated.")
                dispatch('refresh');
            })
            .catch((error) => {
                console.log(error);
                dispatch('alert', { message: error, isError: true });
            });

        resetModals();
        dispatch('refresh');
    }

    function alertMainPage(msg: string) {
        dispatch('alert', { message: msg, isError: false });
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
                    <Input id="firstName" bind:value={editrow.first_name} />
                </FormGroup>
                <FormGroup>
                    <Label for="lastName">Last Name</Label>
                    <Input id="lastName" bind:value={editrow.last_name} />
                </FormGroup>
                <FormGroup>
                    <Label for="email">Email</Label>
                    <Input id="email" bind:value={editrow.email} />
                </FormGroup>
                <FormGroup>
                    <Label for="phone">Phone</Label>
                    <Input id="phone" bind:value={editrow.phone} />
                </FormGroup>
            </Form>
        </ModalBody>
        <ModalFooter>
            <Button color="primary" on:click={() => ( addMode ? addRow() : toggleUpdateModal())}>Submit</Button>
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
                        <Button size="sm" color="danger" on:click={() => showDeleteModal(row)}><Icon name="trash" /></Button>
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
