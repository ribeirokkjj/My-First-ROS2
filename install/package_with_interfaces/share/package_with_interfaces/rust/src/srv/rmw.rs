#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "package_with_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__package_with_interfaces__srv__WhatIsThePoint_Request() -> *const std::ffi::c_void;
}

#[link(name = "package_with_interfaces__rosidl_generator_c")]
extern "C" {
    fn package_with_interfaces__srv__WhatIsThePoint_Request__init(msg: *mut WhatIsThePoint_Request) -> bool;
    fn package_with_interfaces__srv__WhatIsThePoint_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WhatIsThePoint_Request>, size: usize) -> bool;
    fn package_with_interfaces__srv__WhatIsThePoint_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WhatIsThePoint_Request>);
    fn package_with_interfaces__srv__WhatIsThePoint_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WhatIsThePoint_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<WhatIsThePoint_Request>) -> bool;
}

// Corresponds to package_with_interfaces__srv__WhatIsThePoint_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WhatIsThePoint_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub quote: super::super::msg::rmw::AmazingQuote,

}



impl Default for WhatIsThePoint_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !package_with_interfaces__srv__WhatIsThePoint_Request__init(&mut msg as *mut _) {
        panic!("Call to package_with_interfaces__srv__WhatIsThePoint_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WhatIsThePoint_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { package_with_interfaces__srv__WhatIsThePoint_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { package_with_interfaces__srv__WhatIsThePoint_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { package_with_interfaces__srv__WhatIsThePoint_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WhatIsThePoint_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WhatIsThePoint_Request where Self: Sized {
  const TYPE_NAME: &'static str = "package_with_interfaces/srv/WhatIsThePoint_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__package_with_interfaces__srv__WhatIsThePoint_Request() }
  }
}


#[link(name = "package_with_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__package_with_interfaces__srv__WhatIsThePoint_Response() -> *const std::ffi::c_void;
}

#[link(name = "package_with_interfaces__rosidl_generator_c")]
extern "C" {
    fn package_with_interfaces__srv__WhatIsThePoint_Response__init(msg: *mut WhatIsThePoint_Response) -> bool;
    fn package_with_interfaces__srv__WhatIsThePoint_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WhatIsThePoint_Response>, size: usize) -> bool;
    fn package_with_interfaces__srv__WhatIsThePoint_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WhatIsThePoint_Response>);
    fn package_with_interfaces__srv__WhatIsThePoint_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WhatIsThePoint_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<WhatIsThePoint_Response>) -> bool;
}

// Corresponds to package_with_interfaces__srv__WhatIsThePoint_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WhatIsThePoint_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub point: geometry_msgs::msg::rmw::Point,

}



impl Default for WhatIsThePoint_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !package_with_interfaces__srv__WhatIsThePoint_Response__init(&mut msg as *mut _) {
        panic!("Call to package_with_interfaces__srv__WhatIsThePoint_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WhatIsThePoint_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { package_with_interfaces__srv__WhatIsThePoint_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { package_with_interfaces__srv__WhatIsThePoint_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { package_with_interfaces__srv__WhatIsThePoint_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WhatIsThePoint_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WhatIsThePoint_Response where Self: Sized {
  const TYPE_NAME: &'static str = "package_with_interfaces/srv/WhatIsThePoint_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__package_with_interfaces__srv__WhatIsThePoint_Response() }
  }
}






#[link(name = "package_with_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__package_with_interfaces__srv__WhatIsThePoint() -> *const std::ffi::c_void;
}

// Corresponds to package_with_interfaces__srv__WhatIsThePoint
#[allow(missing_docs, non_camel_case_types)]
pub struct WhatIsThePoint;

impl rosidl_runtime_rs::Service for WhatIsThePoint {
    type Request = WhatIsThePoint_Request;
    type Response = WhatIsThePoint_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__package_with_interfaces__srv__WhatIsThePoint() }
    }
}


