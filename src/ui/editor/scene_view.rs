use egui::{Context, Ui, Color32, Vec2, Rect, Pos2, Stroke};
use crate::ui::editor::ui_components::{SceneViewTool, EntityTransform};
use std::collections::HashMap;

/// Scene view panel for editing scenes
pub struct SceneViewPanel {
    /// The scene view size
    pub scene_view_size: [f32; 2],
    /// The current scene view tool
    pub scene_view_tool: SceneViewTool,
    /// Entity transforms
    pub entity_transforms: HashMap<u32, EntityTransform>,
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
        let mut entity_transforms = HashMap::new();
        entity_transforms.insert(1, EntityTransform {
            position: [0.0, 0.0, -10.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        });
        entity_transforms.insert(2, EntityTransform {
            position: [5.0, 10.0, 0.0],
            rotation: [45.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        });
        entity_transforms.insert(3, EntityTransform {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        });
        entity_transforms.insert(4, EntityTransform {
            position: [0.0, -2.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [20.0, 1.0, 1.0],
        });
        entity_transforms.insert(5, EntityTransform {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        });
        
        Self {
            scene_view_size: [0.0, 0.0],
            scene_view_tool: SceneViewTool::Select,
            entity_transforms,
            selected_entity: None,
            entity_names: HashMap::new(),
            show_grid: true,
            camera_position: [0.0, 0.0],
            camera_zoom: 1.0,
            last_mouse_pos: None,
            active_axis: None,
        }
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
        
        // Draw the scene contents
        self.draw_mock_scene(ui, rect);
        
        // Handle click selection
        if response.clicked() {
            let click_pos = response.interact_pointer_pos.unwrap();
            let mut entity_clicked = false;
            
            // Check if clicked on an entity
            for (&id, _) in &self.entity_transforms {
                if self.is_point_in_entity(click_pos, rect, id) {
                    self.selected_entity = Some(id);
                    entity_clicked = true;
                    if let Some(name) = self.entity_names.get(&id) {
                        log_info(&format!("Selected entity: {}", name));
                    }
                    break;
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
                    if let Some(transform) = self.entity_transforms.get_mut(&entity_id) {
                        let delta = response.drag_delta();
                        
                        // Apply transformation based on active axis
                        match self.active_axis {
                            Some(GizmoAxis::X) => {
                                transform.position[0] += delta.x * 0.01 / self.camera_zoom;
                                log_info(&format!("Moving {} along X axis", entity_id));
                            }
                            Some(GizmoAxis::Y) => {
                                transform.position[1] -= delta.y * 0.01 / self.camera_zoom;
                                log_info(&format!("Moving {} along Y axis", entity_id));
                            }
                            Some(GizmoAxis::Z) => {
                                // Z axis movement would depend on the view projection
                                transform.position[2] += (delta.x - delta.y) * 0.005 / self.camera_zoom;
                                log_info(&format!("Moving {} along Z axis", entity_id));
                            }
                            None => {
                                // Move in all axes
                                transform.position[0] += delta.x * 0.01 / self.camera_zoom;
                                transform.position[1] -= delta.y * 0.01 / self.camera_zoom;
                            }
                        }
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
        
        if let Some(transform) = self.entity_transforms.get(&entity_id) {
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
        
        if let Some(transform) = self.entity_transforms.get(&entity_id) {
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
        
        if let Some(transform) = self.entity_transforms.get(&entity_id) {
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
            if let Some(transform) = self.entity_transforms.get(&entity_id) {
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
        // Draw the player (entity 3)
        self.draw_entity(ui, rect, 3);
        
        // Draw the ground (entity 4)
        self.draw_entity(ui, rect, 4);
        
        // Draw the camera (entity 1)
        self.draw_entity(ui, rect, 1);
        
        // Draw the light (entity 2)
        self.draw_entity(ui, rect, 2);
    }
    
    /// Draw an entity with proper transformation
    fn draw_entity(&self, ui: &mut Ui, rect: Rect, entity_id: u32) {
        let painter = ui.painter();
        
        if let Some(transform) = self.entity_transforms.get(&entity_id) {
            let center_x = rect.center().x + (transform.position[0] + self.camera_position[0]) * 50.0 * self.camera_zoom;
            let center_y = rect.center().y - (transform.position[1] + self.camera_position[1]) * 50.0 * self.camera_zoom;
            let size_x = 30.0 * transform.scale[0] * self.camera_zoom;
            let size_y = 30.0 * transform.scale[1] * self.camera_zoom;
            
            let entity_rect = Rect::from_center_size(
                Pos2::new(center_x, center_y),
                Vec2::new(size_x, size_y),
            );
            
            let color = if Some(entity_id) == self.selected_entity {
                Color32::YELLOW
            } else {
                match entity_id {
                    1 => Color32::from_rgb(0, 150, 255), // Camera
                    2 => Color32::from_rgb(255, 200, 0), // Light
                    3 => Color32::from_rgb(0, 200, 0),   // Player
                    4 => Color32::from_rgb(150, 75, 0),  // Ground
                    _ => Color32::WHITE,
                }
            };
            
            // Draw different shapes based on entity type
            match entity_id {
                1 => {
                    // Camera icon
                    painter.rect_filled(
                        Rect::from_center_size(
                            Pos2::new(center_x, center_y),
                            Vec2::new(20.0 * self.camera_zoom, 12.0 * self.camera_zoom),
                        ),
                        3.0,
                        color,
                    );
                    
                    painter.circle_filled(
                        Pos2::new(center_x + 10.0 * self.camera_zoom, center_y),
                        4.0 * self.camera_zoom,
                        color,
                    );
                    
                    // Draw frustum lines
                    let line_length = 30.0 * self.camera_zoom;
                    let line_width = 1.0;
                    painter.line_segment(
                        [Pos2::new(center_x, center_y), 
                         Pos2::new(center_x - line_length, center_y + line_length * 0.8)],
                        Stroke::new(line_width, color),
                    );
                    painter.line_segment(
                        [Pos2::new(center_x, center_y), 
                         Pos2::new(center_x - line_length, center_y - line_length * 0.8)],
                        Stroke::new(line_width, color),
                    );
                },
                2 => {
                    // Light icon
                    painter.circle_filled(
                        Pos2::new(center_x, center_y),
                        8.0 * self.camera_zoom,
                        color,
                    );
                    
                    // Light rays
                    for i in 0..8 {
                        let angle = i as f32 * std::f32::consts::PI / 4.0;
                        let ray_length = 15.0 * self.camera_zoom;
                        let dx = angle.cos() * ray_length;
                        let dy = angle.sin() * ray_length;
                        
                        painter.line_segment(
                            [Pos2::new(center_x, center_y), 
                             Pos2::new(center_x + dx, center_y + dy)],
                            Stroke::new(1.0, color),
                        );
                    }
                },
                3 => {
                    // Player (cube)
                    painter.rect_filled(entity_rect, 0.0, color);
                    
                    // Draw outline if selected
                    if Some(entity_id) == self.selected_entity {
                        painter.rect_stroke(entity_rect, 0.0, Stroke::new(2.0, Color32::WHITE));
                    }
                },
                4 => {
                    // Ground (wide rectangle)
                    painter.rect_filled(entity_rect, 0.0, color);
                    
                    // Draw outline if selected
                    if Some(entity_id) == self.selected_entity {
                        painter.rect_stroke(entity_rect, 0.0, Stroke::new(2.0, Color32::WHITE));
                    }
                },
                _ => {
                    // Default shape
                    painter.rect_filled(entity_rect, 0.0, color);
                }
            }
            
            // Draw entity name label if selected or hovered
            if Some(entity_id) == self.selected_entity {
                if let Some(name) = self.entity_names.get(&entity_id) {
                    painter.text(
                        Pos2::new(center_x, center_y - size_y/2.0 - 15.0),
                        egui::Align2::CENTER_BOTTOM,
                        name,
                        egui::FontId::default(),
                        Color32::WHITE,
                    );
                }
            }
        }
    }
    
    /// Check if a point is inside an entity
    pub fn is_point_in_entity(&self, point: Pos2, rect: Rect, entity_id: u32) -> bool {
        if let Some(transform) = self.entity_transforms.get(&entity_id) {
            let center_x = rect.center().x + (transform.position[0] + self.camera_position[0]) * 50.0 * self.camera_zoom;
            let center_y = rect.center().y - (transform.position[1] + self.camera_position[1]) * 50.0 * self.camera_zoom;
            
            let entity_size_x = 30.0 * transform.scale[0] * self.camera_zoom;
            let entity_size_y = 30.0 * transform.scale[1] * self.camera_zoom;
            
            let entity_rect = Rect::from_center_size(
                Pos2::new(center_x, center_y),
                Vec2::new(entity_size_x, entity_size_y),
            );
            
            return entity_rect.contains(point);
        }
        
        false
    }
} 