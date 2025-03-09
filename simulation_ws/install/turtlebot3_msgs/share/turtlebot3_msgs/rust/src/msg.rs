pub mod rmw {
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__msg__SensorState() -> *const std::os::raw::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__msg__SensorState__init(msg: *mut SensorState) -> bool;
    fn turtlebot3_msgs__msg__SensorState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SensorState>, size: usize) -> bool;
    fn turtlebot3_msgs__msg__SensorState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SensorState>);
    fn turtlebot3_msgs__msg__SensorState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SensorState>, out_seq: *mut rosidl_runtime_rs::Sequence<SensorState>) -> bool;
}

// Corresponds to turtlebot3_msgs__msg__SensorState
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SensorState {
    pub header: std_msgs::msg::rmw::Header,
    pub bumper: u8,
    pub cliff: f32,
    pub sonar: f32,
    pub illumination: f32,
    pub led: u8,
    pub button: u8,
    pub torque: bool,
    pub left_encoder: i32,
    pub right_encoder: i32,
    pub battery: f32,
}

impl SensorState {
    pub const BUMPER_FORWARD: u8 = 1;
    pub const BUMPER_BACKWARD: u8 = 2;
    /// Cliff sensor states (states are combined, when multiple cliff sensors are triggered)
    pub const CLIFF: u8 = 1;
    /// Sonar sensor states (states are combined, when multiple sonar sensors are triggered)
    pub const SONAR: u8 = 1;
    /// Illumination sensor (states are combined, when multiple illumination sensors are triggered)
    pub const ILLUMINATION: u8 = 1;
    /// Button states (states are combined, when multiple buttons are pressed)
    pub const BUTTON0: u8 = 1;
    pub const BUTTON1: u8 = 2;
    /// Motor errors
    pub const ERROR_LEFT_MOTOR: u8 = 1;
    pub const ERROR_RIGHT_MOTOR: u8 = 2;
    /// Motor torque
    pub const TORQUE_ON: u8 = 1;
    pub const TORQUE_OFF: u8 = 2;
}


impl Default for SensorState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__msg__SensorState__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__msg__SensorState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SensorState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__msg__SensorState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__msg__SensorState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__msg__SensorState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SensorState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SensorState where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/msg/SensorState";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__msg__SensorState() }
  }
}


#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__msg__Sound() -> *const std::os::raw::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__msg__Sound__init(msg: *mut Sound) -> bool;
    fn turtlebot3_msgs__msg__Sound__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Sound>, size: usize) -> bool;
    fn turtlebot3_msgs__msg__Sound__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Sound>);
    fn turtlebot3_msgs__msg__Sound__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Sound>, out_seq: *mut rosidl_runtime_rs::Sequence<Sound>) -> bool;
}

// Corresponds to turtlebot3_msgs__msg__Sound
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sound {
    pub value: u8,
}

impl Sound {
    pub const OFF: u8 = 0;
    pub const ON: u8 = 1;
    pub const LOW_BATTERY: u8 = 2;
    pub const ERROR: u8 = 3;
    pub const BUTTON1: u8 = 4;
    pub const BUTTON2: u8 = 5;
}


impl Default for Sound {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__msg__Sound__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__msg__Sound__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Sound {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__msg__Sound__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__msg__Sound__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__msg__Sound__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Sound {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Sound where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/msg/Sound";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__msg__Sound() }
  }
}


#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__msg__VersionInfo() -> *const std::os::raw::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__msg__VersionInfo__init(msg: *mut VersionInfo) -> bool;
    fn turtlebot3_msgs__msg__VersionInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<VersionInfo>, size: usize) -> bool;
    fn turtlebot3_msgs__msg__VersionInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<VersionInfo>);
    fn turtlebot3_msgs__msg__VersionInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<VersionInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<VersionInfo>) -> bool;
}

// Corresponds to turtlebot3_msgs__msg__VersionInfo
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VersionInfo {
    pub hardware: rosidl_runtime_rs::String,
    pub firmware: rosidl_runtime_rs::String,
    pub software: rosidl_runtime_rs::String,
}



impl Default for VersionInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__msg__VersionInfo__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__msg__VersionInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for VersionInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__msg__VersionInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__msg__VersionInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__msg__VersionInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for VersionInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for VersionInfo where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/msg/VersionInfo";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__msg__VersionInfo() }
  }
}


}  // mod rmw


#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SensorState {
    pub header: std_msgs::msg::Header,
    pub bumper: u8,
    pub cliff: f32,
    pub sonar: f32,
    pub illumination: f32,
    pub led: u8,
    pub button: u8,
    pub torque: bool,
    pub left_encoder: i32,
    pub right_encoder: i32,
    pub battery: f32,
}

impl SensorState {
    pub const BUMPER_FORWARD: u8 = 1;
    pub const BUMPER_BACKWARD: u8 = 2;
    /// Cliff sensor states (states are combined, when multiple cliff sensors are triggered)
    pub const CLIFF: u8 = 1;
    /// Sonar sensor states (states are combined, when multiple sonar sensors are triggered)
    pub const SONAR: u8 = 1;
    /// Illumination sensor (states are combined, when multiple illumination sensors are triggered)
    pub const ILLUMINATION: u8 = 1;
    /// Button states (states are combined, when multiple buttons are pressed)
    pub const BUTTON0: u8 = 1;
    pub const BUTTON1: u8 = 2;
    /// Motor errors
    pub const ERROR_LEFT_MOTOR: u8 = 1;
    pub const ERROR_RIGHT_MOTOR: u8 = 2;
    /// Motor torque
    pub const TORQUE_ON: u8 = 1;
    pub const TORQUE_OFF: u8 = 2;
}


impl Default for SensorState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::SensorState::default())
  }
}

impl rosidl_runtime_rs::Message for SensorState {
  type RmwMsg = crate::msg::rmw::SensorState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        bumper: msg.bumper,
        cliff: msg.cliff,
        sonar: msg.sonar,
        illumination: msg.illumination,
        led: msg.led,
        button: msg.button,
        torque: msg.torque,
        left_encoder: msg.left_encoder,
        right_encoder: msg.right_encoder,
        battery: msg.battery,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      bumper: msg.bumper,
      cliff: msg.cliff,
      sonar: msg.sonar,
      illumination: msg.illumination,
      led: msg.led,
      button: msg.button,
      torque: msg.torque,
      left_encoder: msg.left_encoder,
      right_encoder: msg.right_encoder,
      battery: msg.battery,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      bumper: msg.bumper,
      cliff: msg.cliff,
      sonar: msg.sonar,
      illumination: msg.illumination,
      led: msg.led,
      button: msg.button,
      torque: msg.torque,
      left_encoder: msg.left_encoder,
      right_encoder: msg.right_encoder,
      battery: msg.battery,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sound {
    pub value: u8,
}

impl Sound {
    pub const OFF: u8 = 0;
    pub const ON: u8 = 1;
    pub const LOW_BATTERY: u8 = 2;
    pub const ERROR: u8 = 3;
    pub const BUTTON1: u8 = 4;
    pub const BUTTON2: u8 = 5;
}


impl Default for Sound {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::Sound::default())
  }
}

impl rosidl_runtime_rs::Message for Sound {
  type RmwMsg = crate::msg::rmw::Sound;

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
pub struct VersionInfo {
    pub hardware: std::string::String,
    pub firmware: std::string::String,
    pub software: std::string::String,
}



impl Default for VersionInfo {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::VersionInfo::default())
  }
}

impl rosidl_runtime_rs::Message for VersionInfo {
  type RmwMsg = crate::msg::rmw::VersionInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        hardware: msg.hardware.as_str().into(),
        firmware: msg.firmware.as_str().into(),
        software: msg.software.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        hardware: msg.hardware.as_str().into(),
        firmware: msg.firmware.as_str().into(),
        software: msg.software.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      hardware: msg.hardware.to_string(),
      firmware: msg.firmware.to_string(),
      software: msg.software.to_string(),
    }
  }
}


