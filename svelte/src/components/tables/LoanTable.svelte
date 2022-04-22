<script lang="ts">
    import { Datatable, SearchInput, PaginationButtons, PaginationRowCount } from 'svelte-simple-datatables';
    import { Button, Icon, Popover, Spinner } from 'sveltestrap';
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
    const alertMainPage = msg => dispatch('alert', { message: msg });
    const reloadData = () => dispatch('refresh', {});

	export let data;
    let rows;
    let editrow;
    let addMode = false;

    let modals;
    let popoverTitle;

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

    function initEditRow() {
        editrow = {
            loan_id: null,
            customer_id: null,
            loan_amount: null,
            interest_rate: null,
            amount_paid: null,
            number_of_payments: null,
            start_date: null,
            end_date: null,

            child: 'auto',
            auto: {
                make: null,
                model: null,
                year: null,
                vin: null
            },
            mortgage: {
                address: null,
                area: null,
                num_bedrooms: null,
                num_bathrooms: null,
                price: null
            },
            personal: {
                purpose: null
            }
        };
    }
    
    function showAddModal() {
        addMode = true;
        initEditRow();
        toggleEditModal();
    }
    
    async function showEditModal(row: any) {
        initEditRow();
        editrow = row;
        await getLoanData(editrow.loan_id)
            .then((result: any) => {
                editrow.child = result.child;
                if (editrow.auto) { editrow.auto = result.auto; }
                if (editrow.mortgage) { editrow.mortgage = result.mortgage; }
                if (editrow.personal) { editrow.personal = result.personal; }
            });

        toggleEditModal();
    }
    async function deleteRow(row: any) {
        // call delete on backend

        resetModals();

        alertMainPage("Loan Deleted.");
        reloadData();
    }
    async function updateRow() {
        // call update on backend
        if (addMode) {
            // add
            alertMainPage("Loan Added.");
        } else {
            // update
            alertMainPage("Loan Updated.");
        }

        resetModals();
        reloadData();
    }

    async function getLoanData(loan_id: number) {
        popoverTitle = "Loan Data";

        return await invoke('get_loan', { id: loan_id })
            .then((result: any) => {
                console.log(result);
                popoverTitle = result.child.charAt(0).toUpperCase() + result.child.slice(1) + ' Loan';

                return result;
            })
            .catch((error) => {
                console.log(error);

                return error
            });
    }

    async function getCustomerData(customer_id: number) {
        popoverTitle = "Customer Data";

        return await invoke('get_customer', { id: customer_id })
			.then((result: any) => {
				console.log(result);
                popoverTitle = result.first_name + ' ' + result.last_name;

				return result;
			})
			.catch((error) => {
				console.log(error);

				return error
			});
    }

    resetModals();

</script>

<section>
    <Modal isOpen={modals.edit} toggle={toggleEditModal}>
        <ModalHeader>{ addMode === true ? 'Add' : 'Edit' } Loan</ModalHeader>
        <ModalBody>
            {#key editrow.child}
            <Form>
                <FormGroup>
                    <Label for="cid">Customer ID</Label>
                    <Input id="cid" type="number" value={editrow.customer_id} />
                </FormGroup>
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
                <FormGroup>
                    <Label for="type">Loan Type</Label>
                    <Input id="type" type="select" bind:value={ editrow.child }>
                        <option value="auto">Auto</option>
                        <option value="mortgage">Mortgage</option>
                        <option value="personal">Personal</option>
                    </Input>
                </FormGroup>
                {#if editrow.child == 'auto'}
                    <FormGroup><Label for="make">Make</Label><Input id="make" value={ editrow.auto.make } /></FormGroup>
                    <FormGroup><Label for="model">Model</Label><Input id="model" value={ editrow.auto.model } /></FormGroup>
                    <FormGroup><Label for="year">Year</Label><Input id="year" value={ editrow.auto.year } /></FormGroup>
                    <FormGroup><Label for="vin">VIN</Label><Input id="vin" value={ editrow.auto.vin } /></FormGroup>
                {:else if editrow.child == 'mortgage'}
                    <FormGroup><Label for="addr">Address</Label><Input id="addr" value={ editrow.mortgage.address } /></FormGroup>
                    <FormGroup><Label for="area">Area</Label><Input id="area" value={ editrow.mortgage.area } /></FormGroup>
                    <FormGroup><Label for="bedrooms">Bedrooms</Label><Input id="bedrooms" type="number" value={ editrow.mortgage.num_bedrooms } /></FormGroup>
                    <FormGroup><Label for="bathrooms">Bathrooms</Label><Input id="bathrooms" type="number" value={ editrow.mortgage.num_bathrooms } /></FormGroup>
                    <FormGroup><Label for="price">Price</Label><Input id="price" type="number" value={ editrow.mortgage.price } /></FormGroup>
                {:else if editrow.child == 'personal'}
                    <FormGroup><Label for="purpose">Purpose</Label><Input id="purpose" type="textarea" value={ editrow.personal.purpose } /></FormGroup>
                {/if}
            </Form>
            {/key}
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
                <th data-key="customer_id">Customer</th>
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
                {@const rowId = Math.random().toString(36).slice(-6)}
                <tr>
                    <td>
                        <Button size="sm" color="danger" on:click={toggleDeleteModal}><Icon name="trash" /></Button>
                        <Button size="sm" color="primary" on:click={() => showEditModal(row)}><Icon name="pencil-square" /></Button>
                    </td>
                    <td id={ `loan-${rowId}` }>{ row.loan_id }</td>
                    <td id={ `cust-${rowId}` }>{ row.customer_id }</td>
                    <td>{ row.loan_amount.toLocaleString('en-US', { style: 'currency', currency: 'USD' }) }</td>
                    <td>{ row.interest_rate.toLocaleString('en-US', { style: 'percent', minimumFractionDigits: 2 }) }</td>
                    <td>{ row.amount_paid.toLocaleString('en-US', { style: 'currency', currency: 'USD' }) }</td>
                    <td>{ row.number_of_payments }</td>
                    <td>{ row.start_date }</td>
                    <td>{ row.end_date }</td>
                </tr>
                <Popover trigger="hover" target={ `loan-${rowId}` } title={ popoverTitle }>
                    {#await getLoanData(row.loan_id)}
                        <Spinner color="primary" />
                    {:then loan}
                        {#if loan.child == 'auto'}
                            <p>Make: { loan.auto.make }</p>
                            <p>Model: { loan.auto.model }</p>
                            <p>Year: { loan.auto.year }</p>
                            <p>VIN: { loan.auto.vin }</p>
                        {:else if loan.child == 'mortgage'}
                            <p>Address: { loan.mortgage.address }</p>
                            <p>Area: { loan.mortgage.area }</p>
                            <p>Bedrooms: { loan.mortgage.num_bedrooms }</p>
                            <p>Bathrooms: { loan.mortgage.num_bathrooms }</p>
                            <p>Price: { loan.mortgage.price }</p>
                        {:else if loan.child == 'personal'}
                            <p>{ loan.personal.purpose }</p>
                        {:else}
                            <p>Unmatched loan type: { loan.child }</p>
                        {/if}
                    {:catch err}
                        <p>error locating customer data: { err }</p>
                    {/await}
                </Popover>
                <Popover trigger="hover" target={ `cust-${rowId}` } title={ popoverTitle }>
                    {#await getCustomerData(row.customer_id)}
                        <Spinner color="primary" />
                    {:then cust}
                        <p>Email: {cust.email}</p>
                        <p>Phone: {cust.phone}</p>
                    {:catch err}
                        <p>error locating customer data: { err }</p>
                    {/await}
                </Popover>
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
