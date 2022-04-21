<script lang="ts">
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import { Datatable, SearchInput, PaginationButtons, PaginationRowCount } from 'svelte-simple-datatables'

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

	export let username;
	let data, rows;

	onMount(load_loans);

	async function load_loans() {
		await invoke('get_cust_loans', { user: username })
			.then((result: [any]) => {
				// console.log(result);

				data = result.map(x => {
					x.editing = false;

					return x;
				});

				console.log(data);
			})
			.catch((error) => console.log(error));
	}
</script>

<div transition:slide={{delay: 250, duration: 500}}>
	<section>
		<aside>
			{#if $rows}
				<SearchInput id={'loan-table'}/>
			{/if}
		</aside>
	
		<div>
			<Datatable {settings} {data} bind:dataRows={rows} id={'loan-table'}>
				<thead>
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
</div>

<style>
    section{padding:24px 48px;background:#eee;border-radius:8px;}
    aside{display:flex;justify-content:space-between;padding:8px 16px;background:#fff;border-radius:8px;margin:8px 0;}
    div{padding:8px 0;height:400px;background:#fff;border-radius:8px;}
    td{text-align:center;padding:4px 8px;white-space:nowrap;}
</style>