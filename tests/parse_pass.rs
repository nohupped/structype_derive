use std::collections::HashMap;
use structype_derive::StrucType;
#[derive(StrucType)]
// #[structype_meta("labelover_ride=name")] // This will panic the macro
struct UserStruct {
    #[structype_meta(override_name = "Primary ID", order = "1")]
    _id: i64,
    #[structype_meta(override_name = "name", order = "0")]
    _username: String,
    _org: String,
    _details: Details,
}

#[derive(StrucType)]
struct Details {
    _user_attributes: HashMap<String, String>,
}

#[derive(StrucType)]
enum MyEnum {
    // #[structype_label = "my_over-ridden-enum"]
    _VariantA,
    _VariantB,
}

#[derive(StrucType)]
union MyUnion {
    // #[structype_label = "my_over-ridden-union"]
    _unsigned: u32,
    _signed: i32,
}

fn main() {
    UserStruct::print_fields();
    let data = UserStruct::as_string();
    println!("{}", data);
    Details::print_fields();
    let data = Details::as_string();
    println!("{}", data);

    MyEnum::print_fields();
    let data = MyEnum::as_string();
    println!("{}", data);
    MyUnion::print_fields();
    let data = MyUnion::as_string();
    println!("{}", data);
}
