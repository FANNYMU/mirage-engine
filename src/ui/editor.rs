use egui::{Context, RichText, Ui, Window, SidePanel, TopBottomPanel, CentralPanel, ScrollArea, Sense, Color32, Vec2};
use std::collections::HashMap;

/// The main editor UI for the engine
pub struct EditorUI {
    /// The currently selected entity
    selected_entity: Option<u32>,
    /// The hierarchy expanded state
    hierarchy_expanded: HashMap<u32, bool>,
    /// The scene view size
    scene_view_size: [f32; 2],
    /// The game view size
    game_view_size: [f32; 2],
    /// Whether the editor is in play mode
    play_mode: bool,
    /// The current scene view tool
    scene_view_tool: SceneViewTool,
    /// Entity names
    entity_names: HashMap<u32, String>,
}

impl EditorUI {
    /// Create a new editor UI
    pub fn new() -> Self {
        let mut entity_names = HashMap::new();
        entity_names.insert(1, "Main Camera".to_string());
        entity_names.insert(2, "Directional Light".to_string());
        entity_names.insert(3, "Player".to_string());
        entity_names.insert(4, "Background".to_string());
        entity_names.insert(5, "UI Canvas".to_string());
        
        Self {
            selected_entity: None,
            hierarchy_expanded: HashMap::new(),
            scene_view_size: [800.0, 600.0],
            game_view_size: [800.0, 600.0],
            play_mode: false,
            scene_view_tool: SceneViewTool::Select,
            entity_names,
        }
    }
    
    /// Update the editor UI
    pub fn update(&mut self, ctx: &Context, _delta_time: f32) {
        // Render the UI
        self.render_menu_bar(ctx);
        self.render_hierarchy_panel(ctx);
        self.render_inspector_panel(ctx);
        self.render_scene_view(ctx);
        self.render_game_view(ctx);
        self.render_project_panel(ctx);
        self.render_console_panel(ctx);
    }
    
    /// Render the menu bar
    fn render_menu_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Scene").clicked() {
                        self.new_scene();
                        ui.close_menu();
                    }
                    if ui.button("Open Scene...").clicked() {
                        // TODO: Open scene dialog
                        ui.close_menu();
                    }
                    if ui.button("Save Scene").clicked() {
                        // TODO: Save scene
                        ui.close_menu();
                    }
                    if ui.button("Save Scene As...").clicked() {
                        // TODO: Save scene as dialog
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        // TODO: Exit application
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        // TODO: Undo
                        ui.close_menu();
                    }
                    if ui.button("Redo").clicked() {
                        // TODO: Redo
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Cut").clicked() {
                        // TODO: Cut
                        ui.close_menu();
                    }
                    if ui.button("Copy").clicked() {
                        // TODO: Copy
                        ui.close_menu();
                    }
                    if ui.button("Paste").clicked() {
                        // TODO: Paste
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("GameObject", |ui| {
                    if ui.button("Create Empty").clicked() {
                        self.create_empty_entity();
                        ui.close_menu();
                    }
                    ui.menu_button("2D Object", |ui| {
                        if ui.button("Sprite").clicked() {
                            self.create_sprite_entity();
                            ui.close_menu();
                        }
                        if ui.button("Text").clicked() {
                            // TODO: Create text entity
                            ui.close_menu();
                        }
                    });
                });
                
                // Play mode controls
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.play_mode {
                        if ui.button("■ Stop").clicked() {
                            self.play_mode = false;
                            // TODO: Reset scene
                        }
                    } else {
                        if ui.button("▶ Play").clicked() {
                            self.play_mode = true;
                        }
                    }
                });
            });
        });
    }
    
    /// Render the hierarchy panel
    fn render_hierarchy_panel(&mut self, ctx: &Context) {
        SidePanel::left("hierarchy_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Hierarchy");
                ui.separator();
                
                // Add entity button
                if ui.button("+ Add Entity").clicked() {
                    self.create_empty_entity();
                }
                
                ui.separator();
                
                // Entity list
                ScrollArea::vertical().show(ui, |ui| {
                    // Display entities
                    for &id in &[1, 2, 3, 4, 5] {
                        let is_selected = self.selected_entity.map_or(false, |e| e == id);
                        
                        // Get entity name, use a default if not found
                        let name = match self.entity_names.get(&id) {
                            Some(name) => name,
                            None => {
                                let default_name = format!("Entity {}", id);
                                self.entity_names.insert(id, default_name.clone());
                                self.entity_names.get(&id).unwrap()
                            }
                        };
                        
                        let response = ui.selectable_label(is_selected, name);
                        
                        if response.clicked() {
                            self.selected_entity = Some(id);
                        }
                        
                        if response.double_clicked() {
                            // TODO: Focus on entity in scene view
                        }
                        
                        response.context_menu(|ui| {
                            if ui.button("Rename").clicked() {
                                // TODO: Rename entity
                                ui.close_menu();
                            }
                            if ui.button("Duplicate").clicked() {
                                // TODO: Duplicate entity
                                ui.close_menu();
                            }
                            ui.separator();
                            if ui.button("Delete").clicked() {
                                self.selected_entity = None;
                                ui.close_menu();
                            }
                        });
                    }
                });
            });
    }
    
    /// Render the inspector panel
    fn render_inspector_panel(&mut self, ctx: &Context) {
        SidePanel::right("inspector_panel")
            .resizable(true)
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Inspector");
                ui.separator();
                
                if let Some(entity_id) = self.selected_entity {
                    // Name component
                    ui.collapsing("Name", |ui| {
                        // Get entity name, use a default if not found
                        let mut name_value = match self.entity_names.get(&entity_id) {
                            Some(name) => name.clone(),
                            None => {
                                let default_name = format!("Entity {}", entity_id);
                                self.entity_names.insert(entity_id, default_name.clone());
                                default_name
                            }
                        };
                        
                        if ui.text_edit_singleline(&mut name_value).changed() {
                            // Update name
                            self.entity_names.insert(entity_id, name_value);
                        }
                    });
                    
                    // Transform component
                    ui.collapsing("Transform", |ui| {
                        let mut position = [0.0, 0.0, 0.0];
                        let mut rotation = [0.0, 0.0, 0.0];
                        let mut scale = [1.0, 1.0, 1.0];
                        
                        ui.horizontal(|ui| {
                            ui.label("Position:");
                            ui.add(egui::DragValue::new(&mut position[0]).speed(0.1).prefix("X: "));
                            ui.add(egui::DragValue::new(&mut position[1]).speed(0.1).prefix("Y: "));
                            ui.add(egui::DragValue::new(&mut position[2]).speed(0.1).prefix("Z: "));
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Rotation:");
                            ui.add(egui::DragValue::new(&mut rotation[0]).speed(0.1).prefix("X: "));
                            ui.add(egui::DragValue::new(&mut rotation[1]).speed(0.1).prefix("Y: "));
                            ui.add(egui::DragValue::new(&mut rotation[2]).speed(0.1).prefix("Z: "));
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Scale:");
                            ui.add(egui::DragValue::new(&mut scale[0]).speed(0.1).prefix("X: "));
                            ui.add(egui::DragValue::new(&mut scale[1]).speed(0.1).prefix("Y: "));
                            ui.add(egui::DragValue::new(&mut scale[2]).speed(0.1).prefix("Z: "));
                        });
                    });
                    
                    // Entity-specific components
                    match entity_id {
                        1 => { // Camera
                            ui.collapsing("Camera", |ui| {
                                let mut projection_type = 0;
                                ui.horizontal(|ui| {
                                    ui.label("Projection:");
                                    ui.radio_value(&mut projection_type, 0, "Perspective");
                                    ui.radio_value(&mut projection_type, 1, "Orthographic");
                                });
                                
                                let mut fov = 60.0;
                                ui.add(egui::Slider::new(&mut fov, 1.0..=179.0).text("Field of View"));
                                
                                let mut near_clip = 0.1;
                                let mut far_clip = 1000.0;
                                ui.add(egui::Slider::new(&mut near_clip, 0.01..=10.0).text("Near Clip"));
                                ui.add(egui::Slider::new(&mut far_clip, 10.0..=10000.0).text("Far Clip"));
                            });
                        },
                        2 => { // Light
                            ui.collapsing("Light", |ui| {
                                let mut light_type = 0;
                                ui.horizontal(|ui| {
                                    ui.label("Type:");
                                    ui.radio_value(&mut light_type, 0, "Directional");
                                    ui.radio_value(&mut light_type, 1, "Point");
                                    ui.radio_value(&mut light_type, 2, "Spot");
                                });
                                
                                let mut color = [1.0, 1.0, 1.0];
                                ui.horizontal(|ui| {
                                    ui.label("Color:");
                                    ui.color_edit_button_rgb(&mut color);
                                });
                                
                                let mut intensity = 1.0;
                                ui.add(egui::Slider::new(&mut intensity, 0.0..=10.0).text("Intensity"));
                            });
                        },
                        3 => { // Player
                            ui.collapsing("Sprite Renderer", |ui| {
                                ui.label("Sprite: [Player.png]");
                                
                                let mut color = [1.0, 1.0, 1.0, 1.0];
                                ui.horizontal(|ui| {
                                    ui.label("Color:");
                                    ui.color_edit_button_rgba_unmultiplied(&mut color);
                                });
                                
                                let mut flip_x = false;
                                let mut flip_y = false;
                                ui.checkbox(&mut flip_x, "Flip X");
                                ui.checkbox(&mut flip_y, "Flip Y");
                            });
                            
                            ui.collapsing("Rigidbody 2D", |ui| {
                                let mut body_type = 0;
                                ui.horizontal(|ui| {
                                    ui.label("Body Type:");
                                    ui.radio_value(&mut body_type, 0, "Dynamic");
                                    ui.radio_value(&mut body_type, 1, "Kinematic");
                                    ui.radio_value(&mut body_type, 2, "Static");
                                });
                                
                                let mut mass = 1.0;
                                ui.add(egui::Slider::new(&mut mass, 0.1..=100.0).text("Mass"));
                                
                                let mut gravity_scale = 1.0;
                                ui.add(egui::Slider::new(&mut gravity_scale, 0.0..=10.0).text("Gravity Scale"));
                            });
                        },
                        _ => {
                            // Generic components
                        }
                    }
                    
                    // Add component button
                    if ui.button("Add Component").clicked() {
                        // TODO: Show component selection popup
                    }
                } else {
                    ui.label("No entity selected");
                }
            });
    }
    
    /// Render the scene view
    fn render_scene_view(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Scene View");
            
            // Scene view toolbar
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.scene_view_tool, SceneViewTool::Select, "Select");
                ui.selectable_value(&mut self.scene_view_tool, SceneViewTool::Move, "Move");
                ui.selectable_value(&mut self.scene_view_tool, SceneViewTool::Rotate, "Rotate");
                ui.selectable_value(&mut self.scene_view_tool, SceneViewTool::Scale, "Scale");
            });
            
            // Scene view
            let available_size = ui.available_size();
            self.scene_view_size = [available_size.x, available_size.y - 40.0]; // Subtract toolbar height
            
            let (rect, response) = ui.allocate_exact_size(
                egui::vec2(self.scene_view_size[0], self.scene_view_size[1]),
                Sense::click_and_drag(),
            );
            
            // Handle scene view interactions
            if response.clicked() {
                // TODO: Handle click in scene view (entity selection)
            }
            
            if response.dragged() {
                // TODO: Handle drag in scene view (camera pan or entity manipulation)
            }
            
            // Draw scene view background
            ui.painter().rect_filled(
                rect,
                0.0,
                Color32::from_rgb(40, 40, 40),
            );
            
            // Draw grid
            self.draw_grid(ui, rect);
            
            // Draw mock scene elements
            self.draw_mock_scene(ui, rect);
        });
    }
    
    /// Draw a grid in the scene view
    fn draw_grid(&self, ui: &mut Ui, rect: egui::Rect) {
        let grid_size = 20.0;
        let grid_color = Color32::from_rgb(60, 60, 60);
        
        // Draw horizontal lines
        for y in 0..((rect.height() / grid_size) as i32 + 1) {
            let y_pos = rect.min.y + y as f32 * grid_size;
            ui.painter().line_segment(
                [egui::pos2(rect.min.x, y_pos), egui::pos2(rect.max.x, y_pos)],
                egui::Stroke::new(1.0, grid_color),
            );
        }
        
        // Draw vertical lines
        for x in 0..((rect.width() / grid_size) as i32 + 1) {
            let x_pos = rect.min.x + x as f32 * grid_size;
            ui.painter().line_segment(
                [egui::pos2(x_pos, rect.min.y), egui::pos2(x_pos, rect.max.y)],
                egui::Stroke::new(1.0, grid_color),
            );
        }
        
        // Draw axes
        let center_x = rect.min.x + rect.width() / 2.0;
        let center_y = rect.min.y + rect.height() / 2.0;
        
        // X axis (red)
        ui.painter().line_segment(
            [egui::pos2(center_x - 1000.0, center_y), egui::pos2(center_x + 1000.0, center_y)],
            egui::Stroke::new(2.0, Color32::from_rgb(200, 50, 50)),
        );
        
        // Y axis (green)
        ui.painter().line_segment(
            [egui::pos2(center_x, center_y - 1000.0), egui::pos2(center_x, center_y + 1000.0)],
            egui::Stroke::new(2.0, Color32::from_rgb(50, 200, 50)),
        );
    }
    
    /// Draw mock scene elements
    fn draw_mock_scene(&self, ui: &mut Ui, rect: egui::Rect) {
        let center_x = rect.min.x + rect.width() / 2.0;
        let center_y = rect.min.y + rect.height() / 2.0;
        
        // Draw a mock player (blue square)
        let player_size = 40.0;
        let player_rect = egui::Rect::from_center_size(
            egui::pos2(center_x, center_y),
            egui::vec2(player_size, player_size),
        );
        
        let player_color = if self.selected_entity == Some(3) {
            Color32::from_rgb(100, 150, 255)
        } else {
            Color32::from_rgb(50, 100, 200)
        };
        
        ui.painter().rect_filled(
            player_rect,
            5.0,
            player_color,
        );
        
        // Draw a mock background (gray rectangle)
        let bg_rect = egui::Rect::from_center_size(
            egui::pos2(center_x, center_y + 100.0),
            egui::vec2(300.0, 50.0),
        );
        
        let bg_color = if self.selected_entity == Some(4) {
            Color32::from_rgb(150, 150, 150)
        } else {
            Color32::from_rgb(100, 100, 100)
        };
        
        ui.painter().rect_filled(
            bg_rect,
            3.0,
            bg_color,
        );
        
        // Draw a mock camera icon
        if self.selected_entity != Some(1) {
            self.draw_camera_icon(ui, center_x - 150.0, center_y - 100.0, 30.0, Color32::from_rgb(200, 200, 50));
        } else {
            self.draw_camera_icon(ui, center_x - 150.0, center_y - 100.0, 30.0, Color32::from_rgb(255, 255, 100));
        }
        
        // Draw a mock light icon
        if self.selected_entity != Some(2) {
            self.draw_light_icon(ui, center_x + 150.0, center_y - 100.0, 30.0, Color32::from_rgb(255, 200, 50));
        } else {
            self.draw_light_icon(ui, center_x + 150.0, center_y - 100.0, 30.0, Color32::from_rgb(255, 255, 100));
        }
    }
    
    /// Draw a camera icon
    fn draw_camera_icon(&self, ui: &mut Ui, x: f32, y: f32, size: f32, color: Color32) {
        let points = [
            egui::pos2(x - size/2.0, y - size/3.0),
            egui::pos2(x + size/2.0, y - size/3.0),
            egui::pos2(x + size/2.0, y + size/3.0),
            egui::pos2(x - size/2.0, y + size/3.0),
        ];
        
        ui.painter().add(egui::Shape::convex_polygon(
            points.to_vec(),
            color,
            egui::Stroke::new(1.0, Color32::BLACK),
        ));
        
        // Draw lens
        ui.painter().circle_filled(
            egui::pos2(x + size/4.0, y),
            size/5.0,
            Color32::from_rgb(50, 50, 50),
        );
    }
    
    /// Draw a light icon
    fn draw_light_icon(&self, ui: &mut Ui, x: f32, y: f32, size: f32, color: Color32) {
        // Draw sun-like icon
        ui.painter().circle_filled(
            egui::pos2(x, y),
            size/3.0,
            color,
        );
        
        // Draw rays
        for i in 0..8 {
            let angle = i as f32 * std::f32::consts::PI / 4.0;
            let inner_x = x + (size/3.0) * angle.cos();
            let inner_y = y + (size/3.0) * angle.sin();
            let outer_x = x + (size/2.0) * angle.cos();
            let outer_y = y + (size/2.0) * angle.sin();
            
            ui.painter().line_segment(
                [egui::pos2(inner_x, inner_y), egui::pos2(outer_x, outer_y)],
                egui::Stroke::new(2.0, color),
            );
        }
    }
    
    /// Render the game view
    fn render_game_view(&mut self, ctx: &Context) {
        Window::new("Game")
            .resizable(true)
            .default_size(Vec2::new(800.0, 600.0))
            .show(ctx, |ui| {
                // Game view
                let available_size = ui.available_size();
                self.game_view_size = [available_size.x, available_size.y];
                
                let (rect, _response) = ui.allocate_exact_size(
                    egui::vec2(self.game_view_size[0], self.game_view_size[1]),
                    Sense::hover(),
                );
                
                // Draw game view background
                ui.painter().rect_filled(
                    rect,
                    0.0,
                    Color32::from_rgb(20, 20, 20),
                );
                
                // Draw mock game view (similar to scene view but without grid and gizmos)
                let center_x = rect.min.x + rect.width() / 2.0;
                let center_y = rect.min.y + rect.height() / 2.0;
                
                // Draw a mock player (blue square)
                let player_size = 40.0;
                let player_rect = egui::Rect::from_center_size(
                    egui::pos2(center_x, center_y),
                    egui::vec2(player_size, player_size),
                );
                
                ui.painter().rect_filled(
                    player_rect,
                    5.0,
                    Color32::from_rgb(50, 100, 200),
                );
                
                // Draw a mock background (gray rectangle)
                let bg_rect = egui::Rect::from_center_size(
                    egui::pos2(center_x, center_y + 100.0),
                    egui::vec2(300.0, 50.0),
                );
                
                ui.painter().rect_filled(
                    bg_rect,
                    3.0,
                    Color32::from_rgb(100, 100, 100),
                );
            });
    }
    
    /// Render the project panel
    fn render_project_panel(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("project_panel")
            .resizable(true)
            .default_height(200.0)
            .show(ctx, |ui| {
                ui.heading("Project");
                ui.separator();
                
                // Project browser
                ScrollArea::horizontal().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Mock folders
                        for i in 0..5 {
                            ui.vertical(|ui| {
                                let folder_name = match i {
                                    0 => "Scripts",
                                    1 => "Scenes",
                                    2 => "Prefabs",
                                    3 => "Sprites",
                                    4 => "Audio",
                                    _ => "Misc",
                                };
                                
                                ui.add(egui::Label::new(RichText::new("📁").size(32.0)).sense(Sense::click()));
                                ui.label(folder_name);
                            });
                            ui.add_space(10.0);
                        }
                        
                        // Mock files
                        for i in 0..3 {
                            ui.vertical(|ui| {
                                let file_name = match i {
                                    0 => "Player.png",
                                    1 => "MainScene.scene",
                                    2 => "Background.png",
                                    _ => "File.txt",
                                };
                                
                                let icon = match i {
                                    0 => "🖼️",
                                    1 => "🎮",
                                    2 => "🖼️",
                                    _ => "📄",
                                };
                                
                                ui.add(egui::Label::new(RichText::new(icon).size(32.0)).sense(Sense::click()));
                                ui.label(file_name);
                            });
                            ui.add_space(10.0);
                        }
                    });
                });
            });
    }
    
    /// Render the console panel
    fn render_console_panel(&mut self, ctx: &Context) {
        Window::new("Console")
            .resizable(true)
            .default_size(Vec2::new(800.0, 200.0))
            .show(ctx, |ui| {
                // Console toolbar
                ui.horizontal(|ui| {
                    ui.label("Filter:");
                    let mut filter_text = String::new();
                    ui.text_edit_singleline(&mut filter_text);
                    
                    ui.checkbox(&mut true, "Info");
                    ui.checkbox(&mut true, "Warning");
                    ui.checkbox(&mut true, "Error");
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Clear").clicked() {
                            // TODO: Clear console
                        }
                    });
                });
                
                ui.separator();
                
                // Console log
                ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                    // Mock log messages
                    ui.label(RichText::new("12:34:56 [INFO] Game initialized").color(Color32::WHITE));
                    ui.label(RichText::new("12:34:57 [INFO] Scene loaded").color(Color32::WHITE));
                    ui.label(RichText::new("12:34:58 [WARNING] Missing sprite reference").color(Color32::YELLOW));
                    ui.label(RichText::new("12:34:59 [INFO] Player spawned").color(Color32::WHITE));
                    ui.label(RichText::new("12:35:00 [ERROR] Failed to load audio file").color(Color32::RED));
                });
            });
    }
    
    /// Create a new scene
    fn new_scene(&mut self) {
        self.selected_entity = None;
        self.hierarchy_expanded.clear();
        // TODO: Actual scene creation
    }
    
    /// Create an empty entity
    fn create_empty_entity(&mut self) {
        // TODO: Actual entity creation
        let entity_id = self.entity_names.len() as u32 + 1;
        self.entity_names.insert(entity_id, format!("New Entity {}", entity_id));
        self.selected_entity = Some(entity_id);
    }
    
    /// Create a sprite entity
    fn create_sprite_entity(&mut self) {
        // TODO: Actual sprite entity creation
        let entity_id = self.entity_names.len() as u32 + 1;
        self.entity_names.insert(entity_id, format!("New Sprite {}", entity_id));
        self.selected_entity = Some(entity_id);
    }
}

/// The scene view tool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SceneViewTool {
    /// Select entities
    Select,
    /// Move entities
    Move,
    /// Rotate entities
    Rotate,
    /// Scale entities
    Scale,
} 