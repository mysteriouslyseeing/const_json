use const_json::{Json, const_json};

const VARIABLE: &str = "string";

const fn add_12(n: i64) -> i64 {
    n + 12
}

const JSON: Json = const_json!({
    "null": null,
    "bool": true,
    "float": 12.3,
    "int": 42,
    "str": "Hello, World!",
    "array": [1, null],
    "object": {
        "inner_bool": false,
        "inner_str": "foo bar"
    },

    "rust": VARIABLE,
    // Has to be surrounded in parentheses if it is a complex expression
    "function_result": (add_12(10))
});

fn main() {
    assert_eq!((), JSON["null"].null());
    assert_eq!(true, JSON["bool"].bool());
    assert_eq!(12.3, JSON["float"].float());
    assert_eq!(42, JSON["int"].int());
    assert_eq!("Hello, World!", JSON["str"].str());
    assert_eq!(1, JSON["array"][0].int());
    assert_eq!((), JSON["array"][1].null());
    assert_eq!(false, JSON["object"]["inner_bool"].bool());
    assert_eq!("foo bar", JSON["object"]["inner_str"].str());
    assert_eq!("string", JSON["rust"].str());
    assert_eq!(22, JSON["function_result"].int());

    // You can get the result as a constant value
    const FUNCTION_RESULT: i64 = JSON.get_val("function_result").int();

    let mut arr = [0; FUNCTION_RESULT as usize];

    for i in 0..arr.len() {
        arr[i] = i;
    }

    assert_eq!(arr.as_slice(), (0..22).collect::<Vec<_>>().as_slice());

    println!("{JSON:#?}");
}
