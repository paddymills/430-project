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

	export let data;
    let rows;
    let customers;
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
            loan: {
                loan_id: 999,
                customer_id: null,
                loan_amount: null,
                interest_rate: null,
                amount_paid: null,
                number_of_payments: null,
                start_date: null,
                end_date: null,
            },

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

    function prepareEditRowExport() {
        // console.log(editrow);

        switch (editrow.child) {
            case 'auto':
                editrow.auto.loan_id = editrow.loan.loan_id;
                editrow.mortgage = null;
                editrow.personal = null;
                
                break;
            case 'mortgage':
                editrow.mortgage.loan_id = editrow.loan.loan_id;
                editrow.auto = null;
                editrow.personal = null;

                break;
            case 'personal':
                editrow.personal.loan_id = editrow.loan.loan_id;
                editrow.auto = null;
                editrow.mortgage = null;
                break;
        
            default:
                break;
        }
    }
    
    function showAddModal() {
        addMode = true;
        initEditRow();
        toggleEditModal();
    }
    
    async function showEditModal(row: any) {
        // initEditRow();
        editrow.loan = row;
        await getLoanData(editrow.loan.loan_id)
            .then((result: any) => {
                editrow.child = result.child;
                switch (editrow.child) {
                    case 'auto':
                        editrow.auto = result.auto;
                        break;
                    case 'mortgage':
                        editrow.mortgage = result.mortgage;
                        break;
                    case 'personal':
                        editrow.personal = result.personal;
                        break;
                
                    default:
                        break;
                }
            });

        toggleEditModal();
    }
    function showDeleteModal(row: any) {
        editrow.loan = row;
        toggleDeleteModal();
    }
    async function deleteRow() {
        invoke('delete_loan', { id: editrow.loan.loan_id })
            .then((result: [any]) => {
                alertMainPage("Loan deleted.")
                dispatch('refresh');
            })
            .catch((error) => {
                console.log(error);
                dispatch('alert', { message: error, isError: true });
            });

        resetModals();
    }
    async function addRow() {
        prepareEditRowExport();

        invoke('add_loan', { data: editrow })
            .then((result: [any]) => {
                alertMainPage("Loan Added.")
                dispatch('refresh');
            })
            .catch((error) => {
                console.log(error);
                dispatch('alert', { message: error, isError: true });
            });

        resetModals();
    }
    async function updateRow() {
        prepareEditRowExport();
        // console.log(editrow);

        invoke('update_loan', { data: editrow })
            .then((result: [any]) => {
                alertMainPage("Loan Updated.");
                dispatch('refresh');
            })
            .catch((error) => {
                console.log(error);
                dispatch('alert', { message: error, isError: true });
            });

        resetModals();
    }

    async function getLoanData(loan_id: number) {
        popoverTitle = "Loan Data";

        return await invoke('get_loan', { id: loan_id })
            .then((result: any) => {
                // console.log(result);
                popoverTitle = result.child + ' Loan';

                return result;
            })
            .catch((error) => {
                console.log(error);

                return error
            });
    }

    async function loadCustomers() {
		return await invoke('get_customers', {})
			.then((result: [any]) => {
                customers = result;
			});
	}

    async function getCustomerData(customer_id: number) {
        popoverTitle = "Customer Data";

        return await invoke('get_customer', { id: customer_id })
			.then((result: any) => {
				// console.log(result);
                popoverTitle = result.first_name + ' ' + result.last_name;

				return result;
			})
			.catch((error) => {
				console.log(error);

				return error
			});
    }

    resetModals();
    initEditRow();
    loadCustomers();

</script>

<section>
    <Modal isOpen={modals.edit} toggle={toggleEditModal}>
        <ModalHeader>{ addMode === true ? 'Add' : 'Edit' } Loan</ModalHeader>
        <ModalBody class="d-flex flex-row">
            {#key editrow.child}
            <Form class="m-3">
                <FormGroup>
                    <Label for="cid">Customer ID</Label>
                    <Input id="cid" type="select" bind:value={ editrow.loan.customer_id }>
                        {#each customers as cust}
                            <option value={cust.customer_id}>{ cust.customer_id }: { cust.first_name } { cust.last_name }</option>
                        {/each}
                    </Input>
                </FormGroup>
                <FormGroup>
                    <Label for="amount">Loan Amount</Label>
                    <Input id="amount" type="number" bind:value={editrow.loan.loan_amount} />
                </FormGroup>
                <FormGroup>
                    <Label for="rate">Interest Rate</Label>
                    <Input id="rate" type="number" bind:value={editrow.loan.interest_rate} />
                </FormGroup>
                <FormGroup>
                    <Label for="paid">Amount Paid</Label>
                    <Input id="paid" type="number" bind:value={editrow.loan.amount_paid} />
                </FormGroup>
                <FormGroup>
                    <Label for="payments">Number of Payments</Label>
                    <Input id="payments" type="number" bind:value={editrow.loan.number_of_payments} />
                </FormGroup>
                <FormGroup>
                    <Label for="start">Start Date</Label>
                    <Input id="start" type="date" bind:value={editrow.loan.start_date} />
                </FormGroup>
                <FormGroup>
                    <Label for="end">End Date</Label>
                    <Input id="end" type="date" bind:value={editrow.loan.end_date} />
                </FormGroup>
            </Form>
            <Form class="m-3">
                <FormGroup>
                    <Label for="type">Loan Type</Label>
                    <Input id="type" type="select" bind:value={ editrow.child }>
                        <option value="auto">Auto</option>
                        <option value="mortgage">Mortgage</option>
                        <option value="personal">Personal</option>
                    </Input>
                </FormGroup>
                {#if editrow.child == 'auto'}
                    <FormGroup><Label for="make">Make</Label><Input id="make" bind:value={ editrow.auto.make } /></FormGroup>
                    <FormGroup><Label for="model">Model</Label><Input id="model" bind:value={ editrow.auto.model } /></FormGroup>
                    <FormGroup><Label for="year">Year</Label><Input id="year" bind:value={ editrow.auto.year } /></FormGroup>
                    <FormGroup><Label for="vin">VIN</Label><Input id="vin" bind:value={ editrow.auto.vin } /></FormGroup>
                {:else if editrow.child == 'mortgage'}
                    <FormGroup><Label for="addr">Address</Label><Input id="addr" bind:value={ editrow.mortgage.address } /></FormGroup>
                    <FormGroup><Label for="area">Area</Label><Input id="area" bind:value={ editrow.mortgage.area } /></FormGroup>
                    <FormGroup><Label for="bedrooms">Bedrooms</Label><Input id="bedrooms" type="number" bind:value={ editrow.mortgage.num_bedrooms } /></FormGroup>
                    <FormGroup><Label for="bathrooms">Bathrooms</Label><Input id="bathrooms" type="number" bind:value={ editrow.mortgage.num_bathrooms } /></FormGroup>
                    <FormGroup><Label for="price">Price</Label><Input id="price" type="number" bind:value={ editrow.mortgage.price } /></FormGroup>
                {:else if editrow.child == 'personal'}
                    <FormGroup><Label for="purpose">Purpose</Label><Input id="purpose" type="textarea" bind:value={ editrow.personal.purpose } /></FormGroup>
                {/if}
            </Form>
            {/key}
        </ModalBody>
        <ModalFooter>
            <Button color="primary" on:click={() => ( addMode ? addRow() : toggleUpdateModal())}>Submit</Button>
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
                        <Button size="sm" color="danger" on:click={() => showDeleteModal(row)}><Icon name="trash" /></Button>
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
                <Popover class="text-capitalize" trigger="hover" target={ `loan-${rowId}` } title={ popoverTitle }>
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
