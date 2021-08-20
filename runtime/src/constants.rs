pub mod currency {
	pub type Balance = u128;

	pub const WEI: Balance = 1;
	pub const KILOWEI: Balance = 1_000;
	pub const MEGAWEI: Balance = 1_000_000;
	pub const GIGAWEI: Balance = 1_000_000_000;
	pub const MICROTRAC: Balance = 1_000_000_000_000;
	pub const MILLITRAC: Balance = 1_000_000_000_000_000;
	pub const TRAC: Balance = 1_000_000_000_000_000_000;
	pub const KILOTRAC: Balance = 1_000_000_000_000_000_000_000;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 15 * MICROTRAC + (bytes as Balance) * 6 * MICROTRAC
	}
}

pub mod time {
	pub type Moment = u64;
	pub type BlockNumber = u32;

	pub const SECS_PER_BLOCK: Moment = 12;
	pub const MILLISECS_PER_BLOCK: Moment = SECS_PER_BLOCK * 1000;

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

}

