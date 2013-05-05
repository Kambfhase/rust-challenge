/** Sort.rs
 * by @Kambfhase
 * rust 0.6
 */
extern mod std;
use core::io::{Reader,ReaderUtil};
use core::str;

fn main() {

	let mut numOfTests : int;

	let mut line : ~str;
	let mut list : ~[int];

	line = io::stdin().read_line();

	match int::from_str(line) {
		Some( number) => numOfTests = number,
		None => numOfTests = -1 // should not happen :)
	}

	while numOfTests > 0 {
		line = io::stdin().read_line();

		list = vec::build( |f| {
			str::each_word(line, |word| {
				let mut n : int;
				match int::from_str(word) {
					Some( number) => n = number,
					None => n = -1
				};
				f(n);
				true
			})
		});

		printlist(qsort(list.tail()));

		numOfTests -= 1;
	}
}


fn printlist (list: &[int]) -> () {
	let foo = list.map(|e| {
		fmt!("%d", *e)
	});
	let bar = str::connect(foo, ~" ");

	print(fmt!("%s\n", bar));
}

/** Quick Sort
 *
 * This is an oversimplified quick sort. It might not be as fast as the
 * sort in the standard library, but it is easy to understand. The "not
 * as fast" is just a constant factor, because this code is O(n log n)
 * in the worst case.
 */
fn qsort( L : &[int]) -> ~[int] {
	use core::rand::RngUtil;
	if L.len() == 0 {
		return ~[]
	}
	if L.len() == 1 {
		return ~[L[0]]
	}
	let pivot = L[rand::Rng().gen_int_range(0, L.len() as int)];
	let lower = L.filtered(|e| *e < pivot);
	let equal = L.filtered(|e| *e == pivot);
	let higher = L.filtered(|e| *e > pivot);
	
	return qsort( lower) + equal + qsort( higher);
}
