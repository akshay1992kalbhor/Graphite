use crate::tools::ToolType;
use crate::Color;
use bitflags::bitflags;

use document_core::LayerId;
use serde::{Deserialize, Serialize};

#[doc(inline)]
pub use document_core::DocumentResponse;

use std::{
	fmt,
	ops::{Deref, DerefMut},
};

#[derive(Debug, Clone)]
#[repr(C)]
pub enum Event {
	SelectTool(ToolType),
	SelectPrimaryColor(Color),
	SelectSecondaryColor(Color),
	SelectLayer(Vec<LayerId>),
	ToggleLayerVisibility(Vec<LayerId>),
	ToggleLayerExpansion(Vec<LayerId>),
	DeleteLayer(Vec<LayerId>),
	AddLayer(Vec<LayerId>),
	RenameLayer(Vec<LayerId>, String),
	SwapColors,
	ResetColors,
	AmbiguousMouseDown(MouseState),
	AmbiguousMouseUp(MouseState),
	LmbDown(MouseState),
	RmbDown(MouseState),
	MmbDown(MouseState),
	LmbUp(MouseState),
	RmbUp(MouseState),
	MmbUp(MouseState),
	MouseMove(ViewportPosition),
	KeyUp(Key),
	KeyDown(Key),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub enum ToolResponse {
	// These may not have the same names as any of the DocumentResponses
	SetActiveTool { tool_name: String },
	UpdateCanvas { document: String },
	EnableTextInput,
	DisableTextInput,
}

impl fmt::Display for ToolResponse {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		use ToolResponse::*;

		let name = match_variant_name!(match (self) {
			SetActiveTool,
			UpdateCanvas,
			EnableTextInput,
			DisableTextInput,
		});

		formatter.write_str(name)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
// TODO - Make Copy when possible
pub enum Response {
	Tool(ToolResponse),
	Document(DocumentResponse),
}

impl From<ToolResponse> for Response {
	fn from(response: ToolResponse) -> Self {
		Response::Tool(response)
	}
}

impl From<DocumentResponse> for Response {
	fn from(response: DocumentResponse) -> Self {
		Response::Document(response)
	}
}

impl fmt::Display for Response {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		use Response::*;

		let name = match_variant_name!(match (self) {
			Tool,
			Document
		});
		let appendix = match self {
			Tool(t) => t.to_string(),
			Document(d) => d.to_string(),
		};

		formatter.write_str(format!("{}::{}", name, appendix).as_str())
	}
}

// origin is top left
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct ViewportPosition {
	pub x: u32,
	pub y: u32,
}

impl ViewportPosition {
	pub fn distance(&self, other: &Self) -> f64 {
		let x_diff = other.x as f64 - self.x as f64;
		let y_diff = other.y as f64 - self.y as f64;
		f64::sqrt(x_diff * x_diff + y_diff * y_diff)
	}
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct TracePoint {
	pub mouse_state: MouseState,
	pub mod_keys: ModKeys,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct MouseState {
	pub position: ViewportPosition,
	pub mouse_keys: MouseKeys,
}

impl MouseState {
	pub fn new() -> MouseState {
		Self::default()
	}

	pub fn from_pos(x: u32, y: u32) -> MouseState {
		MouseState {
			position: ViewportPosition { x, y },
			mouse_keys: MouseKeys::default(),
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Key {
	UnknownKey,
	KeyR,
	KeyM,
	KeyE,
	KeyL,
	KeyP,
	KeyV,
	KeyX,
	KeyZ,
	KeyY,
	KeyEnter,
	Key0,
	Key1,
	Key2,
	Key3,
	Key4,
	Key5,
	Key6,
	Key7,
	Key8,
	Key9,
	KeyShift,
	KeyCaps,
	KeyControl,
	KeyAlt,
	KeyEscape,
}

bitflags! {
	#[derive(Default)]
	#[repr(transparent)]
	pub struct ModKeys: u8 {
		const CONTROL = 0b0000_0001;
		const SHIFT   = 0b0000_0010;
		const ALT     = 0b0000_0100;
	}
}

bitflags! {
	#[derive(Default)]
	#[repr(transparent)]
	pub struct MouseKeys: u8 {
		const LEFT   = 0b0000_0001;
		const RIGHT  = 0b0000_0010;
		const MIDDLE = 0b0000_0100;
	}
}
