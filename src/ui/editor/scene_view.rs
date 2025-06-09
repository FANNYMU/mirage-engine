use egui::{Context, Ui, Color32, Vec2, Rect, Pos2, Stroke};
use crate::ui::editor::ui_components::{SceneViewTool, EntityTransform};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Scene view panel for editing scenes
pub struct SceneViewPanel {
    /// The scene view size
    pub scene_view_size: [f32; 2],
    /// The current scene view tool
    pub scene_view_tool: SceneViewTool,
    /// Entity transforms - shared between scene view and inspector
    pub entity_transforms: Arc<Mutex<HashMap<u32, EntityTransform>>>,
    /// The currently selected entity
    pub selected_entity: Option<u32>,
    /// Entity names
    pub entity_names: HashMap<u32, String>,
    /// Show grid in scene view
    pub show_grid: bool,
    /// Camera position
    pub camera_position: [f32; 2],
    /// Camera zoom
    pub camera_zoom: f32,
    /// Last mouse position for panning
    pub last_mouse_pos: Option<Pos2>,
    /// Active gizmo axis
    pub active_axis: Option<GizmoAxis>,
    /// Dirty flag to track changes
    pub dirty: bool,
}

/// Gizmo axis for transformation
#[derive(Clone, Copy, PartialEq)]
pub enum GizmoAxis {
    /// X axis (red)
    X,
    /// Y axis (green)
    Y,
    /// Z axis (blue)
    Z,
}

impl SceneViewPanel {
    /// Create a new scene view panel
    pub fn new() -> Self {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
            
        let mut transforms = HashMap::new();
        transforms.insert(1, EntityTransform {
            position: [0.0, 0.0, -10.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            last_update: current_time,
        });
        transforms.insert(2, EntityTransform {
            position: [5.0, 10.0, 0.0],
            rotation: [45.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            last_update: current_time,
        });
        transforms.insert(3, EntityTransform {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            last_update: current_time,
        });
        transforms.insert(4, EntityTransform {
            position: [0.0, -2.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [20.0, 1.0, 1.0],
            last_update: current_time,
        });
        transforms.insert(5, EntityTransform {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            last_update: current_time,
        });
        
        Self {
            scene_view_size: [0.0, 0.0],
            scene_view_tool: SceneViewTool::Select,
            entity_transforms: Arc::new(Mutex::new(transforms)),
            selected_entity: None,
            entity_names: HashMap::new(),
            show_grid: true,
            camera_position: [0.0, 0.0],
            camera_zoom: 1.0,
            last_mouse_pos: None,
            active_axis: None,
            dirty: false,
        }
    }
    
    /// Get entity transforms
    pub fn get_entity_transforms(&self) -> Arc<Mutex<HashMap<u32, EntityTransform>>> {
        self.entity_transforms.clone()
    }
    
    /// Set entity names reference
    pub fn set_entity_names(&mut self, entity_names: HashMap<u32, String>) {
        self.entity_names = entity_names;
    }
    
    /// Set selected entity
    pub fn set_selected_entity(&mut self, entity_id: Option<u32>) {
        self.selected_entity = entity_id;
    }
    
    /// Render the scene view
    pub fn render(&mut self, ui: &mut Ui, log_info: &mut dyn FnMut(&str)) {
        // Reset dirty flag di awal
        self.dirty = false;
        
        let available_size = ui.available_size();
        self.scene_view_size = [available_size.x, available_size.y];
        
        // Create scene view response area
        let (response, painter) = ui.allocate_painter(
            Vec2::new(self.scene_view_size[0], self.scene_view_size[1]),
            egui::Sense::click_and_drag(),
        );
        
        let rect = response.rect;
        
        // Draw background
        painter.rect_filled(rect, 0.0, Color32::from_rgb(40, 40, 40));
        
        // Draw grid if enabled
        if self.show_grid {
            self.draw_unity_style_grid(ui, rect);
        }
        
        // Handle camera movement with middle mouse button
        if response.dragged_by(egui::PointerButton::Middle) {
            if let Some(_mouse_pos) = self.last_mouse_pos {
                let delta = response.drag_delta();
                self.camera_position[0] -= delta.x * 0.01 / self.camera_zoom;
                self.camera_position[1] += delta.y * 0.01 / self.camera_zoom;
            }
            self.last_mouse_pos = response.hover_pos();
        } else {
            self.last_mouse_pos = None;
        }
        
        // Handle zooming with scroll
        if response.hovered() {
            let scroll_delta = ui.input(|i| i.scroll_delta.y);
            if scroll_delta != 0.0 {
                self.camera_zoom *= (1.0 + scroll_delta * 0.001).max(0.1).min(10.0);
                log_info(&format!("Camera zoom: {:.2}", self.camera_zoom));
            }
        }
        
        // Draw the scene contents - read the shared entity transforms
        {
            self.draw_mock_scene(ui, rect);
        }
        
        // Handle click selection
        if response.clicked() {
            let click_pos = response.interact_pointer_pos.unwrap();
            let mut entity_clicked = false;
            
            // Check if clicked on an entity
            {
                let transforms = self.entity_transforms.lock().unwrap();
                for (&id, _) in transforms.iter() {
                    if self.is_point_in_entity(click_pos, rect, id) {
                        self.selected_entity = Some(id);
                        entity_clicked = true;
                        if let Some(name) = self.entity_names.get(&id) {
                            log_info(&format!("Selected entity: {}", name));
                        }
                        break;
                    }
                }
            }
            
            // If clicked on empty space, deselect
            if !entity_clicked {
                self.selected_entity = None;
            }
        }
        
        // Handle transformation tools
        if let Some(entity_id) = self.selected_entity {
            // Draw transform gizmo based on current tool
            match self.scene_view_tool {
                SceneViewTool::Move => self.draw_move_gizmo(ui, rect, entity_id),
                SceneViewTool::Rotate => self.draw_rotate_gizmo(ui, rect, entity_id),
                SceneViewTool::Scale => self.draw_scale_gizmo(ui, rect, entity_id),
                _ => {}
            }
            
            // Handle dragging for transform tools
            if response.dragged() && self.scene_view_tool == SceneViewTool::Move {
                if let Some(entity_id) = self.selected_entity {
                    let mut transforms = self.entity_transforms.lock().unwrap();
                    if let Some(transform) = transforms.get_mut(&entity_id) {
                        let delta = response.drag_delta();
                        let mut position_changed = false;
                        
                        // Apply transformation based on active axis
                        match self.active_axis {
                            Some(GizmoAxis::X) => {
                                transform.position[0] += delta.x * 0.01 / self.camera_zoom;
                                position_changed = true;
                                log_info(&format!("Moving {} along X axis", entity_id));
                            }
                            Some(GizmoAxis::Y) => {
                                transform.position[1] -= delta.y * 0.01 / self.camera_zoom;
                                position_changed = true;
                                log_info(&format!("Moving {} along Y axis", entity_id));
                            }
                            Some(GizmoAxis::Z) => {
                                // Z axis movement would depend on the view projection
                                transform.position[2] += (delta.x - delta.y) * 0.005 / self.camera_zoom;
                                position_changed = true;
                                log_info(&format!("Moving {} along Z axis", entity_id));
                            }
                            None => {
                                // Move in all axes
                                transform.position[0] += delta.x * 0.01 / self.camera_zoom;
                                transform.position[1] -= delta.y * 0.01 / self.camera_zoom;
                                position_changed = true;
                            }
                        }
                        
                        // Set dirty flag if position changed
                        if position_changed {
                            self.dirty = true;
                        }
                    }
                }
            }
            
            // Handle scale
            if response.dragged() && self.scene_view_tool == SceneViewTool::Scale {
                let mut transforms = self.entity_transforms.lock().unwrap();
                if let Some(transform) = transforms.get_mut(&entity_id) {
                    let delta = response.drag_delta();
                    let mut scale_changed = false;
                    
                    // Apply scale based on active axis
                    match self.active_axis {
                        Some(GizmoAxis::X) => {
                            transform.scale[0] += delta.x * 0.01;
                            if transform.scale[0] < 0.1 { transform.scale[0] = 0.1; }
                            scale_changed = true;
                            log_info(&format!("Scaling {} along X axis", entity_id));
                        }
                        Some(GizmoAxis::Y) => {
                            transform.scale[1] -= delta.y * 0.01;
                            if transform.scale[1] < 0.1 { transform.scale[1] = 0.1; }
                            scale_changed = true;
                            log_info(&format!("Scaling {} along Y axis", entity_id));
                        }
                        Some(GizmoAxis::Z) => {
                            transform.scale[2] += (delta.x - delta.y) * 0.005;
                            if transform.scale[2] < 0.1 { transform.scale[2] = 0.1; }
                            scale_changed = true;
                            log_info(&format!("Scaling {} along Z axis", entity_id));
                        }
                        None => {
                            // Uniform scale
                            let scale_factor = (delta.x + delta.y) * 0.01;
                            transform.scale[0] += scale_factor;
                            transform.scale[1] += scale_factor;
                            transform.scale[2] += scale_factor;
                            
                            // Ensure minimum scale
                            if transform.scale[0] < 0.1 { transform.scale[0] = 0.1; }
                            if transform.scale[1] < 0.1 { transform.scale[1] = 0.1; }
                            if transform.scale[2] < 0.1 { transform.scale[2] = 0.1; }
                            
                            scale_changed = true;
                        }
                    }
                    
                    // Set dirty flag if scale changed
                    if scale_changed {
                        self.dirty = true;
                    }
                }
            }
            
            // Handle rotation
            if response.dragged() && self.scene_view_tool == SceneViewTool::Rotate {
                let mut transforms = self.entity_transforms.lock().unwrap();
                if let Some(transform) = transforms.get_mut(&entity_id) {
                    let delta = response.drag_delta();
                    let mut rotation_changed = false;
                    
                    // Apply rotation based on active axis or general rotation
                    match self.active_axis {
                        Some(GizmoAxis::X) => {
                            transform.rotation[0] += delta.y * 0.5;
                            rotation_changed = true;
                            log_info(&format!("Rotating {} around X axis", entity_id));
                        }
                        Some(GizmoAxis::Y) => {
                            transform.rotation[1] += delta.x * 0.5;
                            rotation_changed = true;
                            log_info(&format!("Rotating {} around Y axis", entity_id));
                        }
                        Some(GizmoAxis::Z) => {
                            transform.rotation[2] += (delta.x - delta.y) * 0.5;
                            rotation_changed = true;
                            log_info(&format!("Rotating {} around Z axis", entity_id));
                        }
                        None => {
                            // General rotation based on mouse movement
                            transform.rotation[1] += delta.x * 0.5;
                            transform.rotation[0] += delta.y * 0.5;
                            rotation_changed = true;
                        }
                    }
                    
                    // Normalize angles to 0-360 degrees
                    for i in 0..3 {
                        while transform.rotation[i] >= 360.0 {
                            transform.rotation[i] -= 360.0;
                        }
                        while transform.rotation[i] < 0.0 {
                            transform.rotation[i] += 360.0;
                        }
                    }
                    
                    // Set dirty flag if rotation changed
                    if rotation_changed {
                        self.dirty = true;
                    }
                }
            }
        }
        
        // Draw Unity-style viewport overlay
        self.draw_viewport_overlay(ui, rect);
    }
    
    /// Draw Unity-style grid
    pub fn draw_unity_style_grid(&self, ui: &mut Ui, rect: Rect) {
        let painter = ui.painter();
        
        let grid_size = 50.0 * self.camera_zoom;
        let grid_color_major = Color32::from_rgb(80, 80, 80);
        let grid_color_minor = Color32::from_rgb(60, 60, 60);
        
        let center_x = rect.center().x + self.camera_position[0] * 50.0 * self.camera_zoom;
        let center_y = rect.center().y - self.camera_position[1] * 50.0 * self.camera_zoom;
        
        // Calculate grid boundaries
        let left = rect.left();
        let right = rect.right();
        let top = rect.top();
        let bottom = rect.bottom();
        
        // Calculate grid starting positions (aligned to grid_size)
        let start_x = center_x - (((center_x - left) / grid_size).floor() * grid_size);
        let start_y = center_y - (((center_y - top) / grid_size).floor() * grid_size);
        
        // Draw minor grid lines
        let minor_grid_size = grid_size / 5.0;
        let minor_start_x = center_x - (((center_x - left) / minor_grid_size).floor() * minor_grid_size);
        let minor_start_y = center_y - (((center_y - top) / minor_grid_size).floor() * minor_grid_size);
        
        // Draw minor grid lines
        for x in (minor_start_x as i32..=right as i32).step_by(minor_grid_size as usize) {
            let x = x as f32;
            if (x - center_x).abs() % grid_size < 1.0 {
                continue; // Skip where major lines will be
            }
            painter.line_segment(
                [Pos2::new(x, top), Pos2::new(x, bottom)],
                Stroke::new(1.0, grid_color_minor),
            );
        }
        
        for y in (minor_start_y as i32..=bottom as i32).step_by(minor_grid_size as usize) {
            let y = y as f32;
            if (y - center_y).abs() % grid_size < 1.0 {
                continue; // Skip where major lines will be
            }
            painter.line_segment(
                [Pos2::new(left, y), Pos2::new(right, y)],
                Stroke::new(1.0, grid_color_minor),
            );
        }
        
        // Draw major vertical lines
        for x in (start_x as i32..=right as i32).step_by(grid_size as usize) {
            let x = x as f32;
            let is_center = (x - center_x).abs() < 1.0;
            let color = if is_center { Color32::from_rgb(150, 20, 20) } else { grid_color_major };
            let stroke_width = if is_center { 2.0 } else { 1.0 };
            
            painter.line_segment(
                [Pos2::new(x, top), Pos2::new(x, bottom)],
                Stroke::new(stroke_width, color),
            );
        }
        
        // Draw major horizontal lines
        for y in (start_y as i32..=bottom as i32).step_by(grid_size as usize) {
            let y = y as f32;
            let is_center = (y - center_y).abs() < 1.0;
            let color = if is_center { Color32::from_rgb(20, 150, 20) } else { grid_color_major };
            let stroke_width = if is_center { 2.0 } else { 1.0 };
            
            painter.line_segment(
                [Pos2::new(left, y), Pos2::new(right, y)],
                Stroke::new(stroke_width, color),
            );
        }
    }
    
    /// Draw move gizmo
    fn draw_move_gizmo(&mut self, ui: &mut Ui, rect: Rect, entity_id: u32) {
        let painter = ui.painter();
        
        if let Some(transform) = self.entity_transforms.lock().unwrap().get(&entity_id) {
            let center_x = rect.center().x + (transform.position[0] + self.camera_position[0]) * 50.0 * self.camera_zoom;
            let center_y = rect.center().y - (transform.position[1] + self.camera_position[1]) * 50.0 * self.camera_zoom;
            
            let axis_length = 50.0 * self.camera_zoom;
            let axis_width = 3.0;
            let arrow_size = 10.0 * self.camera_zoom;
            
            // X axis (red)
            let x_end = center_x + axis_length;
            painter.line_segment(
                [Pos2::new(center_x, center_y), Pos2::new(x_end, center_y)],
                Stroke::new(axis_width, Color32::from_rgb(255, 0, 0)),
            );
            
            // X axis arrowhead
            let x_arrow = [
                Pos2::new(x_end, center_y),
                Pos2::new(x_end - arrow_size, center_y - arrow_size/2.0),
                Pos2::new(x_end - arrow_size, center_y + arrow_size/2.0),
            ];
            painter.add(egui::Shape::convex_polygon(
                x_arrow.to_vec(),
                Color32::from_rgb(255, 0, 0),
                Stroke::NONE,
            ));
            
            // Y axis (green)
            let y_end = center_y - axis_length;
            painter.line_segment(
                [Pos2::new(center_x, center_y), Pos2::new(center_x, y_end)],
                Stroke::new(axis_width, Color32::from_rgb(0, 255, 0)),
            );
            
            // Y axis arrowhead
            let y_arrow = [
                Pos2::new(center_x, y_end),
                Pos2::new(center_x - arrow_size/2.0, y_end + arrow_size),
                Pos2::new(center_x + arrow_size/2.0, y_end + arrow_size),
            ];
            painter.add(egui::Shape::convex_polygon(
                y_arrow.to_vec(),
                Color32::from_rgb(0, 255, 0),
                Stroke::NONE,
            ));
            
            // Z axis (blue) - shorter for perspective
            let z_length = axis_length * 0.7;
            let z_end_x = center_x + z_length * 0.7;
            let z_end_y = center_y - z_length * 0.7;
            painter.line_segment(
                [Pos2::new(center_x, center_y), Pos2::new(z_end_x, z_end_y)],
                Stroke::new(axis_width, Color32::from_rgb(0, 0, 255)),
            );
            
            // Check if mouse is hovering over any axis
            if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                // Check X axis
                let x_axis_rect = Rect::from_min_max(
                    Pos2::new(center_x, center_y - axis_width),
                    Pos2::new(x_end, center_y + axis_width),
                );
                if x_axis_rect.contains(mouse_pos) {
                    self.active_axis = Some(GizmoAxis::X);
                    painter.line_segment(
                        [Pos2::new(center_x, center_y), Pos2::new(x_end, center_y)],
                        Stroke::new(axis_width + 2.0, Color32::from_rgb(255, 100, 100)),
                    );
                }
                // Check Y axis
                let y_axis_rect = Rect::from_min_max(
                    Pos2::new(center_x - axis_width, y_end),
                    Pos2::new(center_x + axis_width, center_y),
                );
                if y_axis_rect.contains(mouse_pos) {
                    self.active_axis = Some(GizmoAxis::Y);
                    painter.line_segment(
                        [Pos2::new(center_x, center_y), Pos2::new(center_x, y_end)],
                        Stroke::new(axis_width + 2.0, Color32::from_rgb(100, 255, 100)),
                    );
                }
                // Check Z axis
                let dist_to_z = mouse_pos.distance(Pos2::new(
                    center_x + (mouse_pos.x - center_x) * 0.5,
                    center_y + (mouse_pos.y - center_y) * 0.5,
                ));
                if dist_to_z < 5.0 {
                    self.active_axis = Some(GizmoAxis::Z);
                    painter.line_segment(
                        [Pos2::new(center_x, center_y), Pos2::new(z_end_x, z_end_y)],
                        Stroke::new(axis_width + 2.0, Color32::from_rgb(100, 100, 255)),
                    );
                }
            }
        }
    }
    
    /// Draw rotate gizmo
    fn draw_rotate_gizmo(&mut self, ui: &mut Ui, rect: Rect, entity_id: u32) {
        let painter = ui.painter();
        
        if let Some(transform) = self.entity_transforms.lock().unwrap().get(&entity_id) {
            let center_x = rect.center().x + (transform.position[0] + self.camera_position[0]) * 50.0 * self.camera_zoom;
            let center_y = rect.center().y - (transform.position[1] + self.camera_position[1]) * 50.0 * self.camera_zoom;
            
            let radius = 40.0 * self.camera_zoom;
            
            // X rotation circle (red)
            painter.circle_stroke(
                Pos2::new(center_x, center_y),
                radius,
                Stroke::new(2.0, Color32::from_rgb(255, 0, 0)),
            );
            
            // Y rotation circle (green) - drawn as an ellipse to simulate perspective
            let points_y = (0..=32).map(|i| {
                let angle = i as f32 * 2.0 * std::f32::consts::PI / 32.0;
                let x = center_x + radius * angle.cos() * 0.5;
                let y = center_y + radius * angle.sin();
                Pos2::new(x, y)
            }).collect::<Vec<_>>();
            
            for i in 0..points_y.len()-1 {
                painter.line_segment(
                    [points_y[i], points_y[i+1]],
                    Stroke::new(2.0, Color32::from_rgb(0, 255, 0)),
                );
            }
            
            // Z rotation circle (blue) - drawn as an ellipse to simulate perspective
            let points_z = (0..=32).map(|i| {
                let angle = i as f32 * 2.0 * std::f32::consts::PI / 32.0;
                let x = center_x + radius * angle.cos();
                let y = center_y + radius * angle.sin() * 0.5;
                Pos2::new(x, y)
            }).collect::<Vec<_>>();
            
            for i in 0..points_z.len()-1 {
                painter.line_segment(
                    [points_z[i], points_z[i+1]],
                    Stroke::new(2.0, Color32::from_rgb(0, 0, 255)),
                );
            }
        }
    }
    
    /// Draw scale gizmo
    fn draw_scale_gizmo(&mut self, ui: &mut Ui, rect: Rect, entity_id: u32) {
        let painter = ui.painter();
        
        if let Some(transform) = self.entity_transforms.lock().unwrap().get(&entity_id) {
            let center_x = rect.center().x + (transform.position[0] + self.camera_position[0]) * 50.0 * self.camera_zoom;
            let center_y = rect.center().y - (transform.position[1] + self.camera_position[1]) * 50.0 * self.camera_zoom;
            
            let axis_length = 40.0 * self.camera_zoom;
            let box_size = 10.0 * self.camera_zoom;
            
            // X axis (red)
            let x_end = center_x + axis_length;
            painter.line_segment(
                [Pos2::new(center_x, center_y), Pos2::new(x_end - box_size/2.0, center_y)],
                Stroke::new(2.0, Color32::from_rgb(255, 0, 0)),
            );
            
            // X handle
            painter.rect_filled(
                Rect::from_center_size(
                    Pos2::new(x_end, center_y),
                    Vec2::new(box_size, box_size)
                ),
                0.0, Color32::from_rgb(255, 0, 0)
            );
            
            // Y axis (green)
            let y_end = center_y - axis_length;
            painter.line_segment(
                [Pos2::new(center_x, center_y), Pos2::new(center_x, y_end + box_size/2.0)],
                Stroke::new(2.0, Color32::from_rgb(0, 255, 0)),
            );
            
            // Y handle
            painter.rect_filled(
                Rect::from_center_size(
                    Pos2::new(center_x, y_end),
                    Vec2::new(box_size, box_size)
                ),
                0.0, Color32::from_rgb(0, 255, 0)
            );
            
            // Z axis (blue) - shorter for perspective
            let z_length = axis_length * 0.7;
            let z_end_x = center_x + z_length * 0.7;
            let z_end_y = center_y - z_length * 0.7;
            painter.line_segment(
                [Pos2::new(center_x, center_y), Pos2::new(z_end_x - box_size/2.0, z_end_y + box_size/2.0)],
                Stroke::new(2.0, Color32::from_rgb(0, 0, 255)),
            );
            
            // Z handle
            painter.rect_filled(
                Rect::from_center_size(
                    Pos2::new(z_end_x, z_end_y),
                    Vec2::new(box_size, box_size)
                ),
                0.0, Color32::from_rgb(0, 0, 255)
            );
            
            // Center uniform scale handle
            painter.rect_filled(
                Rect::from_center_size(
                    Pos2::new(center_x, center_y),
                    Vec2::new(box_size, box_size)
                ),
                0.0, Color32::from_rgb(255, 255, 0)
            );
        }
    }
    
    /// Draw viewport overlay with orientation gizmo and info
    fn draw_viewport_overlay(&self, ui: &mut Ui, rect: Rect) {
        let painter = ui.painter();
        
        // Draw orientation gizmo in the top right corner
        let gizmo_size = 50.0;
        let gizmo_pos = Pos2::new(rect.right() - gizmo_size - 10.0, rect.top() + gizmo_size + 10.0);
        
        // Draw gizmo lines
        let x_end = gizmo_pos.x + 20.0;
        let y_end = gizmo_pos.y - 20.0;
        let z_end_x = gizmo_pos.x + 10.0;
        let z_end_y = gizmo_pos.y - 10.0;
        
        // X axis
        painter.line_segment(
            [gizmo_pos, Pos2::new(x_end, gizmo_pos.y)],
            Stroke::new(2.0, Color32::from_rgb(255, 0, 0)),
        );
        painter.text(
            Pos2::new(x_end + 5.0, gizmo_pos.y),
            egui::Align2::LEFT_CENTER,
            "X",
            egui::FontId::default(),
            Color32::from_rgb(255, 0, 0),
        );
        
        // Y axis
        painter.line_segment(
            [gizmo_pos, Pos2::new(gizmo_pos.x, y_end)],
            Stroke::new(2.0, Color32::from_rgb(0, 255, 0)),
        );
        painter.text(
            Pos2::new(gizmo_pos.x, y_end - 5.0),
            egui::Align2::CENTER_BOTTOM,
            "Y",
            egui::FontId::default(),
            Color32::from_rgb(0, 255, 0),
        );
        
        // Z axis
        painter.line_segment(
            [gizmo_pos, Pos2::new(z_end_x, z_end_y)],
            Stroke::new(2.0, Color32::from_rgb(0, 0, 255)),
        );
        painter.text(
            Pos2::new(z_end_x + 5.0, z_end_y - 5.0),
            egui::Align2::LEFT_BOTTOM,
            "Z",
            egui::FontId::default(),
            Color32::from_rgb(0, 0, 255),
        );
        
        // Camera info text at bottom left
        if let Some(entity_id) = self.selected_entity {
            if let Some(transform) = self.entity_transforms.lock().unwrap().get(&entity_id) {
                // Create a persistent string for the entity name to avoid temporary value drop
                let entity_name_str = match self.entity_names.get(&entity_id) {
                    Some(name) => name.clone(),
                    None => format!("Entity {}", entity_id),
                };
                
                painter.text(
                    Pos2::new(rect.left() + 10.0, rect.bottom() - 30.0),
                    egui::Align2::LEFT_BOTTOM,
                    format!("Position: ({:.2}, {:.2}, {:.2})", transform.position[0], transform.position[1], transform.position[2]),
                    egui::FontId::default(),
                    Color32::WHITE,
                );
                painter.text(
                    Pos2::new(rect.left() + 10.0, rect.bottom() - 50.0),
                    egui::Align2::LEFT_BOTTOM,
                    format!("Selected: {}", entity_name_str),
                    egui::FontId::default(),
                    Color32::WHITE,
                );
            }
        }
        
        // Draw camera info
        painter.text(
            Pos2::new(rect.left() + 10.0, rect.bottom() - 10.0),
            egui::Align2::LEFT_BOTTOM,
            format!("Camera: ({:.2}, {:.2}) | Zoom: {:.1}x", 
                self.camera_position[0], self.camera_position[1], self.camera_zoom),
            egui::FontId::default(),
            Color32::WHITE,
        );
    }
    
    /// Draw a mock scene for visualization
    pub fn draw_mock_scene(&self, ui: &mut Ui, rect: Rect) {
        // Salin transformasi entitas dari shared state
        let entity_transforms = {
            self.entity_transforms.lock().unwrap().clone()
        };
        
        // Dapatkan semua entity ID dari transforms dan urutkan berdasarkan Z
        let mut entities_with_z = Vec::new();
        for (&id, transform) in entity_transforms.iter() {
            entities_with_z.push((id, transform.position[2]));
        }
        
        // Urutkan berdasarkan z-position (yang lebih jauh/kecil duluan)
        entities_with_z.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        
        // Gambar grid terlebih dahulu jika opsi grid diaktifkan
        if self.show_grid {
            self.draw_grid(ui, rect);
        }
        
        // Gambar setiap entitas berdasarkan urutan Z (dari belakang ke depan)
        // Ini membuat entitas yang lebih dekat (Z lebih besar) menimpa entitas yang lebih jauh
        for (entity_id, _) in entities_with_z {
            // Debug print untuk membantu pelacakan render order
            if let Some(name) = self.entity_names.get(&entity_id) {
                if let Some(transform) = entity_transforms.get(&entity_id) {
                    println!("Rendering: {} (ID: {}) at Z: {:.3}", name, entity_id, transform.position[2]);
                }
            }
            
            self.draw_entity(ui, rect, entity_id);
        }
        
        // Draw manipulator pada objek yang dipilih jika ada
        if let Some(entity_id) = self.selected_entity {
            self.draw_manipulator(ui, rect, entity_id);
        }
        
        // Tampilkan informasi scene view di pojok kiri bawah
        self.draw_scene_info(ui, rect);
    }
    
    /// Draw a grid in the scene view
    fn draw_grid(&self, ui: &mut Ui, rect: Rect) {
        let painter = ui.painter();
        
        // Definisikan parameter grid
        let grid_size = 50.0; // Ukuran grid dalam pixel
        let grid_color = Color32::from_rgba_premultiplied(80, 80, 80, 180);
        let axis_color_x = Color32::from_rgba_premultiplied(200, 80, 80, 180); // Merah untuk sumbu X
        let axis_color_z = Color32::from_rgba_premultiplied(80, 80, 200, 180); // Biru untuk sumbu Z
        
        let center_x = rect.center().x;
        let center_y = rect.center().y;
        
        // Gambar grid dengan memperhitungkan camera position dan zoom
        let scaled_grid_size = grid_size * self.camera_zoom;
        let offset_x = (self.camera_position[0] * scaled_grid_size) % scaled_grid_size;
        let offset_y = (self.camera_position[1] * scaled_grid_size) % scaled_grid_size;
        
        // Gambar grid horizontal
        for i in -50..50 {
            let y = center_y + i as f32 * scaled_grid_size - offset_y;
            
            // Skip jika di luar rect
            if y < rect.min.y || y > rect.max.y {
                continue;
            }
            
            // Gunakan warna khusus untuk axis Z (horizontal tengah)
            let line_color = if i == 0 { axis_color_z } else { grid_color };
            
            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                Stroke::new(if i == 0 { 2.0 } else { 1.0 }, line_color),
            );
        }
        
        // Gambar grid vertikal
        for i in -50..50 {
            let x = center_x + i as f32 * scaled_grid_size + offset_x;
            
            // Skip jika di luar rect
            if x < rect.min.x || x > rect.max.x {
                continue;
            }
            
            // Gunakan warna khusus untuk axis X (vertikal tengah)
            let line_color = if i == 0 { axis_color_x } else { grid_color };
            
            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(if i == 0 { 2.0 } else { 1.0 }, line_color),
            );
        }
    }
    
    /// Draw a manipulator gizmo for the selected entity
    fn draw_manipulator(&self, ui: &mut Ui, rect: Rect, entity_id: u32) {
        // Hanya tampilkan manipulator jika mode Select tidak aktif
        if self.scene_view_tool == SceneViewTool::Select {
            return;
        }
        
        let painter = ui.painter();
        
        let transforms = self.entity_transforms.lock().unwrap();
        if let Some(transform) = transforms.get(&entity_id) {
            // Konversi posisi entity ke screen space
            let scale_factor = 50.0;
            let center_x = rect.center().x;
            let center_y = rect.center().y;
            
            let pos_x = center_x + (transform.position[0] + self.camera_position[0]) * scale_factor * self.camera_zoom;
            let pos_y = center_y - (transform.position[1] + self.camera_position[1]) * scale_factor * self.camera_zoom;
            
            // Gambar gizmo berdasarkan tool yang aktif
            match self.scene_view_tool {
                SceneViewTool::Move => {
                    // Gambar arrow untuk Move gizmo
                    let arrow_length = 30.0 * self.camera_zoom;
                    let arrow_width = 5.0 * self.camera_zoom;
                    
                    // X-axis arrow (red)
                    painter.line_segment(
                        [Pos2::new(pos_x, pos_y), Pos2::new(pos_x + arrow_length, pos_y)],
                        Stroke::new(arrow_width, Color32::RED),
                    );
                    
                    // Y-axis arrow (green)
                    painter.line_segment(
                        [Pos2::new(pos_x, pos_y), Pos2::new(pos_x, pos_y - arrow_length)],
                        Stroke::new(arrow_width, Color32::GREEN),
                    );
                },
                SceneViewTool::Rotate => {
                    // Gambar rotation gizmo
                    let radius = 25.0 * self.camera_zoom;
                    painter.circle_stroke(
                        Pos2::new(pos_x, pos_y),
                        radius,
                        Stroke::new(2.0, Color32::YELLOW),
                    );
                    
                    // Tambahkan indikator untuk rotation saat ini
                    let rotation_rad = transform.rotation[2].to_radians();
                    let indicator_x = pos_x + radius * rotation_rad.cos();
                    let indicator_y = pos_y - radius * rotation_rad.sin();
                    
                    painter.circle_filled(
                        Pos2::new(indicator_x, indicator_y),
                        5.0 * self.camera_zoom,
                        Color32::YELLOW,
                    );
                },
                SceneViewTool::Scale => {
                    // Gambar scale gizmo
                    let size = 20.0 * self.camera_zoom;
                    
                    // Gambar uniform scale box
                    painter.rect_stroke(
                        Rect::from_center_size(
                            Pos2::new(pos_x, pos_y),
                            Vec2::new(size, size),
                        ),
                        0.0,
                        Stroke::new(2.0, Color32::LIGHT_BLUE),
                    );
                    
                    // Tambahkan corner handles
                    let corner_size = 5.0 * self.camera_zoom;
                    
                    // Top-right corner
                    painter.rect_filled(
                        Rect::from_center_size(
                            Pos2::new(pos_x + size/2.0, pos_y - size/2.0),
                            Vec2::new(corner_size, corner_size),
                        ),
                        0.0,
                        Color32::WHITE,
                    );
                    
                    // Bottom-right corner
                    painter.rect_filled(
                        Rect::from_center_size(
                            Pos2::new(pos_x + size/2.0, pos_y + size/2.0),
                            Vec2::new(corner_size, corner_size),
                        ),
                        0.0,
                        Color32::WHITE,
                    );
                    
                    // Bottom-left corner
                    painter.rect_filled(
                        Rect::from_center_size(
                            Pos2::new(pos_x - size/2.0, pos_y + size/2.0),
                            Vec2::new(corner_size, corner_size),
                        ),
                        0.0,
                        Color32::WHITE,
                    );
                    
                    // Top-left corner
                    painter.rect_filled(
                        Rect::from_center_size(
                            Pos2::new(pos_x - size/2.0, pos_y - size/2.0),
                            Vec2::new(corner_size, corner_size),
                        ),
                        0.0,
                        Color32::WHITE,
                    );
                },
                _ => {}
            }
        }
    }
    
    /// Draw scene information in the bottom left corner
    fn draw_scene_info(&self, ui: &mut Ui, rect: Rect) {
        let painter = ui.painter();
        
        // Tampilkan informasi kamera
        let info_text = format!(
            "Camera Position: ({:.2}, {:.2}) | Zoom: {:.2}x", 
            self.camera_position[0], 
            self.camera_position[1],
            self.camera_zoom
        );
        
        // Tambahkan informasi entity yang dipilih
        let mut selection_text = String::from("No entity selected");
        if let Some(entity_id) = self.selected_entity {
            if let Some(name) = self.entity_names.get(&entity_id) {
                if let Some(transform) = self.entity_transforms.lock().unwrap().get(&entity_id) {
                    selection_text = format!(
                        "Selected: {} (ID: {}) | Position: ({:.2}, {:.2}, {:.2})", 
                        name, 
                        entity_id,
                        transform.position[0],
                        transform.position[1],
                        transform.position[2]
                    );
                }
            }
        }
        
        // Render teks dengan background semi-transparan
        let font_id = egui::FontId::proportional(14.0);
        let padding = 5.0;
        
        // Info kamera
        let info_galley = ui.painter().layout_no_wrap(
            info_text.clone(),
            font_id.clone(),
            Color32::WHITE,
        );
        
        let info_rect = Rect::from_min_size(
            Pos2::new(rect.min.x + padding, rect.max.y - info_galley.rect.height() - padding * 2.0 - 20.0),
            Vec2::new(info_galley.rect.width() + padding * 2.0, info_galley.rect.height() + padding * 2.0),
        );
        
        painter.rect_filled(
            info_rect,
            3.0,
            Color32::from_rgba_premultiplied(0, 0, 0, 180),
        );
        
        painter.text(
            Pos2::new(info_rect.min.x + padding, info_rect.min.y + padding),
            egui::Align2::LEFT_TOP,
            info_text,
            font_id.clone(),
            Color32::WHITE,
        );
        
        // Selection info
        let selection_galley = ui.painter().layout_no_wrap(
            selection_text.clone(),
            font_id.clone(),
            Color32::WHITE,
        );
        
        let selection_rect = Rect::from_min_size(
            Pos2::new(rect.min.x + padding, rect.max.y - selection_galley.rect.height() - padding * 2.0),
            Vec2::new(selection_galley.rect.width() + padding * 2.0, selection_galley.rect.height() + padding * 2.0),
        );
        
        painter.rect_filled(
            selection_rect,
            3.0,
            Color32::from_rgba_premultiplied(0, 0, 0, 180),
        );
        
        painter.text(
            Pos2::new(selection_rect.min.x + padding, selection_rect.min.y + padding),
            egui::Align2::LEFT_TOP,
            selection_text,
            font_id,
            Color32::WHITE,
        );
    }
    
    /// Handle input for the scene view panel
    fn handle_input(&mut self, ui: &mut Ui, rect: Rect) -> bool {
        let mut response = ui.interact(rect, ui.id().with("scene_view"), egui::Sense::click_and_drag());
        let mut scene_changed = false;
        
        let camera_speed = 0.05;
        let zoom_speed = 0.1;
        
        // Handle panning with middle mouse or Alt + left mouse
        if ui.input(|i| i.pointer.middle_down()) || 
           (ui.input(|i| i.pointer.primary_down()) && ui.input(|i| i.modifiers.alt)) {
            let delta = ui.input(|i| i.pointer.delta());
            self.camera_position[0] -= delta.x * camera_speed / self.camera_zoom;
            self.camera_position[1] += delta.y * camera_speed / self.camera_zoom;
            scene_changed = true;
        }
        
        // Handle zooming with scroll wheel
        let scroll_delta = ui.input(|i| i.scroll_delta.y);
        if scroll_delta != 0.0 {
            let old_zoom = self.camera_zoom;
            self.camera_zoom *= 1.0 + scroll_delta * zoom_speed * 0.01;
            self.camera_zoom = self.camera_zoom.clamp(0.1, 10.0);
            
            // Adjust zoom around cursor position
            if ui.input(|i| i.pointer.has_pointer()) {
                if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                    if rect.contains(mouse_pos) {
                        let center_x = rect.center().x;
                        let center_y = rect.center().y;
                        let dx = (mouse_pos.x - center_x) / old_zoom;
                        let dy = (mouse_pos.y - center_y) / old_zoom;
                        
                        let zoom_ratio = 1.0 - self.camera_zoom / old_zoom;
                        self.camera_position[0] += dx * zoom_ratio;
                        self.camera_position[1] -= dy * zoom_ratio;
                    }
                }
            }
            scene_changed = true;
        }
        
        // Handle selection with left click
        if response.clicked() {
            // Find which entity was clicked
            let mut clicked_entity = None;
            let mut closest_z = std::f32::NEG_INFINITY;
            
            if let Some(mouse_pos) = ui.input(|i| i.pointer.interact_pos()) {
                // Get all entities from transforms
                let transforms = self.entity_transforms.lock().unwrap();
                
                // Convert all entities to screen space and check for click
                for (&id, transform) in transforms.iter() {
                    let scale_factor = 50.0;
                    let center_x = rect.center().x;
                    let center_y = rect.center().y;
                    
                    let z_offset = transform.position[2];
                    let perspective_scale = 1.0 + z_offset * 0.02;
                    let perspective_zoom = if perspective_scale <= 0.1 { 0.1 } else { perspective_scale };
                    
                    let pos_x = center_x + (transform.position[0] + self.camera_position[0]) * scale_factor * self.camera_zoom;
                    let pos_y = center_y - (transform.position[1] + self.camera_position[1]) * scale_factor * self.camera_zoom;
                    
                    let base_size = 30.0 * self.camera_zoom * perspective_zoom;
                    let size_x = base_size * transform.scale[0];
                    let size_y = base_size * transform.scale[1];
                    
                    // Custom hit testing based on entity type
                    let is_hit = match id {
                        1 => { // Camera
                            let camera_width = 20.0 * self.camera_zoom * perspective_zoom;
                            let camera_height = 12.0 * self.camera_zoom * perspective_zoom;
                            
                            let camera_rect = Rect::from_center_size(
                                Pos2::new(pos_x, pos_y),
                                Vec2::new(camera_width + 10.0, camera_height + 10.0),
                            );
                            
                            camera_rect.contains(mouse_pos)
                        },
                        2 => { // Light
                            let light_radius = 15.0 * self.camera_zoom * perspective_zoom;
                            let distance = ((pos_x - mouse_pos.x).powi(2) + (pos_y - mouse_pos.y).powi(2)).sqrt();
                            
                            distance <= light_radius
                        },
                        4 => { // Background - dengan bounding box yang lebih besar
                            let bg_rect = Rect::from_center_size(
                                Pos2::new(pos_x, pos_y),
                                Vec2::new(size_x, size_y),
                            );
                            
                            bg_rect.contains(mouse_pos)
                        },
                        _ => { // Default untuk entity lain (termasuk Player)
                            let entity_rect = Rect::from_center_size(
                                Pos2::new(pos_x, pos_y),
                                Vec2::new(size_x, size_y),
                            );
                            
                            entity_rect.contains(mouse_pos)
                        }
                    };
                    
                    // Jika entity terklik dan memiliki Z lebih besar dari entity yang sudah dipilih,
                    // update entity yang dipilih (memilih objek yang lebih depan)
                    if is_hit && transform.position[2] > closest_z {
                        clicked_entity = Some(id);
                        closest_z = transform.position[2];
                    }
                }
            }
            
            // Update selected entity
            if self.selected_entity != clicked_entity {
                println!("Selected entity changed from {:?} to {:?}", self.selected_entity, clicked_entity);
                self.selected_entity = clicked_entity;
                scene_changed = true;
            }
        }
        
        // Handle transform tool operations based on selected entity and current tool
        if let Some(entity_id) = self.selected_entity {
            // Handle transform tools based on scene_view_tool
            match self.scene_view_tool {
                SceneViewTool::Move => {
                    // Handle moving the entity when dragging
                    if response.dragged() && !ui.input(|i| i.modifiers.alt) {
                        let delta = ui.input(|i| i.pointer.delta());
                        let scale_factor = 50.0 * self.camera_zoom;
                        
                        let mut transforms = self.entity_transforms.lock().unwrap();
                        if let Some(transform) = transforms.get_mut(&entity_id) {
                            // Update transform position based on delta
                            transform.position[0] += delta.x / scale_factor;
                            transform.position[1] -= delta.y / scale_factor;
                            
                            // Update timestamp
                            transform.last_update = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs_f64();
                            
                            scene_changed = true;
                        }
                    }
                },
                SceneViewTool::Rotate => {
                    // Handle rotating the entity when dragging
                    if response.dragged() && !ui.input(|i| i.modifiers.alt) {
                        let delta = ui.input(|i| i.pointer.delta());
                        
                        let mut transforms = self.entity_transforms.lock().unwrap();
                        if let Some(transform) = transforms.get_mut(&entity_id) {
                            // Update rotation based on horizontal movement for Z rotation
                            transform.rotation[2] += delta.x * 0.5;
                            
                            // Wrap rotation to keep it in 0-360 range
                            while transform.rotation[2] < 0.0 {
                                transform.rotation[2] += 360.0;
                            }
                            while transform.rotation[2] >= 360.0 {
                                transform.rotation[2] -= 360.0;
                            }
                            
                            // Update timestamp
                            transform.last_update = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs_f64();
                            
                            scene_changed = true;
                        }
                    }
                },
                SceneViewTool::Scale => {
                    // Handle scaling the entity when dragging
                    if response.dragged() && !ui.input(|i| i.modifiers.alt) {
                        let delta = ui.input(|i| i.pointer.delta());
                        
                        let mut transforms = self.entity_transforms.lock().unwrap();
                        if let Some(transform) = transforms.get_mut(&entity_id) {
                            // Scale proportionally using diagonal movement
                            let scale_delta = (delta.x + delta.y) * 0.01;
                            transform.scale[0] = (transform.scale[0] + scale_delta).max(0.1);
                            transform.scale[1] = (transform.scale[1] + scale_delta).max(0.1);
                            
                            // Update timestamp
                            transform.last_update = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs_f64();
                            
                            scene_changed = true;
                        }
                    }
                },
                _ => {}
            }
        }
        
        scene_changed
    }
    
    /// Draw an entity with proper transformation
    fn draw_entity(&self, ui: &mut Ui, rect: Rect, entity_id: u32) {
        let painter = ui.painter();
        
        let transforms = self.entity_transforms.lock().unwrap();
        if let Some(transform) = transforms.get(&entity_id) {
            // Kita perlu mengkonversi koordinat 3D ke koordinat layar 2D
            // Posisi di scene view adalah:
            // X screen = center_x + (position.x + camera_position.x) * scale_factor * camera_zoom
            // Y screen = center_y - (position.y + camera_position.y) * scale_factor * camera_zoom
            let scale_factor = 50.0; // Faktor konversi dari unit game ke pixel
            let center_x = rect.center().x;
            let center_y = rect.center().y;
            
            // Perspektif sederhana: benda yang lebih jauh (z lebih negatif) akan tampak lebih kecil
            let z_offset = transform.position[2];
            let perspective_scale = 1.0 + z_offset * 0.02;  // Buat efek perspektif lebih terlihat
            let perspective_zoom = if perspective_scale <= 0.1 { 0.1 } else { perspective_scale };
            
            let pos_x = center_x + (transform.position[0] + self.camera_position[0]) * scale_factor * self.camera_zoom;
            let pos_y = center_y - (transform.position[1] + self.camera_position[1]) * scale_factor * self.camera_zoom;
            
            // Gunakan nilai skala dari transform untuk menentukan ukuran entitas
            let base_size = 30.0 * self.camera_zoom * perspective_zoom;
            let size_x = base_size * transform.scale[0];
            let size_y = base_size * transform.scale[1];
            
            let entity_rect = Rect::from_center_size(
                Pos2::new(pos_x, pos_y),
                Vec2::new(size_x, size_y),
            );
            
            // Debug untuk Background (entity_id 4)
            if entity_id == 4 {
                let debug_info = format!("Background Transform: pos={:?}, scale={:?}, screen_pos=({:.1},{:.1}), size=({:.1},{:.1})", 
                                        transform.position, transform.scale, pos_x, pos_y, size_x, size_y);
                println!("{}", debug_info);
            }
            
            // Warna dasar entitas
            let color = match entity_id {
                1 => Color32::from_rgb(0, 150, 255), // Camera
                2 => Color32::from_rgb(255, 200, 0), // Light
                3 => Color32::from_rgb(0, 200, 0),   // Player
                4 => Color32::from_rgb(150, 75, 0),  // Ground/Background
                _ => Color32::WHITE,
            };
            
            // Jika entitas dipilih, gambar highlight di sekitarnya
            let is_selected = Some(entity_id) == self.selected_entity;
            
            // Hitung rotasi dari nilai transform
            let rotation_radians = transform.rotation[2].to_radians();
            
            // Draw different shapes based on entity type
            match entity_id {
                1 => { // Camera
                    // Camera icon - skala berdasarkan Z position
                    let camera_width = 20.0 * self.camera_zoom * perspective_zoom;
                    let camera_height = 12.0 * self.camera_zoom * perspective_zoom;
                    
                    painter.rect_filled(
                        Rect::from_center_size(
                            Pos2::new(pos_x, pos_y),
                            Vec2::new(camera_width, camera_height),
                        ),
                        3.0,
                        color,
                    );
                    
                    painter.circle_filled(
                        Pos2::new(pos_x + 10.0 * self.camera_zoom * perspective_zoom, pos_y),
                        4.0 * self.camera_zoom * perspective_zoom,
                        color,
                    );
                    
                    // Draw frustum lines
                    let line_length = 30.0 * self.camera_zoom * perspective_zoom;
                    let line_width = 1.0;
                    painter.line_segment(
                        [Pos2::new(pos_x, pos_y), 
                         Pos2::new(pos_x - line_length, pos_y + line_length * 0.8)],
                        Stroke::new(line_width, color),
                    );
                    painter.line_segment(
                        [Pos2::new(pos_x, pos_y), 
                         Pos2::new(pos_x - line_length, pos_y - line_length * 0.8)],
                        Stroke::new(line_width, color),
                    );
                    
                    // Highlight jika dipilih
                    if is_selected {
                        let camera_rect = Rect::from_center_size(
                            Pos2::new(pos_x, pos_y),
                            Vec2::new(camera_width + 8.0, camera_height + 8.0),
                        );
                        
                        painter.rect_stroke(
                            camera_rect,
                            3.0,
                            Stroke::new(2.0, Color32::YELLOW),
                        );
                        
                        // Tambahkan pulsing glow effect
                        let time = ui.input(|i| i.time);
                        let glow_alpha = ((time * 2.0).sin() * 0.5 + 0.5) as f32 * 0.7 + 0.3;
                        let glow_color = Color32::from_rgba_premultiplied(
                            255, 255, 0, (glow_alpha * 100.0) as u8
                        );
                        
                        painter.rect_stroke(
                            camera_rect.expand(4.0),
                            3.0,
                            Stroke::new(1.0, glow_color),
                        );
                    }
                },
                2 => { // Directional Light
                    // Light icon
                    let light_radius = 10.0 * self.camera_zoom * perspective_zoom;
                    painter.circle_filled(
                        Pos2::new(pos_x, pos_y),
                        light_radius,
                        color,
                    );
                    
                    // Light rays - rotasi sesuai dengan transform.rotation[1] (Y)
                    let ray_count = 8;
                    let ray_length = 20.0 * self.camera_zoom * perspective_zoom;
                    let y_rotation_radians = transform.rotation[1].to_radians();
                    
                    for i in 0..ray_count {
                        let angle = i as f32 * std::f32::consts::PI * 2.0 / ray_count as f32 + y_rotation_radians;
                        let dx = angle.cos() * ray_length;
                        let dy = angle.sin() * ray_length;
                        
                        painter.line_segment(
                            [Pos2::new(pos_x, pos_y), 
                             Pos2::new(pos_x + dx, pos_y + dy)],
                            Stroke::new(2.0, color),
                        );
                    }
                    
                    // Draw direction arrow for light direction
                    let arrow_length = 30.0 * self.camera_zoom * perspective_zoom;
                    let arrow_dir_x = y_rotation_radians.sin() * arrow_length;
                    let arrow_dir_y = y_rotation_radians.cos() * arrow_length;
                    
                    // Gambar panah utama
                    painter.line_segment(
                        [Pos2::new(pos_x, pos_y), 
                         Pos2::new(pos_x + arrow_dir_x, pos_y - arrow_dir_y)],
                        Stroke::new(3.0, Color32::from_rgb(255, 160, 0)),
                    );
                    
                    // Arrow head
                    let head_size = 8.0 * self.camera_zoom * perspective_zoom;
                    let head_angle = 0.5; // ~30 degrees in radians
                    let head1_x = pos_x + arrow_dir_x - head_size * (y_rotation_radians + std::f32::consts::PI - head_angle).cos();
                    let head1_y = pos_y - arrow_dir_y - head_size * (y_rotation_radians + std::f32::consts::PI - head_angle).sin();
                    let head2_x = pos_x + arrow_dir_x - head_size * (y_rotation_radians + std::f32::consts::PI + head_angle).cos();
                    let head2_y = pos_y - arrow_dir_y - head_size * (y_rotation_radians + std::f32::consts::PI + head_angle).sin();
                    
                    painter.line_segment(
                        [Pos2::new(pos_x + arrow_dir_x, pos_y - arrow_dir_y), 
                         Pos2::new(head1_x, head1_y)],
                        Stroke::new(3.0, Color32::from_rgb(255, 160, 0)),
                    );
                    
                    painter.line_segment(
                        [Pos2::new(pos_x + arrow_dir_x, pos_y - arrow_dir_y), 
                         Pos2::new(head2_x, head2_y)],
                        Stroke::new(3.0, Color32::from_rgb(255, 160, 0)),
                    );
                    
                    // Highlight jika dipilih
                    if is_selected {
                        // Lingkaran glow di sekitar light source
                        painter.circle_stroke(
                            Pos2::new(pos_x, pos_y),
                            light_radius + 8.0,
                            Stroke::new(2.0, Color32::YELLOW),
                        );
                        
                        // Tambahkan pulsing glow effect
                        let time = ui.input(|i| i.time);
                        let glow_alpha = ((time * 2.0).sin() * 0.5 + 0.5) as f32 * 0.7 + 0.3;
                        let glow_color = Color32::from_rgba_premultiplied(
                            255, 255, 0, (glow_alpha * 100.0) as u8
                        );
                        
                        painter.circle_stroke(
                            Pos2::new(pos_x, pos_y),
                            light_radius + 16.0,
                            Stroke::new(1.0, glow_color),
                        );
                    }
                },
                3 => { // Player
                    // Player (cube) - dengan rotasi
                    if rotation_radians.abs() < 0.001 {
                        // Jika tidak ada rotasi, gambar kotak biasa
                        painter.rect_filled(entity_rect, 0.0, color);
                        
                        // Draw outline if selected
                        if is_selected {
                            painter.rect_stroke(
                                entity_rect.expand(4.0), 
                                0.0, 
                                Stroke::new(2.0, Color32::YELLOW)
                            );
                            
                            // Tambahkan pulsing glow effect
                            let time = ui.input(|i| i.time);
                            let glow_alpha = ((time * 2.0).sin() * 0.5 + 0.5) as f32 * 0.7 + 0.3;
                            let glow_color = Color32::from_rgba_premultiplied(
                                255, 255, 0, (glow_alpha * 100.0) as u8
                            );
                            
                            painter.rect_stroke(
                                entity_rect.expand(8.0), 
                                2.0,
                                Stroke::new(1.0, glow_color)
                            );
                        }
                    } else {
                        // Jika ada rotasi, gambar bentuk yang diputar
                        let half_width = size_x / 2.0;
                        let half_height = size_y / 2.0;
                        
                        // Buat 4 titik sudut kotak
                        let points = [
                            rotate_point(-half_width, -half_height, rotation_radians, pos_x, pos_y),
                            rotate_point(half_width, -half_height, rotation_radians, pos_x, pos_y),
                            rotate_point(half_width, half_height, rotation_radians, pos_x, pos_y),
                            rotate_point(-half_width, half_height, rotation_radians, pos_x, pos_y),
                        ];
                        
                        // Gambar poligon
                        painter.add(egui::Shape::convex_polygon(
                            points.to_vec(),
                            color,
                            Stroke::NONE,
                        ));
                        
                        // Outline untuk objek yang dipilih
                        if is_selected {
                            for i in 0..4 {
                                let j = (i + 1) % 4;
                                painter.line_segment(
                                    [points[i], points[j]],
                                    Stroke::new(2.0, Color32::YELLOW),
                                );
                            }
                            
                            // Tambahkan pulsing glow effect
                            let time = ui.input(|i| i.time);
                            let glow_alpha = ((time * 2.0).sin() * 0.5 + 0.5) as f32 * 0.7 + 0.3;
                            let glow_color = Color32::from_rgba_premultiplied(
                                255, 255, 0, (glow_alpha * 100.0) as u8
                            );
                            
                            // Gambar garis glow pada sisi luar
                            for i in 0..4 {
                                let j = (i + 1) % 4;
                                painter.line_segment(
                                    [points[i], points[j]],
                                    Stroke::new(4.0, glow_color),
                                );
                            }
                        }
                    }
                },
                4 => { // Background
                    // Background - panjang dan tipis
                    // Khusus untuk background, kita gambar dengan cara khusus agar terlihat jelas
                    let bg_width = size_x;
                    let bg_height = size_y;
                    
                    let bg_rect = Rect::from_center_size(
                        Pos2::new(pos_x, pos_y),
                        Vec2::new(bg_width, bg_height),
                    );
                    
                    // Gambar background dengan warna coklat
                    painter.rect_filled(bg_rect, 0.0, color);
                    
                    // Draw outline if selected
                    if is_selected {
                        // Gambar outline tebal kuning
                        painter.rect_stroke(
                            bg_rect, 
                            0.0, 
                            Stroke::new(3.0, Color32::YELLOW)
                        );
                        
                        // Tambahkan pulsing glow effect yang lebih jelas
                        let time = ui.input(|i| i.time);
                        let glow_alpha = ((time * 2.0).sin() * 0.5 + 0.5) as f32 * 0.7 + 0.3;
                        let glow_color = Color32::from_rgba_premultiplied(
                            255, 255, 0, (glow_alpha * 100.0) as u8
                        );
                        
                        painter.rect_stroke(
                            bg_rect.expand(8.0), 
                            2.0,
                            Stroke::new(2.0, glow_color)
                        );
                    }
                },
                _ => { // Default
                    // Default shape
                    painter.rect_filled(entity_rect, 0.0, color);
                    
                    // Highlight jika dipilih
                    if is_selected {
                        painter.rect_stroke(
                            entity_rect.expand(4.0),
                            0.0,
                            Stroke::new(2.0, Color32::YELLOW)
                        );
                    }
                }
            }
            
            // Draw entity name label if selected or hovered
            if is_selected {
                if let Some(name) = self.entity_names.get(&entity_id) {
                    // Tambahkan background untuk text agar lebih mudah dibaca
                    let font_id = egui::FontId::proportional(14.0);
                    let text_galley = ui.painter().layout_no_wrap(
                        name.clone(),
                        font_id.clone(),
                        Color32::WHITE,
                    );
                    let label_width = text_galley.rect.width() + 10.0;
                    let label_height = text_galley.rect.height() + 6.0;
                    let label_pos = Pos2::new(pos_x, pos_y - size_y/2.0 - 20.0);
                    
                    painter.rect_filled(
                        Rect::from_center_size(
                            label_pos,
                            Vec2::new(label_width, label_height),
                        ),
                        3.0,
                        Color32::from_rgba_premultiplied(0, 0, 0, 180),
                    );
                    
                    painter.text(
                        label_pos,
                        egui::Align2::CENTER_CENTER,
                        name,
                        font_id.clone(),
                        Color32::WHITE,
                    );
                    
                    // Tambahkan posisi z pada label dengan font id baru
                    let z_font_id = egui::FontId::proportional(14.0);
                    let z_text = format!("z: {:.3}", transform.position[2]);
                    let z_text_galley = ui.painter().layout_no_wrap(
                        z_text.clone(),
                        z_font_id.clone(),
                        Color32::LIGHT_GRAY,
                    );
                    let z_label_width = z_text_galley.rect.width() + 10.0;
                    let z_label_pos = Pos2::new(pos_x, pos_y - size_y/2.0 - 40.0);
                    
                    painter.rect_filled(
                        Rect::from_center_size(
                            z_label_pos,
                            Vec2::new(z_label_width, label_height),
                        ),
                        3.0,
                        Color32::from_rgba_premultiplied(0, 0, 0, 180),
                    );
                    
                    painter.text(
                        z_label_pos,
                        egui::Align2::CENTER_CENTER,
                        z_text,
                        z_font_id,
                        Color32::LIGHT_GRAY,
                    );
                }
            }
        }
    }
    
    /// Check if a point is inside an entity
    pub fn is_point_in_entity(&self, point: Pos2, rect: Rect, entity_id: u32) -> bool {
        if let Some(transform) = self.entity_transforms.lock().unwrap().get(&entity_id) {
            let scale_factor = 50.0;
            let center_x = rect.center().x;
            let center_y = rect.center().y;
            
            let pos_x = center_x + (transform.position[0] + self.camera_position[0]) * scale_factor * self.camera_zoom;
            let pos_y = center_y - (transform.position[1] + self.camera_position[1]) * scale_factor * self.camera_zoom;
            
            let base_size = 30.0 * self.camera_zoom;
            let entity_size_x = base_size * transform.scale[0];
            let entity_size_y = base_size * transform.scale[1];
            
            let rotation_radians = transform.rotation[2].to_radians();
            
            // Jika ada rotasi, perlu pengecekan yang lebih kompleks
            if rotation_radians.abs() > 0.001 {
                // Rotasi titik point relatif terhadap pusat entity
                let dx = point.x - pos_x;
                let dy = point.y - pos_y;
                
                // Rotasi balik untuk mengembalikan ke koordinat lokal
                let cos_rot = rotation_radians.cos();
                let sin_rot = rotation_radians.sin();
                
                let rotated_x = dx * cos_rot + dy * sin_rot;
                let rotated_y = -dx * sin_rot + dy * cos_rot;
                
                // Cek apakah titik yang dirotasi ada di dalam kotak
                return rotated_x.abs() <= entity_size_x / 2.0 && rotated_y.abs() <= entity_size_y / 2.0;
            } else {
                // Tanpa rotasi, gunakan Rect biasa
                let entity_rect = Rect::from_center_size(
                    Pos2::new(pos_x, pos_y),
                    Vec2::new(entity_size_x, entity_size_y),
                );
                
                return entity_rect.contains(point);
            }
        }
        
        false
    }
    
}

/// Rotate a point around the origin
fn rotate_point(x: f32, y: f32, angle: f32, center_x: f32, center_y: f32) -> Pos2 {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    
    let rotated_x = x * cos_a - y * sin_a;
    let rotated_y = x * sin_a + y * cos_a;
    
    Pos2::new(center_x + rotated_x, center_y + rotated_y)
} 