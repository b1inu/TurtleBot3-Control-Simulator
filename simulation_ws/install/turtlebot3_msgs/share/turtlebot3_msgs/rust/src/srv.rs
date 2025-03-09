

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sound_Request {
    pub value: u8,
}



impl Default for Sound_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::Sound_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Sound_Request {
  type RmwMsg = crate::srv::rmw::Sound_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      value: msg.value,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sound_Response {
    pub success: bool,
    pub message: std::string::String,
}



impl Default for Sound_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::Sound_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Sound_Response {
  type RmwMsg = crate::srv::rmw::Sound_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dqn_Request {
    pub action: u8,
    pub init: bool,
}



impl Default for Dqn_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::Dqn_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Dqn_Request {
  type RmwMsg = crate::srv::rmw::Dqn_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        action: msg.action,
        init: msg.init,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      action: msg.action,
      init: msg.init,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      action: msg.action,
      init: msg.init,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dqn_Response {
    pub state: Vec<f32>,
    pub reward: f32,
    pub done: bool,
}



impl Default for Dqn_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::srv::rmw::Dqn_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Dqn_Response {
  type RmwMsg = crate::srv::rmw::Dqn_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.into(),
        reward: msg.reward,
        done: msg.done,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_slice().into(),
      reward: msg.reward,
      done: msg.done,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: msg.state
          .into_iter()
          .collect(),
      reward: msg.reward,
      done: msg.done,
    }
  }
}






#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__Sound() -> *const std::os::raw::c_void;
}

// Corresponds to turtlebot3_msgs__srv__Sound
pub struct Sound;

impl rosidl_runtime_rs::Service for Sound {
  type Request = crate::srv::Sound_Request;
  type Response = crate::srv::Sound_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__Sound() }
  }
}




#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__Dqn() -> *const std::os::raw::c_void;
}

// Corresponds to turtlebot3_msgs__srv__Dqn
pub struct Dqn;

impl rosidl_runtime_rs::Service for Dqn {
  type Request = crate::srv::Dqn_Request;
  type Response = crate::srv::Dqn_Response;

  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__Dqn() }
  }
}



pub mod rmw {
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Sound_Request() -> *const std::os::raw::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__srv__Sound_Request__init(msg: *mut Sound_Request) -> bool;
    fn turtlebot3_msgs__srv__Sound_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Sound_Request>, size: usize) -> bool;
    fn turtlebot3_msgs__srv__Sound_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Sound_Request>);
    fn turtlebot3_msgs__srv__Sound_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Sound_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Sound_Request>) -> bool;
}

// Corresponds to turtlebot3_msgs__srv__Sound_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sound_Request {
    pub value: u8,
}



impl Default for Sound_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__srv__Sound_Request__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__srv__Sound_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Sound_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Sound_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Sound_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Sound_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Sound_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Sound_Request where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/srv/Sound_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Sound_Request() }
  }
}


#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Sound_Response() -> *const std::os::raw::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__srv__Sound_Response__init(msg: *mut Sound_Response) -> bool;
    fn turtlebot3_msgs__srv__Sound_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Sound_Response>, size: usize) -> bool;
    fn turtlebot3_msgs__srv__Sound_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Sound_Response>);
    fn turtlebot3_msgs__srv__Sound_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Sound_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Sound_Response>) -> bool;
}

// Corresponds to turtlebot3_msgs__srv__Sound_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sound_Response {
    pub success: bool,
    pub message: rosidl_runtime_rs::String,
}



impl Default for Sound_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__srv__Sound_Response__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__srv__Sound_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Sound_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Sound_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Sound_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Sound_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Sound_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Sound_Response where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/srv/Sound_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Sound_Response() }
  }
}


#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Dqn_Request() -> *const std::os::raw::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__srv__Dqn_Request__init(msg: *mut Dqn_Request) -> bool;
    fn turtlebot3_msgs__srv__Dqn_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Dqn_Request>, size: usize) -> bool;
    fn turtlebot3_msgs__srv__Dqn_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Dqn_Request>);
    fn turtlebot3_msgs__srv__Dqn_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Dqn_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Dqn_Request>) -> bool;
}

// Corresponds to turtlebot3_msgs__srv__Dqn_Request
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dqn_Request {
    pub action: u8,
    pub init: bool,
}



impl Default for Dqn_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__srv__Dqn_Request__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__srv__Dqn_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Dqn_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Dqn_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Dqn_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Dqn_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Dqn_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Dqn_Request where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/srv/Dqn_Request";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Dqn_Request() }
  }
}


#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Dqn_Response() -> *const std::os::raw::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__srv__Dqn_Response__init(msg: *mut Dqn_Response) -> bool;
    fn turtlebot3_msgs__srv__Dqn_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Dqn_Response>, size: usize) -> bool;
    fn turtlebot3_msgs__srv__Dqn_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Dqn_Response>);
    fn turtlebot3_msgs__srv__Dqn_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Dqn_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Dqn_Response>) -> bool;
}

// Corresponds to turtlebot3_msgs__srv__Dqn_Response
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dqn_Response {
    pub state: rosidl_runtime_rs::Sequence<f32>,
    pub reward: f32,
    pub done: bool,
}



impl Default for Dqn_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__srv__Dqn_Response__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__srv__Dqn_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Dqn_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Dqn_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Dqn_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Dqn_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Dqn_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Dqn_Response where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/srv/Dqn_Response";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Dqn_Response() }
  }
}






  #[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__Sound() -> *const std::os::raw::c_void;
  }

  // Corresponds to turtlebot3_msgs__srv__Sound
  pub struct Sound;

  impl rosidl_runtime_rs::Service for Sound {
    type Request = crate::srv::rmw::Sound_Request;
    type Response = crate::srv::rmw::Sound_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__Sound() }
    }
  }




  #[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
  extern "C" {
      fn rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__Dqn() -> *const std::os::raw::c_void;
  }

  // Corresponds to turtlebot3_msgs__srv__Dqn
  pub struct Dqn;

  impl rosidl_runtime_rs::Service for Dqn {
    type Request = crate::srv::rmw::Dqn_Request;
    type Response = crate::srv::rmw::Dqn_Response;

    fn get_type_support() -> *const std::os::raw::c_void {
      // SAFETY: No preconditions for this function.
      unsafe { rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__Dqn() }
    }
  }



}  // mod rmw
