use asgard_jbc::{AccessFlags, ClassFile, ConstantPoolInfo, Decode};

fn main() {
    let input = std::fs::read("/home/valaphee/Documents/example/build/classes/java/main/com/valaphee/asgard/example/CallMe.class").unwrap();
    let class_file = ClassFile::decode(&mut input.as_ref()).unwrap();

    let mut methods = vec![];
    let mut overridable_methods = vec![];
    let mut overridable_method_bodies = vec![];
    for method in &class_file.methods {
        if !method.access_flags.contains(AccessFlags::PUBLIC) {
            continue;
        }

        let name = match &class_file.constant_pool[(method.name_index - 1) as usize] {
            ConstantPoolInfo::Utf8(value) => value,
            _ => todo!()
        };
        let signature = match &class_file.constant_pool[(method.descriptor_index - 1) as usize] {
            ConstantPoolInfo::Utf8(value) => value,
            _ => todo!()
        };

        if name == "<init>" {
            methods.push(format!(
r#"fn new() -> Self {{
        let env = crate::ENV.get();
        unsafe {{
            let functions = &**env;
            let class = functions.FindClass.unwrap()(env, "org/geysermc/geyser/GeyserMain\0".as_ptr() as _);
            let method_id = functions.GetMethodID.unwrap()(env, class, "{name}\0".as_ptr() as _, "{signature}\0".as_ptr() as _);
            Self(functions.NewObjectA.unwrap()(env, class, method_id, std::ptr::null()))
        }}
    }}"#
            ));
        } else {
            overridable_methods.push(format!(
r#"fn {name}(&self) {{
        self.object().{name}();
    }}"#
            ));
            overridable_method_bodies.push(format!(
r#"fn {name}(&self) {{
        let env = crate::ENV.get();
        unsafe {{
            let functions = &**env;
            let class = functions.FindClass.unwrap()(env, "org/geysermc/geyser/GeyserMain\0".as_ptr() as _);
            let method_id = functions.GetMethodID.unwrap()(env, class, "{name}\0".as_ptr() as _, "{signature}\0".as_ptr() as _);
            functions.CallVoidMethodA.unwrap()(env, self.0, method_id, std::ptr::null());
        }}
    }}"#
            ));
        }
    }

    let overridable_methods = overridable_methods.join("\n\n    ");
    println!(
r#"pub trait GeyserMain {{
    fn object(&self) -> impl GeyserMain;

    {overridable_methods}
}}"#
    );

    println!(
r#"#[derive(Copy, Clone)]
pub struct GeyserMainObject(pub asgard_jni::jobject);"#
    );

    let methods = methods.join("\n\n    ");
    println!(
r#"impl GeyserMainObject {{
    {methods}
}}"#
    );

    let overridable_method_bodies = overridable_method_bodies.join("\n\n    ");
    println!(
r#"impl GeyserMain for GeyserMainObject {{
    fn object(&self) -> impl GeyserMain {{
        *self
    }}

    {overridable_method_bodies}
}}"#
    );
}
