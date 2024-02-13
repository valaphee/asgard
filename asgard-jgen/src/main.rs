use asgard_jbc::{AccessFlags, ClassFile, ConstantPoolInfo, Decode, MethodDescriptor};

fn main() {
    let input = std::fs::read("/home/valaphee/Documents/asgard-example/build/classes/java/main/com/valaphee/asgard/example/CallMe.class").unwrap();
    let class_file = ClassFile::decode(&mut input.as_ref()).unwrap();

    let class_name: &String = match &class_file.constant_pool[(class_file.this_class - 1) as usize] {
        ConstantPoolInfo::Class { name_index } => match &class_file.constant_pool[(name_index - 1) as usize] {
            ConstantPoolInfo::Utf8(value) => value,
            _ => todo!()
        }
        _ => todo!()
    };
    let class_name_without_path = class_name.rsplit("/").next().unwrap();

    let mut methods = vec![];
    let mut overridable_methods = vec![];
    let mut overridable_method_bodies = vec![];
    for method in &class_file.methods {
        if !method.access_flags.contains(AccessFlags::PUBLIC) {
            continue;
        }

        let method_name = match &class_file.constant_pool[(method.name_index - 1) as usize] {
            ConstantPoolInfo::Utf8(value) => value,
            _ => todo!()
        };
        let method_descriptor_raw = match &class_file.constant_pool[(method.descriptor_index - 1) as usize] {
            ConstantPoolInfo::Utf8(value) => value,
            _ => todo!()
        };
        let method_descriptor: MethodDescriptor = method_descriptor_raw.parse().unwrap();

        if method_name == "<init>" {
            methods.push(format!(
r#"fn new() -> Self {{
        let env = crate::ENV.get();
        unsafe {{
            let functions = &**env;
            let class = functions.FindClass.unwrap()(env, "{class_name}\0".as_ptr() as _);
            let method_id = functions.GetMethodID.unwrap()(env, class, "{method_name}\0".as_ptr() as _, "{method_descriptor_raw}\0".as_ptr() as _);
            Self(functions.NewObjectA.unwrap()(env, class, method_id, std::ptr::null()))
        }}
    }}"#
            ));
        } else {
            let rust_type = match method_descriptor.return_type {
                asgard_jbc::FieldType::Byte => "i8",
                asgard_jbc::FieldType::Char => "u16",
                asgard_jbc::FieldType::Double => "f64",
                asgard_jbc::FieldType::Float => "f32",
                asgard_jbc::FieldType::Int => "i32",
                asgard_jbc::FieldType::Long => "i64",
                asgard_jbc::FieldType::Object(_) => todo!(),
                asgard_jbc::FieldType::Short => "i16",
                asgard_jbc::FieldType::Boolean => "bool",
                asgard_jbc::FieldType::Array(_) => todo!(),
                asgard_jbc::FieldType::Void => "()",
            };
            let rust_conv = match method_descriptor.return_type {
                asgard_jbc::FieldType::Boolean => " != 0",
                _ => ""
            };
            let method_call = match method_descriptor.return_type {
                asgard_jbc::FieldType::Byte => "CallByteMethodA",
                asgard_jbc::FieldType::Char => "CallCharMethodA",
                asgard_jbc::FieldType::Double => "CallDoubleMethodA",
                asgard_jbc::FieldType::Float => "CallFloatMethodA",
                asgard_jbc::FieldType::Int => "CallIntMethodA",
                asgard_jbc::FieldType::Long => "CallLongMethodA",
                asgard_jbc::FieldType::Object(_) => todo!(),
                asgard_jbc::FieldType::Short => "CallShortMethodA",
                asgard_jbc::FieldType::Boolean => "CallBooleanMethodA",
                asgard_jbc::FieldType::Array(_) => todo!(),
                asgard_jbc::FieldType::Void => "CallVoidMethodA",
            };

            method_descriptor.parameter_types.iter().map(|parameter_type| {
                match parameter_type {
                    asgard_jbc::FieldType::Byte => "i8",
                    asgard_jbc::FieldType::Char => "u16",
                    asgard_jbc::FieldType::Double => "f64",
                    asgard_jbc::FieldType::Float => "f32",
                    asgard_jbc::FieldType::Int => "i32",
                    asgard_jbc::FieldType::Long => "i64",
                    asgard_jbc::FieldType::Object(_) => todo!(),
                    asgard_jbc::FieldType::Short => "i16",
                    asgard_jbc::FieldType::Boolean => "bool",
                    asgard_jbc::FieldType::Array(_) => todo!(),
                    _ => unreachable!()
                }
            });

            overridable_methods.push(format!(
r#"fn {method_name}(&self) -> {rust_type} {{
        self.object().{method_name}()
    }}"#
            ));
            overridable_method_bodies.push(format!(
r#"fn {method_name}(&self) -> {rust_type} {{
        let env = crate::ENV.get();
        unsafe {{
            let functions = &**env;
            let class = functions.FindClass.unwrap()(env, "{class_name}\0".as_ptr() as _);
            let method_id = functions.GetMethodID.unwrap()(env, class, "{method_name}\0".as_ptr() as _, "{method_descriptor_raw}\0".as_ptr() as _);
            functions.{method_call}.unwrap()(env, self.0, method_id, std::ptr::null()){rust_conv}
        }}
    }}"#
            ));
        }
    }

    let overridable_methods = overridable_methods.join("\n\n    ");
    println!(
r#"pub trait {class_name_without_path} {{
    fn object(&self) -> impl {class_name_without_path};

    {overridable_methods}
}}"#
    );

    println!(
r#"#[derive(Copy, Clone)]
pub struct {class_name_without_path}Object(pub asgard_jni::jobject);"#
    );

    let methods = methods.join("\n\n    ");
    println!(
r#"impl {class_name_without_path}Object {{
    {methods}
}}"#
    );

    let overridable_method_bodies = overridable_method_bodies.join("\n\n    ");
    println!(
r#"impl {class_name_without_path} for {class_name_without_path}Object {{
    fn object(&self) -> impl {class_name_without_path} {{
        *self
    }}

    {overridable_method_bodies}
}}"#
    );
}
