use crate::events::{CanvasTransform, Key, ViewportPosition};
use crate::events::{Event, ToolResponse};
use crate::tools::{Fsm, Tool};
use crate::Document;
use document_core::layers::style;
use document_core::Operation;

use super::DocumentToolData;

#[derive(Default)]
pub struct Ellipse {
	fsm_state: EllipseToolFsmState,
	data: EllipseToolData,
}

impl Tool for Ellipse {
	fn handle_input(&mut self, event: &Event, document: &Document, tool_data: &DocumentToolData, canvas_transform: &CanvasTransform) -> (Vec<ToolResponse>, Vec<Operation>) {
		let mut responses = Vec::new();
		let mut operations = Vec::new();
		self.fsm_state = self.fsm_state.transition(event, document, tool_data, &mut self.data, canvas_transform, &mut responses, &mut operations);

		(responses, operations)
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EllipseToolFsmState {
	Ready,
	LmbDown,
}

impl Default for EllipseToolFsmState {
	fn default() -> Self {
		EllipseToolFsmState::Ready
	}
}
#[derive(Clone, Debug, Default)]
struct EllipseToolData {
	drag_start: ViewportPosition,
	drag_current: ViewportPosition,
	constrain_to_circle: bool,
	center_around_cursor: bool,
}

impl Fsm for EllipseToolFsmState {
	type ToolData = EllipseToolData;

	fn transition(
		self,
		event: &Event,
		document: &Document,
		tool_data: &DocumentToolData,
		data: &mut Self::ToolData,
		canvas_transform: &CanvasTransform,
		_responses: &mut Vec<ToolResponse>,
		operations: &mut Vec<Operation>,
	) -> Self {
		match (self, event) {
			(EllipseToolFsmState::Ready, Event::LmbDown(mouse_state)) => {
				data.drag_start = mouse_state.position;
				data.drag_current = mouse_state.position;
				operations.push(Operation::MountWorkingFolder { path: vec![] });
				EllipseToolFsmState::LmbDown
			}
			(EllipseToolFsmState::Ready, Event::KeyDown(Key::KeyZ)) => {
				if let Some(id) = document.root.list_layers().last() {
					operations.push(Operation::DeleteLayer { path: vec![*id] })
				}
				EllipseToolFsmState::Ready
			}
			(EllipseToolFsmState::LmbDown, Event::MouseMove(mouse_state)) => {
				data.drag_current = *mouse_state;

				operations.push(Operation::ClearWorkingFolder);
				operations.push(make_operation(data, tool_data, canvas_transform));

				EllipseToolFsmState::LmbDown
			}
			(EllipseToolFsmState::LmbDown, Event::LmbUp(mouse_state)) => {
				data.drag_current = mouse_state.position;

				operations.push(Operation::ClearWorkingFolder);
				// TODO - introduce comparison threshold when operating with canvas coordinates (https://github.com/GraphiteEditor/Graphite/issues/100)
				if data.drag_start != data.drag_current {
					operations.push(make_operation(data, tool_data, canvas_transform));
					operations.push(Operation::CommitTransaction);
				}

				EllipseToolFsmState::Ready
			}
			// TODO - simplify with or_patterns when rust 1.53.0 is stable (https://github.com/rust-lang/rust/issues/54883)
			(EllipseToolFsmState::LmbDown, Event::KeyUp(Key::KeyEscape)) | (EllipseToolFsmState::LmbDown, Event::RmbDown(_)) => {
				operations.push(Operation::DiscardWorkingFolder);

				EllipseToolFsmState::Ready
			}
			(state, Event::KeyDown(Key::KeyShift)) => {
				data.constrain_to_circle = true;

				if state == EllipseToolFsmState::LmbDown {
					operations.push(Operation::ClearWorkingFolder);
					operations.push(make_operation(data, tool_data, canvas_transform));
				}

				self
			}
			(state, Event::KeyUp(Key::KeyShift)) => {
				data.constrain_to_circle = false;

				if state == EllipseToolFsmState::LmbDown {
					operations.push(Operation::ClearWorkingFolder);
					operations.push(make_operation(data, tool_data, canvas_transform));
				}

				self
			}
			(state, Event::KeyDown(Key::KeyAlt)) => {
				data.center_around_cursor = true;

				if state == EllipseToolFsmState::LmbDown {
					operations.push(Operation::ClearWorkingFolder);
					operations.push(make_operation(data, tool_data, canvas_transform));
				}

				self
			}
			(state, Event::KeyUp(Key::KeyAlt)) => {
				data.center_around_cursor = false;

				if state == EllipseToolFsmState::LmbDown {
					operations.push(Operation::ClearWorkingFolder);
					operations.push(make_operation(data, tool_data, canvas_transform));
				}

				self
			}
			_ => self,
		}
	}
}

fn make_operation(data: &EllipseToolData, tool_data: &DocumentToolData, canvas_transform: &CanvasTransform) -> Operation {
	let canvas_start = data.drag_start.to_canvas_position(canvas_transform);
	let canvas_end = data.drag_current.to_canvas_position(canvas_transform);
	let x0 = canvas_start.x;
	let y0 = canvas_start.y;
	let x1 = canvas_end.x;
	let y1 = canvas_end.y;

	if data.constrain_to_circle {
		let (cx, cy, r) = if data.center_around_cursor {
			(x0, y0, f64::hypot(x1 - x0, y1 - y0))
		} else {
			let diameter = f64::max((x1 - x0).abs(), (y1 - y0).abs());
			let (x2, y2) = (x0 + (x1 - x0).signum() * diameter, y0 + (y1 - y0).signum() * diameter);
			((x0 + x2) * 0.5, (y0 + y2) * 0.5, diameter * 0.5)
		};
		Operation::AddCircle {
			path: vec![],
			insert_index: -1,
			cx,
			cy,
			r,
			style: style::PathStyle::new(None, Some(style::Fill::new(tool_data.primary_color))),
		}
	} else {
		let (cx, cy, r_scale) = if data.center_around_cursor { (x0, y0, 1.0) } else { ((x0 + x1) * 0.5, (y0 + y1) * 0.5, 0.5) };
		let (rx, ry) = ((x1 - x0).abs() * r_scale, (y1 - y0).abs() * r_scale);
		Operation::AddEllipse {
			path: vec![],
			insert_index: -1,
			cx,
			cy,
			rx,
			ry,
			rot: 0.0,
			style: style::PathStyle::new(None, Some(style::Fill::new(tool_data.primary_color))),
		}
	}
}
