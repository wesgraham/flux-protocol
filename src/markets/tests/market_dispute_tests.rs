use super::*;

#[test]
fn test_valid_market_dispute() {
	testing_env!(get_context(carol(), current_block_timestamp()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(), market_end_timestamp(), 4, 2, "test".to_string());

	contract.place_order(0, 0, 7000, 70);

	testing_env!(get_context(alice(), current_block_timestamp()));
	contract.claim_fdai();
	contract.place_order(0, 1, 1000, 10);
	contract.place_order(0, 2, 2000, 20);

	testing_env!(get_context(alice(), market_end_timestamp()));
	contract.resolute(0, Some(1));
	testing_env!(get_context(carol(), market_end_timestamp()+5));
	contract.dispute(0, Some(0), 160);
    testing_env!(get_context(alice(), market_end_timestamp()+15));
	contract.finalize(0);

	let open_orders_0 = contract.get_open_orders(0, 0);
	let open_orders_1 = contract.get_open_orders(0, 1);
	let open_orders_2 = contract.get_open_orders(0, 2);

	assert_eq!(open_orders_0.len(), 0);
	assert_eq!(open_orders_1.len(), 0);
	assert_eq!(open_orders_2.len(), 0);

	let filled_orders_0 = contract.get_filled_orders(0, 0);
	let filled_orders_1 = contract.get_filled_orders(0, 1);
	let filled_orders_2 = contract.get_filled_orders(0, 2);

	assert_eq!(filled_orders_0.len(), 1);
	assert_eq!(filled_orders_1.len(), 1);
	assert_eq!(filled_orders_2.len(), 1);

	let claimable_carol = contract.get_claimable(0, carol()) ;
	let claimable_alice = contract.get_claimable(0, alice()) ;

	assert_eq!(claimable_carol, 10160);
	assert_eq!(claimable_alice, 0);
}
