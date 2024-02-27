// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jint;

#[no_mangle]
//JNIEXPORT jint JNICALL Java_com_rusty_JollyRogerRadio_calc
//   (JNIEnv *, jclass, jint);
pub extern "system" fn Java_com_rusty_JollyRogerRadio_calc<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    n: jint) -> jint {
    calc(n)
}

//JNIEXPORT jint JNICALL Java_com_rusty_JuniorJetRiders_gen
//   (JNIEnv *, jclass, jint, jint);
#[no_mangle]
pub extern "system" fn Java_com_rusty_JuniorJetRiders_gen<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    m: jint,
    round: jint) -> jint {
    gen(m, round)
}

fn calc(n: i32) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        let r = gen(n, i);
        sum += r;
    }
    sum / n
}

fn gen(m: i32, round: i32) -> i32 {
    (m * round) % 10
}