# Crate structype_derive

This is a derive procedural macro that will let you add custom derive and attributes over structs, enums and unions. This derive will add two impl on the
type. The `as_string()` method returns a json serialized string representation of the type with any meta information annotated with `structype_meta("key"=val)` attribute while the `print_fields()` method will print the same to STDOUT. This macro will panic at compile time if annotated over tuple and unit structs.

## Example

```rust
use structype_derive::StrucType;
#[derive(StrucType)]
// #[structype_meta("labelover_ride=name")] This will panic the macro
struct UserStruct {
    #[structype_meta(override_name="Primary ID", order="1")]
    id: i64,
    #[structype_meta(override_name="name", order="0")]
    username: String,
    org: String,
    details: Details,
}

#[derive(StrucType)]
struct Details {
    user_attributes: std::collections::HashMap<String, String>,
}

fn print_struct_fields() {
    UserStruct::print_fields();
    let data = UserStruct::as_string();
    println!("{}", data);
    Details::print_fields();
    let data = Details::as_string();
    println!("{}", data);
}
```

The above will generate and return a json serialized string representation where the key is the struct's field name and the value is a `HashMap<String, String>` of `structype_meta`'s key-val. If the `structype_meta` is absent, the field's associated value would be an empty `{}`.

## Output

```json
  [
      {
          "id": {
              "override_name": "Primary ID",
              "order": "1"
          }
      },
      {
          "username": {
              "override_name": "name",
              "order": "0"
          }
      },
      {
          "org": {}
      },
      {
          "details": {}
      }
  ]
```
