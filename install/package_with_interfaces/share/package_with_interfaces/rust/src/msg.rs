#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to package_with_interfaces__msg__AmazingQuote
/// AmazingQuote.msg from https://ros2-tutorial.readthedocs.io
/// An inspirational quote a day keeps the therapist away

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AmazingQuote {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub quote: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub philosopher_name: std::string::String,

}



impl Default for AmazingQuote {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AmazingQuote::default())
  }
}

impl rosidl_runtime_rs::Message for AmazingQuote {
  type RmwMsg = super::msg::rmw::AmazingQuote;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        quote: msg.quote.as_str().into(),
        philosopher_name: msg.philosopher_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
        quote: msg.quote.as_str().into(),
        philosopher_name: msg.philosopher_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      quote: msg.quote.to_string(),
      philosopher_name: msg.philosopher_name.to_string(),
    }
  }
}


