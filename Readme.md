# Crate structype_derive

This is a derive procedural macro that will let you add custom derive and attributes over structs and enums. This derive will add two impl on the type.print_fields() will print the type field names to the STDOUT, while as_string() returns a json serialized string key-value representation where key is the type's field name and value is the attribute set in `structype_label`. This macro panics if applied over tuple structs and unit structs and won't let you compile.

## Example

```rust
use structype_derive::StrucType;

#[derive(StrucType)]
#[structype_label = "over_ride name"] // This will panic and won't let you compile
struct MyStruct {
    #[structype_label = "Overridde name for string"]
    _my_string: String,
    #[structype_label = "int_override"]
    _my_int64: i64,
    _my_float: f64,
    _my_nested_struct: MyNestedStruct,
}

#[derive(StrucType)]
struct MyNestedStruct {
    _my_nested_struct_string: String,
}

fn main() {
    MyStruct::print_fields();
    let data = MyStruct::as_string();
    println!("{}", data);
    MyNestedStruct::print_fields();
    let data = MyNestedStruct::as_string();
    println!("{}", data);
}
```

## Output

The above snippet will generate and return a json serialized string representation where the key is the struct's field name and the value is the structype_label's value. If the structype_label is absent, the value will be the same as that of the key.

```json
{
    "_my_string": "Overridde name for string",
    "_my_int64": "int_override",
    "_my_float": "_my_float",
    "_my_nested_struct": "_my_nested_struct"
}
```
