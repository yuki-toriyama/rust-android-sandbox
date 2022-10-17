use jni::{
    objects::{JClass, JString},
    sys::jstring,
    JNIEnv,
};

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_rust_1android_MainActivity_hello(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();
    let output = env
        .new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");
    output.into_inner()
}
