use std::{vec, cmp};
fn main() {

	let firstVec = [11, 12, 13, 14];
	let mut secondVec = [21, 22, 23, 24, 25];
	let thirdVec = [31, 32, 33, 34, 35, 36];
	
	unsafe{
		secondVec.unsafe_set(5, 20);
	}

	/*
	let firstVec = ['w', 'x', 'y', 'z'];
	let secondVec = ['a', 'b', 'c', 'd', 'e'];
	let thirdVec = ['f', 'g', 'h'];
	*/

	unsafe {
		
		println("\n==ORIGINAL\n");

		for i in range(0u, 16u) {
			let val = vec::raw::get(thirdVec, i);
			println!("{:}", val);
		}

		println("\n");
		println("   T\tS\tF");
		for i in range(0, cmp::max(cmp::max(firstVec.len(), secondVec.len()), thirdVec.len())) {

			let mut firstVecPrint = ~"";
			let mut secondVecPrint = ~"";
			let mut thirdVecPrint = ~"";

			if(i < firstVec.len()) {
				firstVecPrint = (firstVec[i] as int).to_str();
			}
			if(i < secondVec.len()) {
				secondVecPrint = (secondVec[i] as int).to_str();
			}
			if(i < thirdVec.len()) {
				thirdVecPrint = (thirdVec[i] as int).to_str();
			}

			println!("{:}| {:}\t{:}\t{:}", i, thirdVecPrint, secondVecPrint, firstVecPrint);

		}

		
		println("\n==MODIFIED\n");

		vec::raw::copy_memory(secondVec, thirdVec, 6);
		for i in range(0u, 16u) {
			let val = vec::raw::get(thirdVec, i);
			println!("{:}", val);
		}

		println("\n");
		println("   T\tS\tF");
		for i in range(0, cmp::max(cmp::max(firstVec.len(), secondVec.len()), thirdVec.len())) {

			let mut firstVecPrint = ~"";
			let mut secondVecPrint = ~"";
			let mut thirdVecPrint = ~"";

			if(i < firstVec.len()) {
				firstVecPrint = (firstVec[i] as int).to_str();
			}
			if(i < secondVec.len()) {
				secondVecPrint = (secondVec[i] as int).to_str();
			}
			if(i < thirdVec.len()) {
				thirdVecPrint = (thirdVec[i] as int).to_str();
			}

			println!("{:}| {:}\t{:}\t{:}", i, thirdVecPrint, secondVecPrint, firstVecPrint);

		}

		println("\n=====================\n");

		let firstVecPtr = vec::raw::to_ptr(firstVec);
		let secondVecPtr = vec::raw::to_ptr(secondVec);
		let thirdVecPtr = vec::raw::to_ptr(thirdVec);


		println!("firstVec addr: {:}", firstVecPtr);
		println!("secondVec addr: {:}", secondVecPtr);
		println!("thirdVec addr: {:}", thirdVecPtr);


		let val = vec::raw::get(secondVec, -1u);
		println!("secondVec[-1] = {:}", val);
	}
}