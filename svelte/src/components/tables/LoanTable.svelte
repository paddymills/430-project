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
    const alertMainPage = msg => dispatch('alert', { message: msg });

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
            loan_amount: null,
            interest_rate: null,
            amount_paid: null,
            number_of_payments: null,
            start_date: null,
            end_date: null
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

        alertMainPage("Loan Deleted.")
    }
    async function updateRow() {
        // call update on backend
        if (addMode) {
            // add
            alertMainPage("Loan Added.")
        } else {
            // update
            alertMainPage("Loan Updated.")
        }

        resetModals();
    }

    resetModals();

</script>

<section>
    <Modal isOpen={modals.edit} toggle={toggleEditModal}>
        <ModalHeader>{ addMode === true ? 'Add' : 'Edit' } Loan</ModalHeader>
        <ModalBody>
            <Form>
                <FormGroup>
                    <Label for="amount">Loan Amount</Label>
                    <Input id="amount" type="number" value={editrow.loan_amount} />
                </FormGroup>
                <FormGroup>
                    <Label for="rate">Interest Rate</Label>
                    <Input id="rate" type="number" value={editrow.interest_rate} />
                </FormGroup>
                <FormGroup>
                    <Label for="paid">Amount Paid</Label>
                    <Input id="paid" type="number" value={editrow.amount_paid} />
                </FormGroup>
                <FormGroup>
                    <Label for="payments">Number of Payments</Label>
                    <Input id="payments" type="number" value={editrow.number_of_payments} />
                </FormGroup>
                <FormGroup>
                    <Label for="start">Start Date</Label>
                    <Input id="start" type="date" value={editrow.start_date} />
                </FormGroup>
                <FormGroup>
                    <Label for="end">End Date</Label>
                    <Input id="end" type="date" value={editrow.end_date} />
                </FormGroup>
            </Form>
        </ModalBody>
        <ModalFooter>
            <Button color="primary" on:click={toggleUpdateModal}>Submit</Button>
            <Button color="secondary" on:click={resetModals}>Cancel</Button>
        </ModalFooter>
    </Modal>
    
    <!-- update confirmation -->
    <Modal size="sm" body header="Update loan?" isOpen={modals.update} toggle={toggleUpdateModal}>
        <Button color="primary" on:click={updateRow}>Yes</Button>
        <Button color="secondary" on:click={toggleUpdateModal}>No</Button>
    </Modal>

    <!-- delete confirmation -->
    <Modal size="sm" body header="Delete loan?" isOpen={modals.delete} toggle={toggleDeleteModal}>
        <Button color="primary" on:click={deleteRow}>Yes</Button>
        <Button color="secondary" on:click={toggleDeleteModal}>No</Button>
    </Modal>

    <aside>
        {#if $rows}
            <SearchInput id={'loan-table'}/>
        {/if}
        <Button size="sm" color="primary" on:click={showAddModal}><Icon name="plus-square" /> Add Loan</Button>
    </aside>

    <div>
        <Datatable {settings} {data} bind:dataRows={rows} id={'loan-table'}>
            <thead>
                <th></th>
                <th data-key="loan_id">#</th>
                <th data-key="loan_amount">Loan Amount</th>
                <th data-key="interest_rate">Rate</th>
                <th data-key="amount_paid">Paid</th>
                <th data-key="number_of_payments">Payments</th>
                <th data-key="start_date">Start</th>
                <th data-key="end_date">End</th>
            </thead>
            <tbody>
            {#if rows}
                {#each $rows as row}
                <tr>
                    <td>
                        <Button size="sm" color="danger" on:click={toggleDeleteModal}><Icon name="trash" /></Button>
                        <Button size="sm" color="primary" on:click={() => showEditModal(row)}><Icon name="pencil-square" /></Button>
                    </td>
                    <td>{ row.loan_id }</td>
                    <td>{ row.loan_amount.toLocaleString('en-US', { style: 'currency', currency: 'USD' }) }</td>
                    <td>{ row.interest_rate.toLocaleString('en-US', { style: 'percent', minimumFractionDigits: 2 }) }</td>
                    <td>{ row.amount_paid.toLocaleString('en-US', { style: 'currency', currency: 'USD' }) }</td>
                    <td>{ row.number_of_payments }</td>
                    <td>{ row.start_date }</td>
                    <td>{ row.end_date }</td>
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
