#[macro_export]
macro_rules! cache {
	($cache_type:ty, $getter_expression:expr) => {
		unsafe {
			static mut STORAGE:Option<$cache_type> = None;
			if STORAGE.is_none() {
				STORAGE = Some($getter_expression);
			}
			STORAGE.as_mut().unwrap()
		}
	};
}
#[macro_export]
macro_rules! age_cache {
	($cache_type:ty, $getter_expression:expr, $max_age:expr) => {
		unsafe {
			static mut STORAGE:Option<($cache_type, std::time::Instant)> = None;
			if STORAGE.is_none() || STORAGE.as_ref().unwrap().1.elapsed() > $max_age {
				STORAGE = Some(($getter_expression, std::time::Instant::now()))
			}
			&STORAGE.as_ref().unwrap().0
		}
	};
}
#[macro_export]
macro_rules! state_cache {
	($cache_type:ty, $getter_expression:expr, $arg_type:ty, $arg:expr) => {
		unsafe {
			static mut STORAGE:Option<($cache_type, $arg_type)> = None;
			if STORAGE.is_none() || STORAGE.as_ref().unwrap().1 != $arg {
				STORAGE = Some(($getter_expression, $arg));
			}
			&STORAGE.as_ref().unwrap().0
		}
	};
}
#[macro_export]
macro_rules! dict_cache {
	($cache_type:ty, $getter_expression:expr, $arg_type:ty, $arg:expr) => {
		unsafe {
			static mut STORAGE:Vec<($cache_type, $arg_type)> = Vec::new();
			match STORAGE.iter().find(|(_, arg)| *arg == $arg) {
				Some(entry) => &entry.0,
				None => {
					STORAGE.push(($getter_expression, $arg));
					&STORAGE.last().unwrap().0
				}
			}
		}
	};
}



#[cfg(test)]
mod tests {
	
	#[test]
	fn simple_cache_uses_stored_data() {
		static mut UPDATES:usize = 0;
		for _ in 0..10 {
			let cached_value:&usize = cache!(usize, { UPDATES += 1; UPDATES });
			assert_eq!(*cached_value, 1);
		}
	}

	#[test]
	fn simple_cache_uses_separate_storage_per_cache() {
		let a:&usize = cache!(usize, 20);
		let b:&usize = cache!(usize, 10);
		assert_eq!(*a, 20);
		assert_eq!(*b, 10);
	}

	#[test]
	fn state_updates_after_arg_change() {
		static mut UPDATES:usize = 0;
		for index in 0..10 {
			let validation:&usize = state_cache!(usize, { UPDATES += 1; index / 3 * 2 }, usize, index / 3);
			assert_eq!(*validation, index / 3 * 2);
		}
		assert_eq!(unsafe { UPDATES }, 4);
	}
	
	#[test]
	fn cache_dict_stores_multiple_data_entries() {
		static mut UPDATES:usize = 0;
		for _ in 0..2 {
			for index in 0..10 {
				let validation:&usize = dict_cache!(usize, { UPDATES += 1; index / 3 * 2 }, usize, index / 3);
				assert_eq!(*validation, index / 3 * 2);
			}
		}
		assert_eq!(unsafe { UPDATES }, 4);
	}
}