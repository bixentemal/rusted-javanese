#![ allow(nonstandard_style)]
pub mod jni_inc;

use std::ffi::CString;
use std::ptr;
use jni_inc::{JavaVM, JavaVMInitArgs, JavaVMOption, jint, JNI_CreateJavaVM, JNI_VERSION_1_8, JNIEnv, jclass, jmethodID};

pub(crate) fn actual_call(env: *mut JNIEnv, class: jclass, method: jmethodID, input1: i32, input2: i32) -> i32{
    unsafe { (*(*env)).CallStaticIntMethod.unwrap()(env, class, method, input1, input2)}
}

fn convert_to_c_char(to_convert: &str) -> *const ::std::os::raw::c_char {
    let c_str_class_name = CString::new(to_convert).unwrap();
    c_str_class_name.as_ptr() as *const ::std::os::raw::c_char
}

pub(crate) fn prepare_static_method(env: *mut JNIEnv, class_name: &str, method_name: &str, method_sig: &str) -> (jclass, jmethodID) {
    //let result: i32 = 0;

    let c_str_class_name = CString::new(class_name).unwrap();
    let c_char_pts_class_name: *const ::std::os::raw::c_char = c_str_class_name.as_ptr() as *const ::std::os::raw::c_char;
    let return_class = unsafe { (*(*env)).FindClass.unwrap()(env, c_char_pts_class_name) };

    let c_str_method_name = CString::new(method_name).unwrap();
    let c_char_pts_method_name: *const ::std::os::raw::c_char = c_str_method_name.as_ptr() as *const ::std::os::raw::c_char;
    //let c_char_pts_method_name: *const ::std::os::raw::c_char = convert_to_c_char("abs");
    let c_str_method_sig = CString::new(method_sig).unwrap();
    let c_char_pts_method_sig: *const ::std::os::raw::c_char = c_str_method_sig.as_ptr() as *const ::std::os::raw::c_char;

    /*let c_char_pts_class_name = convert_to_c_char("java/lang/Math");
    let return_class = unsafe { (*(*env)).FindClass.unwrap()(env, c_char_pts_class_name) };
    let c_char_pts_method_name = convert_to_c_char("abs");
    let c_char_pts_method_sig = convert_to_c_char("(I)I");*/

    let method = unsafe { (*(*env)).GetStaticMethodID.unwrap()(env,return_class, c_char_pts_method_name,c_char_pts_method_sig) };

    (return_class, method)
}

pub(crate) fn create_vm(vm_opt: &str) -> (*mut JavaVM, *mut JNIEnv) {
    let mut env: *mut JNIEnv = ::std::ptr::null_mut();
    let mut jvm: *mut JavaVM = ::std::ptr::null_mut();

    let c_str_opt = CString::new(vm_opt).unwrap();
    let c_char_pts_opt: *mut ::std::os::raw::c_char = c_str_opt.as_ptr() as *mut ::std::os::raw::c_char;

    let mut extr= ptr::null_mut();
    let c_str_opt = CString::new("").unwrap();
    let mut vm_args: JavaVMInitArgs = JavaVMInitArgs{
        version: JNI_VERSION_1_8 as jint,
        nOptions: 1,
        options: &mut JavaVMOption{
            optionString: c_char_pts_opt,
            extraInfo: *&mut extr,
        },
        ignoreUnrecognized: 1,
    };

    let args: *mut ::std::os::raw::c_void =  &mut vm_args as *mut _ as *mut ::std::os::raw::c_void;
    let ret = unsafe { JNI_CreateJavaVM(
        &mut jvm as *mut _,
        &mut env as *mut *mut JNIEnv as *mut *mut ::std::os::raw::c_void,
        args) };
    if ret > 0 {
        panic!("return code fail: {0}", ret);
    }
    (jvm, env)
}