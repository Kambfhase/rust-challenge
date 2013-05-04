/* this code was use to create the test cases */
extern mod std;
use core::str;

fn main() {

	let args : ~[~str] = os::args();
	let l = args.len();
	let args = args.slice(1,l).map(|s| core::int::from_str(*s));

	let mut n : int  = 10;
	let mut m : int = 100;

	if l >= 2 && args[0] != None {
		n = args[0].unwrap()
	}
	if l >= 3 && args[1] != None {
		m = args[1].unwrap()
	}

	let list = ~[n] + gen(n, m);

	printlist(list);
}

fn printlist (list: &[int]) -> () {
	let foo = list.map(|e| {
		fmt!("%d", *e)
	});
	let bar = str::connect(foo, ~" ");

	print(fmt!("%s\n", bar));
}

fn gen( n: int, m : int) -> ~[int] {
	use core::rand::RngUtil;

	vec::build_sized(n as uint, |f| {
		let mut i = 0;
		while i < n {
			f( rand::Rng().gen_int_range(0, m));
			i += 1;
		}
	})
}
