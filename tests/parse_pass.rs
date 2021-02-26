use structype_derive::StrucType;

#[derive(StrucType)]
struct MyStruct {
    #[structype_label("Overridde name for string", something="another")]
    _my_string: String,
    #[structype_label = "int_override"]
    _my_int64: i64,
    _my_float: f64,
    _my_enum: MyEnum,
    _my_number: f64,
    _my_nested_struct: MyAnotherStruct,
}

#[derive(StrucType)]
struct MyAnotherStruct {
    _my_another_string: String,
}

#[derive(StrucType)]
enum MyEnum {
    #[structype_label = "my_over-ridden-enum"]
    _VariantA,
    _VariantB,
}

#[derive(StrucType)]
union MyUnion {
    #[structype_label = "my_over-ridden-union"]
    _unsigned: u32,
    _signed: i32,
}

fn main() {
    MyStruct::print_fields();
    let data = MyStruct::as_string();
    println!("{}", data);
    MyAnotherStruct::print_fields();
    let data = MyAnotherStruct::as_string();
    println!("{}", data);

    MyEnum::print_fields();
    let data = MyEnum::as_string();
    println!("{}", data);
    MyUnion::print_fields();
    let data = MyUnion::as_string();
    println!("{}", data);
}
