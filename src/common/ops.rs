/*****************************************************
             PROJECT  : hpc_allocator_rust
             VERSION  : 0.1.0-dev
             DATE     : 05/2018
             AUTHOR   : Valat Sébastien
             LICENSE  : CeCILL-C
*****************************************************/

///This module implement basic operations which might be used
///everywhere in the code.

<<<<<<< HEAD
<<<<<<< HEAD
use common::types::Size;

/// Ceil the given size to the closer alignement. Alignement should be a power of 2.
=======
use common::types::{Size};

>>>>>>> d43d130... First commit
=======
use common::types::{Size};

>>>>>>> d43d130... First commit
#[inline]
pub fn ceil_to_power_of_2(size:Size,align:Size) -> Size {
	size & !(align-1)
}

<<<<<<< HEAD
<<<<<<< HEAD
/// Round up the given size to the closer alignement. Alignement should be a power of 2.
=======
>>>>>>> d43d130... First commit
=======
>>>>>>> d43d130... First commit
#[inline]
pub fn up_to_power_of_2(size:Size,align:Size) -> Size {
	let ret;

	if size & (align-1) != 0 {
		ret = (size & !(align-1)) + align;
	} else {
		ret = size;
	}

	ret
}

<<<<<<< HEAD
<<<<<<< HEAD
/// Helper function to be used by some functions, it convert an optional reference to pointer.
/// If Option is None, then the pointer is set to NULL.
#[inline]
pub fn ptr_from_option_ref<T>(value:Option<&mut T>) -> * mut T {
	match value {
		Some(x) => x as * mut T,
		None => 0 as * mut T,
	}
}

/// Make the other way by converting an Optional pointer into a ref. The function
/// use unwrap() so will crash if option is None.
#[inline]
pub fn ref_from_option_ptr<'a,T>(value:Option<* mut T>) -> &'a mut T {
	unsafe{&mut *value.unwrap()}
}

=======
>>>>>>> d43d130... First commit
=======
>>>>>>> d43d130... First commit
#[cfg(test)]
mod tests
{
	use common::ops;

	#[test]
<<<<<<< HEAD
<<<<<<< HEAD
	fn ptr_from_option_ref() {
		let mut a = 1;
		
		let ptr1 = ops::ptr_from_option_ref(Some(&mut a));
		let ptr2 = ops::ptr_from_option_ref(Some(&mut a));
		let ptr3 = ops::ptr_from_option_ref::<i32>(None);

		assert!(!ptr1.is_null());
		assert!(!ptr2.is_null());
		assert!(ptr3.is_null());
	}

	#[test]
=======
>>>>>>> d43d130... First commit
=======
>>>>>>> d43d130... First commit
	fn ceil_to_power_of_2() {
		assert_eq!(ops::ceil_to_power_of_2(0,1),0);
		assert_eq!(ops::ceil_to_power_of_2(9,1),9);
		
		assert_eq!(ops::ceil_to_power_of_2(0,2),0);
		assert_eq!(ops::ceil_to_power_of_2(9,2),8);
		assert_eq!(ops::ceil_to_power_of_2(10,2),10);

		assert_eq!(ops::ceil_to_power_of_2(0,8),0);
		assert_eq!(ops::ceil_to_power_of_2(9,8),8);
		assert_eq!(ops::ceil_to_power_of_2(10,8),8);
	}

	#[test]
	fn up_to_power_of_2() {
		assert_eq!(ops::up_to_power_of_2(0,1),0);
		assert_eq!(ops::up_to_power_of_2(9,1),9);
		
		assert_eq!(ops::up_to_power_of_2(0,2),0);
		assert_eq!(ops::up_to_power_of_2(9,2),10);
		assert_eq!(ops::up_to_power_of_2(10,2),10);

		assert_eq!(ops::up_to_power_of_2(0,8),0);
		assert_eq!(ops::up_to_power_of_2(9,8),16);
		assert_eq!(ops::up_to_power_of_2(10,8),16);
	}
}