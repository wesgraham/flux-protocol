use super::*;

#[test]
fn test_liquidity_for_price() {
	testing_env!(get_context(carol(), current_block_timestamp()));	
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 2, outcome_tags(0), categories(), market_end_timestamp());

	contract.place_order(0, 0, 6000, 50); // canceled before fill
	contract.place_order(0, 0, 6000, 50); // canceled before fill
	contract.place_order(0, 0, 6000, 20);
	contract.place_order(0, 0, 8000, 20); // canceled before fill

	let liquidity_60 = contract.get_liquidity_for_price(0, 0, 60);
	let liquidity_50 = contract.get_liquidity_for_price(0, 0, 50);
	let liquidity_20 = contract.get_liquidity_for_price(0, 0, 20);
	
	assert_eq!(liquidity_60, 0);
	assert_eq!(liquidity_50, 12000);
	assert_eq!(liquidity_20, 14000);

	contract.cancel_order(0,0,0);
	contract.cancel_order(0,0,1);
	contract.cancel_order(0,0,3);

	let liquidity_50 = contract.get_liquidity_for_price(0, 0, 50);
	let liquidity_20 = contract.get_liquidity_for_price(0, 0, 20);

	assert_eq!(liquidity_50, 0);
	assert_eq!(liquidity_20, 6000);

	contract.place_order(0, 1, 8000, 80);

	let liquidity_20 = contract.get_liquidity_for_price(0, 0, 20);
	let liquidity_80 = contract.get_liquidity_for_price(0, 1, 80);

	assert_eq!(liquidity_20, 4000);
	assert_eq!(liquidity_80, 0);
}

// #[test]
// fn test_valid_binary_market_depth() {
// 	testing_env!(get_context(carol(), current_block_timestamp()));	
// 	let mut contract = Markets::default();
// 	contract.claim_fdai();
// 	contract.create_market("Hi!".to_string(), empty_string(), 2, outcome_tags(0), categories(), market_end_timestamp());
// 	contract.place_order(0, 0, 6000, 60);
// 	contract.place_order(0, 0, 7000, 70);

// 	testing_env!(get_context(alice(), current_block_timestamp()));
// 	contract.claim_fdai();
// 	contract.place_order(0, 1, 1000, 20);
// 	contract.place_order(0, 1, 1000, 10);

// 	let depth_0 = contract.get_liquidity(0, 0, 1000, 75); // Returns (max_price_payed, number of shares that can be purached, max_spend at max_price)
// 	let depth_1 = contract.get_liquidity(0, 1, 1000, 11);

//     assert_eq!(depth_0, (0 , 0, 0));
// 	assert_eq!(depth_1, (0, 0, 0));
// }


// fn test_valid_categorical_market_depth() {
// }
