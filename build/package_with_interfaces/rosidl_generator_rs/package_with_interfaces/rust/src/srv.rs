#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to package_with_interfaces__srv__WhatIsThePoint_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WhatIsThePoint_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub quote: super::msg::AmazingQuote,

}



impl Default for WhatIsThePoint_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::WhatIsThePoint_Request::default())
  }
}

impl rosidl_runtime_rs::Message for WhatIsThePoint_Request {
  type RmwMsg = super::srv::rmw::WhatIsThePoint_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        quote: super::msg::AmazingQuote::into_rmw_message(std::borrow::Cow::Owned(msg.quote)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        quote: super::msg::AmazingQuote::into_rmw_message(std::borrow::Cow::Borrowed(&msg.quote)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      quote: super::msg::AmazingQuote::from_rmw_message(msg.quote),
    }
  }
}


// Corresponds to package_with_interfaces__srv__WhatIsThePoint_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WhatIsThePoint_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub point: geometry_msgs::msg::Point,

}



impl Default for WhatIsThePoint_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::WhatIsThePoint_Response::default())
  }
}

impl rosidl_runtime_rs::Message for WhatIsThePoint_Response {
  type RmwMsg = super::srv::rmw::WhatIsThePoint_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        point: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.point)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        point: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.point)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      point: geometry_msgs::msg::Point::from_rmw_message(msg.point),
    }
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


