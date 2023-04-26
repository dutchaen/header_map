#[macro_export] macro_rules! header_map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = reqwest::header::HeaderMap::new();
         $( map.append($key, $val.parse::<reqwest::header::HeaderValue>().unwrap_or(reqwest::header::HeaderValue::from_static("n/a"))); )*
         map
    }}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let map = header_map! {
            "User-Agent" => "小猫",
            "Something-Random" => "Random"
        };

        let value = map.get("Something-Random").unwrap()
            .to_str()
            .unwrap();
        assert_eq!(value, "Random");
    }
}
