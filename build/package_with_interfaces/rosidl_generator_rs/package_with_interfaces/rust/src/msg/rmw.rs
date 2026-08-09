#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "package_with_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__package_with_interfaces__msg__AmazingQuote() -> *const std::ffi::c_void;
}

#[link(name = "package_with_interfaces__rosidl_generator_c")]
extern "C" {
    fn package_with_interfaces__msg__AmazingQuote__init(msg: *mut AmazingQuote) -> bool;
    fn package_with_interfaces__msg__AmazingQuote__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AmazingQuote>, size: usize) -> bool;
    fn package_with_interfaces__msg__AmazingQuote__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AmazingQuote>);
    fn package_with_interfaces__msg__AmazingQuote__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AmazingQuote>, out_seq: *mut rosidl_runtime_rs::Sequence<AmazingQuote>) -> bool;
}

// Corresponds to package_with_interfaces__msg__AmazingQuote
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// AmazingQuote.msg from https://ros2-tutorial.readthedocs.io
/// An inspirational quote a day keeps the therapist away

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AmazingQuote {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub quote: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub philosopher_name: rosidl_runtime_rs::String,

}



impl Default for AmazingQuote {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !package_with_interfaces__msg__AmazingQuote__init(&mut msg as *mut _) {
        panic!("Call to package_with_interfaces__msg__AmazingQuote__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AmazingQuote {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { package_with_interfaces__msg__AmazingQuote__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { package_with_interfaces__msg__AmazingQuote__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { package_with_interfaces__msg__AmazingQuote__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AmazingQuote {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AmazingQuote where Self: Sized {
  const TYPE_NAME: &'static str = "package_with_interfaces/msg/AmazingQuote";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__package_with_interfaces__msg__AmazingQuote() }
  }
}


