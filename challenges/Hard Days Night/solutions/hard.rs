/** hard.rs
 * by @Kambfhase
 * rust 0.6
 */
extern mod std;
use core::io::{ReaderUtil};
use core::str;
use core::hashmap::linear;

fn main() {

	let mut numOfTests : int;
	match int::from_str(io::stdin().read_line()) {
		Some( number) => numOfTests = number,
		None => numOfTests = -1 // should not happen :)
	}

	while numOfTests > 0 {

		let numOfFriends = int::from_str(io::stdin().read_line()).get();
		let friends = vec::build_sized( numOfFriends as uint, |f| {
			for iter::repeat(numOfFriends as uint) || {
				f(io::stdin().read_line());
			}
		});

		let numOfPlaces = int::from_str(io::stdin().read_line()).get();
		let places = vec::build_sized( numOfPlaces as uint, |f| {
			for int::range(0 , numOfPlaces) |_| {
				f(io::stdin().read_line());
			}
		});

		let joined = friends + places;
		let size = joined.len();
		let INF = int::max_value /2;

		let mut matrix = @[] + vec::build_sized( size, |f| {
			do iter::repeat(size) {
				let mut inner = @[] + vec::build_sized( size, |g| {
					do iter::repeat(size)  {
						g(@mut INF);
						true
					}
				});
				f(inner);
				true
			}
		});

		let map = @mut linear::LinearMap::new();
		for vec::eachi(joined) |i, &name| {
			map.insert(name, i);
		}

		let numOfEdges = int::from_str(io::stdin().read_line()).get();
		do iter::repeat(numOfEdges as uint) {
			let name1 = io::stdin().read_line();
			let name2 = io::stdin().read_line();
			let dist = int::from_str(io::stdin().read_line()).get();

			// fetch the offset of the names
			let i = map.get(&name1);
			let j = map.get(&name2);

			// all edges are bidirectional
			*matrix[*i][*j] = dist;
			*matrix[*j][*i] = dist;
			true
		}

		// set the main diagonal to zero
		for int::range(0 , size as int) |i| {
			*matrix[i][i] = 0;
			*matrix[i][i] = 0;
		}

		floydwarshall(matrix);

		// sum all distances from an point to all friends, but not places
		let sums = matrix.map(|row|{
			row.slice(0, numOfFriends as uint).foldl(0, |a:&int, b|{
				// we have to be careful with an overflow here
				if *a >= INF || **b >= INF {
					INF
				} else {
					a + **b 
				}
			})
		});

		let mut min = INF;
		let mut minIndex = -1;
		for vec::eachi(sums) |i, &sum| {
			if( sum < min){
				min = sum;
				minIndex = i;
			}
		}

		print(fmt!("%s\n", joined[minIndex]));

		numOfTests -= 1;
	}
}


fn printlist (list: @[@mut int]) -> () {
	let foo = list.map(|&e| {
		fmt!("%2d", *e)
	});
	let bar = str::connect(foo, ~" ");

	print(fmt!("%s\n", bar));
}

fn printmatrix ( m: @[@[@mut int]]) {
	for vec::each(m) |&list| {
		printlist(list)
	};
	print("\n");
}

fn floydwarshall ( m: @[@[@mut int]]) {
	let n = m.len() as int;
	for int::range(0 , n) |k| {
		for int::range(0 , n) |i| {
			for int::range(0 , n) |j| {
				let mut dist = (*m[i][k] + *m[k][j]);

				if dist < *m[i][j] {
					*m[i][j] = dist;
					*m[j][i] = dist;
				}
			}
		}
	}
}
