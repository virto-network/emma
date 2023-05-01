#[macro_export]
macro_rules! map {
	( $($k:expr => $v:expr),* ) => {
		{
			let mut hm = std::collections::HashMap::<String, String>::new();
			$(
				hm.insert($k.to_string(), $v.to_string());
			)*
			hm
		}
	};
}
