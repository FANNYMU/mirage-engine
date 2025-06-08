use egui::{Context, Ui, ScrollArea, RichText, Color32, Vec2, Frame, Rect, Stroke};
use std::collections::HashMap;
use crate::ui::editor::ui_components::{EntityComponent, ComponentType, EntityTransform};

/// Inspector panel for editing entity properties
pub struct InspectorPanel {
    /// Component expanded states
    pub component_expanded: HashMap<String, bool>,
    /// Entity transforms
    pub entity_transforms: HashMap<u32, EntityTransform>,
    /// Show add component menu
    pub show_add_component_menu: bool,
    /// Add component search text
    pub add_component_search: String,
}

impl InspectorPanel {
    /// Create a new inspector panel
    pub fn new() -> Self {
        Self {
            component_expanded: HashMap::new(),
            entity_transforms: HashMap::new(),
            show_add_component_menu: false,
            add_component_search: String::new(),
        }
    }
    
    /// Set entity transforms
    pub fn set_entity_transforms(&mut self, entity_transforms: HashMap<u32, EntityTransform>) {
        self.entity_transforms = entity_transforms;
    }
    
    /// Render the inspector panel
    pub fn render(&mut self, ui: &mut Ui, selected_entity: Option<u32>, entity_names: &HashMap<u32, String>, log_info: &mut dyn FnMut(&str)) {
        // Unity-like header
        ui.horizontal(|ui| {
            ui.heading("Inspector");
        });
        
        ui.separator();
        
        if let Some(entity_id) = selected_entity {
            if let Some(entity_name) = entity_names.get(&entity_id) {
                // Entity Header with name and enable checkbox
                Frame::none()
                    .fill(Color32::from_rgb(60, 60, 60))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let mut is_enabled = true;
                            ui.checkbox(&mut is_enabled, "");
                            
                            let mut name = entity_name.clone();
                            ui.text_edit_singleline(&mut name);
                            
                            if ui.button("⋮").clicked() {
                                log_info("Entity options menu clicked");
                            }
                        });
                    });
                
                // Static tag/layer row
                ui.horizontal(|ui| {
                    ui.label("Tag");
                    ui.add_space(5.0);
                    
                    let mut tag = "Untagged".to_string();
                    egui::ComboBox::from_label("")
                        .selected_text(&tag)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut tag, "Untagged".to_string(), "Untagged");
                            ui.selectable_value(&mut tag, "Player".to_string(), "Player");
                            ui.selectable_value(&mut tag, "Enemy".to_string(), "Enemy");
                            ui.selectable_value(&mut tag, "MainCamera".to_string(), "MainCamera");
                        });
                    
                    ui.add_space(20.0);
                    
                    ui.label("Layer");
                    ui.add_space(5.0);
                    
                    let mut layer = "Default".to_string();
                    egui::ComboBox::from_label("")
                        .selected_text(&layer)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut layer, "Default".to_string(), "Default");
                            ui.selectable_value(&mut layer, "UI".to_string(), "UI");
                            ui.selectable_value(&mut layer, "Player".to_string(), "Player");
                            ui.selectable_value(&mut layer, "Environment".to_string(), "Environment");
                        });
                });
                
                ui.add_space(5.0);
                
                ScrollArea::vertical().show(ui, |ui| {
                    // Always show Transform component
                    self.render_transform_component(ui, entity_id, log_info);
                    
                    // Show other components
                    let components = self.get_entity_components(entity_id);
                    for component in components {
                        self.render_component(ui, &component, entity_id, log_info);
                    }
                    
                    // Add Component button
                    ui.add_space(10.0);
                    if ui.button("Add Component").clicked() {
                        self.show_add_component_menu = true;
                    }
                });
                
                // Render add component menu if needed
                if self.show_add_component_menu {
                    self.render_add_component_menu(ui, entity_id, log_info);
                }
            } else {
                ui.label("Selected entity not found");
            }
        } else {
            ui.weak("No entity selected");
        }
    }
    
    /// Get components for an entity
    fn get_entity_components(&self, entity_id: u32) -> Vec<EntityComponent> {
        // In a real engine, this would query the ECS
        // For now, return mock components based on entity ID
        
        match entity_id {
            1 => vec![
                EntityComponent {
                    name: "Camera".to_string(),
                    component_type: ComponentType::Camera,
                    removable: false,
                },
            ],
            2 => vec![
                EntityComponent {
                    name: "Light".to_string(),
                    component_type: ComponentType::Light,
                    removable: false,
                },
            ],
            3 => vec![
                EntityComponent {
                    name: "Sprite Renderer".to_string(),
                    component_type: ComponentType::SpriteRenderer,
                    removable: true,
                },
                EntityComponent {
                    name: "Rigidbody 2D".to_string(),
                    component_type: ComponentType::Rigidbody2D,
                    removable: true,
                },
                EntityComponent {
                    name: "Box Collider 2D".to_string(),
                    component_type: ComponentType::BoxCollider2D,
                    removable: true,
                },
                EntityComponent {
                    name: "Player Script".to_string(),
                    component_type: ComponentType::LuaScript,
                    removable: true,
                },
            ],
            4 => vec![
                EntityComponent {
                    name: "Sprite Renderer".to_string(),
                    component_type: ComponentType::SpriteRenderer,
                    removable: true,
                },
            ],
            5 => vec![],
            _ => vec![],
        }
    }
    
    /// Render a component in the inspector
    fn render_component(&mut self, ui: &mut Ui, component: &EntityComponent, _entity_id: u32, log_info: &mut dyn FnMut(&str)) {
        let component_id = format!("component_{}", component.name);
        let is_expanded = *self.component_expanded.entry(component_id.clone()).or_insert(true);
        
        // Unity-style component header
        let header_color = Color32::from_rgb(65, 65, 65);
        let header_hovered = Color32::from_rgb(75, 75, 75);
        
        Frame::none()
            .fill(header_color)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Component enabled checkbox
                    let mut is_active = true;
                    ui.checkbox(&mut is_active, "");
                    
                    // Component title that can be clicked to expand/collapse
                    let title_resp = ui.add(egui::Label::new(
                        RichText::new(&component.name).strong()).sense(egui::Sense::click()));
                    
                    if title_resp.clicked() {
                        *self.component_expanded.get_mut(&component_id).unwrap() = !is_expanded;
                    }
                    
                    // Component menu button
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("⋮").clicked() {
                            log_info(&format!("Component menu for {} clicked", component.name));
                        }
                    });
                });
            });
        
        // Component body when expanded
        if is_expanded {
            Frame::none()
                .fill(Color32::from_rgb(50, 50, 50))
                .inner_margin(egui::style::Margin::symmetric(10.0, 5.0))
                .show(ui, |ui| {
                    match component.component_type {
                        ComponentType::Camera => self.render_camera_component(ui),
                        ComponentType::Light => self.render_light_component(ui),
                        ComponentType::SpriteRenderer => self.render_sprite_renderer_component(ui),
                        ComponentType::Rigidbody2D => self.render_rigidbody2d_component(ui),
                        ComponentType::BoxCollider2D => self.render_box_collider2d_component(ui),
                        ComponentType::LuaScript => self.render_lua_script_component(ui),
                        ComponentType::AudioSource => self.render_audio_source_component(ui),
                        ComponentType::AudioListener => self.render_audio_listener_component(ui),
                        ComponentType::Transform => {
                            // Handled separately
                        },
                    }
                });
        }
    }
    
    /// Render transform component
    fn render_transform_component(&mut self, ui: &mut Ui, entity_id: u32, log_info: &mut dyn FnMut(&str)) {
        let transform = self.entity_transforms.entry(entity_id).or_insert_with(|| {
            EntityTransform {
                position: [0.0, 0.0, 0.0],
                rotation: [0.0, 0.0, 0.0],
                scale: [1.0, 1.0, 1.0],
            }
        });
        
        // Unity-style component header
        Frame::none()
            .fill(Color32::from_rgb(65, 65, 65))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let component_id = "component_Transform".to_string();
                    let is_expanded = *self.component_expanded.entry(component_id.clone()).or_insert(true);
                    
                    if ui.button(if is_expanded { "▼" } else { "►" }).clicked() {
                        *self.component_expanded.get_mut(&component_id).unwrap() = !is_expanded;
                    }
                    
                    ui.label(RichText::new("Transform").strong());
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("⋮").clicked() {
                            log_info("Transform component menu clicked");
                        }
                    });
                });
            });
        
        let component_id = "component_Transform".to_string();
        let is_expanded = *self.component_expanded.entry(component_id).or_insert(true);
        
        if is_expanded {
            Frame::none()
                .fill(Color32::from_rgb(50, 50, 50))
                .inner_margin(egui::style::Margin::symmetric(10.0, 5.0))
                .show(ui, |ui| {
                    let mut changed = false;
                    
                    // Position row with xyz inputs
                    ui.horizontal(|ui| {
                        ui.label("Position");
                        ui.add_space(5.0);
                        
                        // X input with label
                        ui.horizontal(|ui| {
                            ui.label("X");
                            changed |= ui.add(egui::DragValue::new(&mut transform.position[0])
                                .speed(0.1)
                                .fixed_decimals(3)).changed();
                        });
                        
                        // Y input with label
                        ui.horizontal(|ui| {
                            ui.label("Y");
                            changed |= ui.add(egui::DragValue::new(&mut transform.position[1])
                                .speed(0.1)
                                .fixed_decimals(3)).changed();
                        });
                        
                        // Z input with label
                        ui.horizontal(|ui| {
                            ui.label("Z");
                            changed |= ui.add(egui::DragValue::new(&mut transform.position[2])
                                .speed(0.1)
                                .fixed_decimals(3)).changed();
                        });
                    });
                    
                    // Rotation row with xyz inputs
                    ui.horizontal(|ui| {
                        ui.label("Rotation");
                        ui.add_space(5.0);
                        
                        // X input with label
                        ui.horizontal(|ui| {
                            ui.label("X");
                            changed |= ui.add(egui::DragValue::new(&mut transform.rotation[0])
                                .speed(1.0)
                                .fixed_decimals(1)
                                .suffix("°")).changed();
                        });
                        
                        // Y input with label
                        ui.horizontal(|ui| {
                            ui.label("Y");
                            changed |= ui.add(egui::DragValue::new(&mut transform.rotation[1])
                                .speed(1.0)
                                .fixed_decimals(1)
                                .suffix("°")).changed();
                        });
                        
                        // Z input with label
                        ui.horizontal(|ui| {
                            ui.label("Z");
                            changed |= ui.add(egui::DragValue::new(&mut transform.rotation[2])
                                .speed(1.0)
                                .fixed_decimals(1)
                                .suffix("°")).changed();
                        });
                    });
                    
                    // Scale row with xyz inputs
                    ui.horizontal(|ui| {
                        ui.label("Scale");
                        ui.add_space(13.0);
                        
                        // X input with label
                        ui.horizontal(|ui| {
                            ui.label("X");
                            changed |= ui.add(egui::DragValue::new(&mut transform.scale[0])
                                .speed(0.1)
                                .fixed_decimals(3)).changed();
                        });
                        
                        // Y input with label
                        ui.horizontal(|ui| {
                            ui.label("Y");
                            changed |= ui.add(egui::DragValue::new(&mut transform.scale[1])
                                .speed(0.1)
                                .fixed_decimals(3)).changed();
                        });
                        
                        // Z input with label
                        ui.horizontal(|ui| {
                            ui.label("Z");
                            changed |= ui.add(egui::DragValue::new(&mut transform.scale[2])
                                .speed(0.1)
                                .fixed_decimals(3)).changed();
                        });
                    });
                    
                    if changed {
                        log_info(&format!("Updated transform for entity {}", entity_id));
                    }
                });
        }
    }
    
    /// Render camera component
    fn render_camera_component(&self, ui: &mut Ui) {
        let mut clear_flags = 0;
        let clear_flag_options = ["Skybox", "Solid Color", "Depth Only", "Don't Clear"];
        
        let mut background_color = [0.39, 0.58, 0.93, 1.0];
        let mut projection = 0;
        let proj_options = ["Perspective", "Orthographic"];
        let mut fov = 60.0;
        let mut near_clip = 0.3;
        let mut far_clip = 1000.0;
        let mut orthographic_size = 5.0;
        let mut hdr = true;
        let mut allow_msaa = true;
        
        // Define field layout function
        let field_layout = |ui: &mut Ui, label: &str, content: Box<dyn FnOnce(&mut Ui)>| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add_space(ui.available_width() * 0.3 - label.len() as f32 * 7.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    content(ui);
                });
            });
        };
        
        field_layout(ui, "Clear Flags", Box::new(|ui| {
            egui::ComboBox::from_id_source("clear_flags")
                .selected_text(clear_flag_options[clear_flags])
                .show_ui(ui, |ui| {
                    for (i, &flag) in clear_flag_options.iter().enumerate() {
                        ui.selectable_value(&mut clear_flags, i, flag);
                    }
                });
        }));
        
        field_layout(ui, "Background", Box::new(|ui| {
            ui.color_edit_button_rgba_unmultiplied(&mut background_color);
        }));
        
        // Projection ComboBox
        {
            // Clone into a temporary to avoid mutable borrowing issues
            let mut tmp_projection = projection;
            field_layout(ui, "Projection", Box::new(move |ui| {
                egui::ComboBox::from_id_source("projection")
                    .selected_text(proj_options[tmp_projection])
                    .show_ui(ui, |ui| {
                        for (i, &option) in proj_options.iter().enumerate() {
                            ui.selectable_value(&mut tmp_projection, i, option);
                        }
                    });
            }));
            projection = tmp_projection;
        }
        
        // Render appropriate fields based on projection type
        if projection == 0 {
            field_layout(ui, "Field of View", Box::new(|ui| {
                ui.add(egui::Slider::new(&mut fov, 1.0..=179.0).suffix("°"));
            }));
        } else {
            field_layout(ui, "Size", Box::new(|ui| {
                ui.add(egui::DragValue::new(&mut orthographic_size).speed(0.1));
            }));
        }
        
        field_layout(ui, "Clipping Planes", Box::new(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Near");
                    ui.add(egui::DragValue::new(&mut near_clip).speed(0.01).fixed_decimals(2));
                });
                ui.horizontal(|ui| {
                    ui.label("Far");
                    ui.add(egui::DragValue::new(&mut far_clip).speed(1.0).fixed_decimals(1));
                });
            });
        }));
        
        field_layout(ui, "HDR", Box::new(|ui| {
            ui.checkbox(&mut hdr, "");
        }));
        
        field_layout(ui, "Allow MSAA", Box::new(|ui| {
            ui.checkbox(&mut allow_msaa, "");
        }));
    }
    
    /// Render light component
    fn render_light_component(&self, ui: &mut Ui) {
        let mut light_type = 0;
        let light_types = ["Directional", "Point", "Spot", "Area"];
        
        let mut color: [f32; 3] = [1.0, 1.0, 1.0];
        let mut intensity = 1.0;
        let mut range = 10.0;
        let mut shadow_enabled = true;
        let mut shadow_resolution = 1;
        let shadow_resolutions = ["Low", "Medium", "High", "Very High"];
        
        // Define field layout function
        let field_layout = |ui: &mut Ui, label: &str, content: Box<dyn FnOnce(&mut Ui)>| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add_space(ui.available_width() * 0.3 - label.len() as f32 * 7.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    content(ui);
                });
            });
        };
        
        // Light Type ComboBox
        {
            let mut tmp_light_type = light_type;
            field_layout(ui, "Type", Box::new(move |ui| {
                egui::ComboBox::from_id_source("light_type")
                    .selected_text(light_types[tmp_light_type])
                    .show_ui(ui, |ui| {
                        for (i, &light_name) in light_types.iter().enumerate() {
                            ui.selectable_value(&mut tmp_light_type, i, light_name);
                        }
                    });
            }));
            light_type = tmp_light_type;
        }
        
        field_layout(ui, "Color", Box::new(|ui| {
            ui.color_edit_button_rgb(&mut color);
        }));
        
        field_layout(ui, "Intensity", Box::new(|ui| {
            ui.add(egui::Slider::new(&mut intensity, 0.0..=10.0));
        }));
        
        // Range slider for non-directional lights
        if light_type != 0 {
            field_layout(ui, "Range", Box::new(|ui| {
                ui.add(egui::Slider::new(&mut range, 1.0..=100.0).logarithmic(true));
            }));
        }
        
        // Shadow toggles
        {
            let mut tmp_shadow_enabled = shadow_enabled;
            field_layout(ui, "Shadows", Box::new(move |ui| {
                ui.checkbox(&mut tmp_shadow_enabled, "");
            }));
            shadow_enabled = tmp_shadow_enabled;
        }
        
        // Shadow quality settings if shadows are enabled
        if shadow_enabled {
            field_layout(ui, "Shadow Quality", Box::new(|ui| {
                egui::ComboBox::from_id_source("shadow_resolution")
                    .selected_text(shadow_resolutions[shadow_resolution])
                    .show_ui(ui, |ui| {
                        for (i, &res_name) in shadow_resolutions.iter().enumerate() {
                            ui.selectable_value(&mut shadow_resolution, i, res_name);
                        }
                    });
            }));
        }
    }
    
    /// Render sprite renderer component
    fn render_sprite_renderer_component(&self, ui: &mut Ui) {
        let mut color = [1.0, 1.0, 1.0, 1.0];
        let mut sorting_layer = 0;
        let sorting_layers = ["Default", "Background", "Foreground", "UI"];
        let mut order_in_layer = 0;
        let mut material = "Default Sprite".to_string();
        let mut flip_x = false;
        let mut flip_y = false;
        
        // Define field layout function
        let field_layout = |ui: &mut Ui, label: &str, content: Box<dyn FnOnce(&mut Ui)>| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add_space(ui.available_width() * 0.3 - label.len() as f32 * 7.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    content(ui);
                });
            });
        };
        
        field_layout(ui, "Sprite", Box::new(|ui| {
            ui.horizontal(|ui| {
                // Mock sprite preview
                let rect = ui.allocate_space(Vec2::new(40.0, 40.0)).1;
                ui.painter().rect_filled(rect, 3.0, Color32::DARK_GRAY);
                ui.painter().rect_stroke(rect, 3.0, Stroke::new(1.0, Color32::WHITE));
                
                if ui.button("Select").clicked() {
                    // Would open sprite selector
                }
            });
        }));
        
        field_layout(ui, "Color", Box::new(|ui| {
            ui.color_edit_button_rgba_unmultiplied(&mut color);
        }));
        
        field_layout(ui, "Material", Box::new(|ui| {
            ui.text_edit_singleline(&mut material);
        }));
        
        field_layout(ui, "Sorting Layer", Box::new(|ui| {
            egui::ComboBox::from_id_source("sorting_layer")
                .selected_text(sorting_layers[sorting_layer])
                .show_ui(ui, |ui| {
                    for (i, &layer_name) in sorting_layers.iter().enumerate() {
                        ui.selectable_value(&mut sorting_layer, i, layer_name);
                    }
                });
        }));
        
        field_layout(ui, "Order in Layer", Box::new(|ui| {
            ui.add(egui::DragValue::new(&mut order_in_layer));
        }));
        
        field_layout(ui, "Flip", Box::new(|ui| {
            ui.checkbox(&mut flip_x, "X");
            ui.checkbox(&mut flip_y, "Y");
        }));
    }
    
    /// Render rigidbody2d component
    fn render_rigidbody2d_component(&self, ui: &mut Ui) {
        let mut body_type = 0;
        let body_types = ["Dynamic", "Kinematic", "Static"];
        let mut mass = 1.0;
        let mut linear_drag = 0.0;
        let mut angular_drag = 0.05;
        let mut gravity_scale = 1.0;
        let mut freeze_rotation = false;
        
        // Define field layout function
        let field_layout = |ui: &mut Ui, label: &str, content: Box<dyn FnOnce(&mut Ui)>| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add_space(ui.available_width() * 0.3 - label.len() as f32 * 7.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    content(ui);
                });
            });
        };
        
        field_layout(ui, "Body Type", Box::new(|ui| {
            egui::ComboBox::from_id_source("body_type")
                .selected_text(body_types[body_type])
                .show_ui(ui, |ui| {
                    for (i, &type_name) in body_types.iter().enumerate() {
                        ui.selectable_value(&mut body_type, i, type_name);
                    }
                });
        }));
        
        field_layout(ui, "Mass", Box::new(|ui| {
            ui.add(egui::DragValue::new(&mut mass).speed(0.1).clamp_range(0.0001..=1000.0));
        }));
        
        field_layout(ui, "Linear Drag", Box::new(|ui| {
            ui.add(egui::DragValue::new(&mut linear_drag).speed(0.01).clamp_range(0.0..=1000.0));
        }));
        
        field_layout(ui, "Angular Drag", Box::new(|ui| {
            ui.add(egui::DragValue::new(&mut angular_drag).speed(0.01).clamp_range(0.0..=1000.0));
        }));
        
        field_layout(ui, "Gravity Scale", Box::new(|ui| {
            ui.add(egui::DragValue::new(&mut gravity_scale).speed(0.1));
        }));
        
        field_layout(ui, "Freeze Rotation", Box::new(|ui| {
            ui.checkbox(&mut freeze_rotation, "");
        }));
    }
    
    /// Render box collider2d component
    fn render_box_collider2d_component(&self, ui: &mut Ui) {
        let mut offset = [0.0, 0.0];
        let mut size = [1.0, 1.0];
        let mut density = 1.0;
        let mut is_trigger = false;
        let mut material = "Default".to_string();
        
        // Define field layout function
        let field_layout = |ui: &mut Ui, label: &str, content: Box<dyn FnOnce(&mut Ui)>| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add_space(ui.available_width() * 0.3 - label.len() as f32 * 7.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    content(ui);
                });
            });
        };
        
        field_layout(ui, "Material", Box::new(|ui| {
            ui.text_edit_singleline(&mut material);
            if ui.button("⋯").clicked() {
                // Would open material selector
            }
        }));
        
        field_layout(ui, "Is Trigger", Box::new(|ui| {
            ui.checkbox(&mut is_trigger, "");
        }));
        
        field_layout(ui, "Offset", Box::new(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("X");
                    ui.add(egui::DragValue::new(&mut offset[0]).speed(0.1));
                });
                ui.horizontal(|ui| {
                    ui.label("Y");
                    ui.add(egui::DragValue::new(&mut offset[1]).speed(0.1));
                });
            });
        }));
        
        field_layout(ui, "Size", Box::new(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("X");
                    ui.add(egui::DragValue::new(&mut size[0]).speed(0.1).clamp_range(0.001..=1000.0));
                });
                ui.horizontal(|ui| {
                    ui.label("Y");
                    ui.add(egui::DragValue::new(&mut size[1]).speed(0.1).clamp_range(0.001..=1000.0));
                });
            });
        }));
        
        field_layout(ui, "Density", Box::new(|ui| {
            ui.add(egui::DragValue::new(&mut density).speed(0.1));
        }));
    }
    
    /// Render lua script component
    fn render_lua_script_component(&self, ui: &mut Ui) {
        // Mock script editor with fields
        let mut script_path = "Scripts/Player.rs".to_string();
        
        // Define field layout function
        let field_layout = |ui: &mut Ui, label: &str, content: Box<dyn FnOnce(&mut Ui)>| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add_space(ui.available_width() * 0.3 - label.len() as f32 * 7.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    content(ui);
                });
            });
        };
        
        field_layout(ui, "Script", Box::new(|ui| {
            ui.text_edit_singleline(&mut script_path);
            if ui.button("Edit").clicked() {
                // Would open script editor
            }
        }));
        
        ui.separator();
        ui.label("Public Variables");
        
        // Mock public variables
        let mut speed = 5.0;
        let mut jump_force = 10.0;
        let mut health = 100;
        let mut player_name = "Player".to_string();
        let mut is_invincible = false;
        
        field_layout(ui, "Speed", Box::new(|ui| {
            ui.add(egui::DragValue::new(&mut speed).speed(0.1));
        }));
        
        field_layout(ui, "Jump Force", Box::new(|ui| {
            ui.add(egui::DragValue::new(&mut jump_force).speed(0.1));
        }));
        
        field_layout(ui, "Health", Box::new(|ui| {
            ui.add(egui::DragValue::new(&mut health));
        }));
        
        field_layout(ui, "Player Name", Box::new(|ui| {
            ui.text_edit_singleline(&mut player_name);
        }));
        
        field_layout(ui, "Is Invincible", Box::new(|ui| {
            ui.checkbox(&mut is_invincible, "");
        }));
    }
    
    /// Render audio source component
    fn render_audio_source_component(&self, ui: &mut Ui) {
        // Mock audio source component
        ui.label("Audio Source component properties would be shown here");
    }
    
    /// Render audio listener component
    fn render_audio_listener_component(&self, ui: &mut Ui) {
        // Mock audio listener component
        ui.label("Audio Listener component properties would be shown here");
    }
    
    /// Render add component menu
    fn render_add_component_menu(&mut self, ui: &mut Ui, _entity_id: u32, log_info: &mut dyn FnMut(&str)) {
        // Unity-like add component dropdown
        Frame::none()
            .fill(Color32::from_rgb(60, 60, 60))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Add Component");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if ui.button("✖").clicked() {
                                self.show_add_component_menu = false;
                            }
                        });
                    });
                    
                    ui.separator();
                    
                    // Search box
                    ui.horizontal(|ui| {
                        ui.label("🔍");
                        ui.add(egui::TextEdit::singleline(&mut self.add_component_search)
                            .hint_text("Search...")
                            .desired_width(f32::INFINITY));
                    });
                    
                    ui.separator();
                    
                    ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        // Common components section
                        ui.collapsing("Common", |ui| {
                            if ui.selectable_label(false, "Rigidbody 2D").clicked() {
                                log_info("Added Rigidbody 2D component");
                                self.show_add_component_menu = false;
                            }
                            
                            if ui.selectable_label(false, "Box Collider 2D").clicked() {
                                log_info("Added Box Collider 2D component");
                                self.show_add_component_menu = false;
                            }
                            
                            if ui.selectable_label(false, "Sprite Renderer").clicked() {
                                log_info("Added Sprite Renderer component");
                                self.show_add_component_menu = false;
                            }
                        });
                        
                        // Physics section
                        ui.collapsing("Physics", |ui| {
                            if ui.selectable_label(false, "Rigidbody").clicked() {
                                log_info("Added Rigidbody component");
                                self.show_add_component_menu = false;
                            }
                            
                            if ui.selectable_label(false, "Box Collider").clicked() {
                                log_info("Added Box Collider component");
                                self.show_add_component_menu = false;
                            }
                        });
                        
                        // Rendering section
                        ui.collapsing("Rendering", |ui| {
                            if ui.selectable_label(false, "Camera").clicked() {
                                log_info("Added Camera component");
                                self.show_add_component_menu = false;
                            }
                            
                            if ui.selectable_label(false, "Light").clicked() {
                                log_info("Added Light component");
                                self.show_add_component_menu = false;
                            }
                        });
                        
                        // Audio section
                        ui.collapsing("Audio", |ui| {
                            if ui.selectable_label(false, "Audio Source").clicked() {
                                log_info("Added Audio Source component");
                                self.show_add_component_menu = false;
                            }
                            
                            if ui.selectable_label(false, "Audio Listener").clicked() {
                                log_info("Added Audio Listener component");
                                self.show_add_component_menu = false;
                            }
                        });
                        
                        // Scripts section
                        ui.collapsing("Scripts", |ui| {
                            if ui.selectable_label(false, "Lua Script").clicked() {
                                log_info("Added Lua Script component");
                                self.show_add_component_menu = false;
                            }
                        });
                    });
                });
            });
    }
} 