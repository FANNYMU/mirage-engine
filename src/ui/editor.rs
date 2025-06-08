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
    /// Whether to show console
    show_console: bool,
    /// Log messages for console
    console_logs: Vec<ConsoleLog>,
    /// Active tab for project panel
    project_active_tab: ProjectTab,
    /// Original window size before resize
    original_window_size: Option<[f32; 2]>,
    /// Project files
    project_files: Vec<ProjectFile>,
    /// Current project path
    current_project_path: Option<String>,
    /// Show rename dialog
    show_rename_dialog: bool,
    /// Rename file path
    rename_file_path: String,
    /// Rename file new name
    rename_file_new_name: String,
    /// Component expanded states
    component_expanded: HashMap<String, bool>,
    /// Entity transforms
    entity_transforms: HashMap<u32, EntityTransform>,
    /// Show add component menu
    show_add_component_menu: bool,
    /// Add component search text
    add_component_search: String,
    /// Show hierarchy search
    show_hierarchy_search: bool,
    /// Hierarchy search text
    hierarchy_search_text: String,
    /// Show create entity menu
    show_create_entity_menu: bool,
    /// Entity parent map
    entity_parent_map: HashMap<u32, u32>,
    /// Drag entity ID
    drag_entity_id: Option<u32>,
}

/// Console log level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LogLevel {
    Info,
    Warning,
    Error,
}

/// Console log entry
#[derive(Debug, Clone)]
struct ConsoleLog {
    timestamp: String,
    level: LogLevel,
    message: String,
}

/// Project panel tab
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectTab {
    Files,
    Console,
}

/// Project file type
#[derive(Debug, Clone, PartialEq, Eq)]
enum ProjectFileType {
    Folder,
    Scene,
    Script,
    Texture,
    Audio,
    Other,
}

/// Project file
#[derive(Debug, Clone)]
struct ProjectFile {
    name: String,
    file_type: ProjectFileType,
    path: String,
    children: Vec<ProjectFile>,
    expanded: bool,
    parent_path: Option<String>,
}

/// Component type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComponentType {
    Transform,
    Camera,
    Light,
    SpriteRenderer,
    Rigidbody2D,
    BoxCollider2D,
    LuaScript,
}

/// Entity component
#[derive(Debug, Clone)]
struct EntityComponent {
    name: String,
    component_type: ComponentType,
    removable: bool,
}

/// Entity transform data
#[derive(Debug, Clone)]
struct EntityTransform {
    position: [f32; 3],
    rotation: [f32; 3],
    scale: [f32; 3],
}

/// Hierarchy item for representing entity hierarchies
#[derive(Debug, Clone)]
struct HierarchyItem {
    id: u32,
    name: String,
    children: Vec<HierarchyItem>,
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
        
        // Create sample console logs
        let console_logs = vec![
            ConsoleLog {
                timestamp: "12:34:56".to_string(),
                level: LogLevel::Info,
                message: "Editor initialized".to_string(),
            },
            ConsoleLog {
                timestamp: "12:34:57".to_string(),
                level: LogLevel::Info,
                message: "Scene loaded".to_string(),
            },
            ConsoleLog {
                timestamp: "12:34:58".to_string(),
                level: LogLevel::Warning,
                message: "Missing sprite reference".to_string(),
            },
            ConsoleLog {
                timestamp: "12:34:59".to_string(),
                level: LogLevel::Info,
                message: "Project assets loaded".to_string(),
            },
            ConsoleLog {
                timestamp: "12:35:00".to_string(),
                level: LogLevel::Error,
                message: "Failed to load audio file".to_string(),
            },
        ];
        
        // Create sample project structure
        let project_files = vec![
            ProjectFile {
                name: "Scripts".to_string(),
                file_type: ProjectFileType::Folder,
                path: "/Scripts".to_string(),
                children: vec![
                    ProjectFile {
                        name: "Player.rs".to_string(),
                        file_type: ProjectFileType::Script,
                        path: "/Scripts/Player.rs".to_string(),
                        children: vec![],
                        expanded: false,
                        parent_path: Some("/Scripts".to_string()),
                    },
                    ProjectFile {
                        name: "Enemy.rs".to_string(),
                        file_type: ProjectFileType::Script,
                        path: "/Scripts/Enemy.rs".to_string(),
                        children: vec![],
                        expanded: false,
                        parent_path: Some("/Scripts".to_string()),
                    },
                ],
                expanded: false,
                parent_path: Some("/".to_string()),
            },
            ProjectFile {
                name: "Scenes".to_string(),
                file_type: ProjectFileType::Folder,
                path: "/Scenes".to_string(),
                children: vec![
                    ProjectFile {
                        name: "MainScene.scene".to_string(),
                        file_type: ProjectFileType::Scene,
                        path: "/Scenes/MainScene.scene".to_string(),
                        children: vec![],
                        expanded: false,
                        parent_path: Some("/Scenes".to_string()),
                    },
                ],
                expanded: false,
                parent_path: Some("/".to_string()),
            },
            ProjectFile {
                name: "Textures".to_string(),
                file_type: ProjectFileType::Folder,
                path: "/Textures".to_string(),
                children: vec![
                    ProjectFile {
                        name: "Player.png".to_string(),
                        file_type: ProjectFileType::Texture,
                        path: "/Textures/Player.png".to_string(),
                        children: vec![],
                        expanded: false,
                        parent_path: Some("/Textures".to_string()),
                    },
                    ProjectFile {
                        name: "Background.png".to_string(),
                        file_type: ProjectFileType::Texture,
                        path: "/Textures/Background.png".to_string(),
                        children: vec![],
                        expanded: false,
                        parent_path: Some("/Textures".to_string()),
                    },
                ],
                expanded: false,
                parent_path: Some("/".to_string()),
            },
        ];
        
        Self {
            selected_entity: None,
            hierarchy_expanded: HashMap::new(),
            scene_view_size: [800.0, 600.0],
            game_view_size: [800.0, 600.0],
            play_mode: false,
            scene_view_tool: SceneViewTool::Select,
            entity_names,
            show_console: false,
            console_logs,
            project_active_tab: ProjectTab::Files,
            original_window_size: None,
            project_files,
            current_project_path: None,
            show_rename_dialog: false,
            rename_file_path: String::new(),
            rename_file_new_name: String::new(),
            component_expanded: HashMap::new(),
            entity_transforms: HashMap::new(),
            show_add_component_menu: false,
            add_component_search: String::new(),
            show_hierarchy_search: false,
            hierarchy_search_text: String::new(),
            show_create_entity_menu: false,
            entity_parent_map: HashMap::new(),
            drag_entity_id: None,
        }
    }
    
    /// Update the editor UI
    pub fn update(&mut self, ctx: &Context, _delta_time: f32) {
        // Store original window size if not already stored
        if self.original_window_size.is_none() {
            self.original_window_size = Some([ctx.screen_rect().width(), ctx.screen_rect().height()]);
        }
        
        // Render the UI
        self.render_menu_bar(ctx);
        self.render_hierarchy_panel(ctx);
        self.render_inspector_panel(ctx);
        self.render_scene_view(ctx);
        
        // Only render game view in play mode
        if self.play_mode {
            self.render_game_view(ctx);
        }
        
        self.render_project_panel(ctx);
        
        // Show console window if enabled
        if self.show_console {
            self.render_console_window(ctx);
        }
        
        // Show rename dialog if needed
        if self.show_rename_dialog {
            self.render_rename_dialog(ctx);
        }
    }
    
    /// Get the original window size
    pub fn get_original_size(&self) -> Option<[f32; 2]> {
        self.original_window_size
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
                
                ui.menu_button("Window", |ui| {
                    if ui.button("Toggle Console").clicked() {
                        self.show_console = !self.show_console;
                        ui.close_menu();
                    }
                    if ui.button("Reset Layout").clicked() {
                        // TODO: Reset layout
                        ui.close_menu();
                    }
                });
                
                // Title in the center
                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                    ui.heading("Mirage Engine");
                });
                
                // Play mode controls and console toggle on the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let console_text = if self.show_console { "Hide Console" } else { "Show Console" };
                    if ui.button(console_text).clicked() {
                        self.show_console = !self.show_console;
                    }
                    
                    ui.separator();
                    
                    if self.play_mode {
                        if ui.button("■ Stop").clicked() {
                            self.play_mode = false;
                            // Reset scene
                            self.log_info("Game stopped");
                        }
                    } else {
                        if ui.button("▶ Play").clicked() {
                            self.play_mode = true;
                            self.log_info("Game started");
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
            .min_width(150.0)
            .max_width(300.0)
            .show(ctx, |ui| {
                // Hierarchy header with toolbar
                ui.horizontal(|ui| {
                    ui.heading("Hierarchy");
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("⚙").on_hover_text("Hierarchy Settings").clicked() {
                            // TODO: Hierarchy settings
                        }
                        
                        if ui.button("🔍").on_hover_text("Search").clicked() {
                            self.show_hierarchy_search = !self.show_hierarchy_search;
                        }
                    });
                });
                
                ui.separator();
                
                // Search bar
                if self.show_hierarchy_search {
                    ui.horizontal(|ui| {
                        ui.label("🔍");
                        
                        let mut search_text = self.hierarchy_search_text.clone();
                        if ui.text_edit_singleline(&mut search_text).changed() {
                            self.hierarchy_search_text = search_text;
                        }
                        
                        if !self.hierarchy_search_text.is_empty() {
                            if ui.button("✕").clicked() {
                                self.hierarchy_search_text.clear();
                            }
                        }
                    });
                    
                    ui.separator();
                }
                
                // Create entity button row
                ui.horizontal(|ui| {
                    if ui.button("+ Create").clicked() {
                        self.show_create_entity_menu = true;
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("⇧").on_hover_text("Move Up").clicked() && self.selected_entity.is_some() {
                            self.log_info("Moved entity up in hierarchy");
                        }
                        
                        if ui.button("⇩").on_hover_text("Move Down").clicked() && self.selected_entity.is_some() {
                            self.log_info("Moved entity down in hierarchy");
                        }
                    });
                });
                
                // Create entity popup menu
                if self.show_create_entity_menu {
                    self.render_create_entity_menu(ctx);
                }
                
                ui.separator();
                
                // Entity list
                ScrollArea::vertical().show(ui, |ui| {
                    // Organize entities into a hierarchy
                    let hierarchy = self.build_entity_hierarchy();
                    
                    // Display entities
                    if hierarchy.is_empty() {
                        ui.label("No entities in scene");
                        ui.label("Click '+ Create' to add entities");
                    } else {
                        self.render_entity_hierarchy(ui, &hierarchy, 0);
                    }
                });
            });
    }
    
    /// Build entity hierarchy for display
    fn build_entity_hierarchy(&self) -> Vec<HierarchyItem> {
        let mut hierarchy = Vec::new();
        
        // Add all entities to the hierarchy
        for (&id, name) in &self.entity_names {
            // Skip entities that don't match search if search is active
            if !self.hierarchy_search_text.is_empty() {
                let search_lower = self.hierarchy_search_text.to_lowercase();
                let name_lower = name.to_lowercase();
                if !name_lower.contains(&search_lower) {
                    continue;
                }
            }
            
            // Check if this entity has a parent
            if let Some(parent_id) = self.entity_parent_map.get(&id) {
                // This entity has a parent, we'll add it to the parent's children later
                continue;
            }
            
            // Create hierarchy item for root entity
            let mut item = HierarchyItem {
                id,
                name: name.clone(),
                children: Vec::new(),
            };
            
            // Add children
            self.add_children_to_hierarchy(&mut item);
            
            hierarchy.push(item);
        }
        
        hierarchy
    }
    
    /// Add children to a hierarchy item
    fn add_children_to_hierarchy(&self, parent: &mut HierarchyItem) {
        // Find all entities that have this parent
        for (&id, name) in &self.entity_names {
            if let Some(parent_id) = self.entity_parent_map.get(&id) {
                if *parent_id == parent.id {
                    let mut child = HierarchyItem {
                        id,
                        name: name.clone(),
                        children: Vec::new(),
                    };
                    
                    // Recursively add children
                    self.add_children_to_hierarchy(&mut child);
                    
                    parent.children.push(child);
                }
            }
        }
    }
    
    /// Render the entity hierarchy
    fn render_entity_hierarchy(&mut self, ui: &mut Ui, items: &[HierarchyItem], depth: usize) {
        for item in items {
            // Get entity info
            let id = item.id;
            let name = &item.name;
            let has_children = !item.children.is_empty();
            let is_expanded = *self.hierarchy_expanded.get(&id).unwrap_or(&false);
            let is_selected = self.selected_entity.map_or(false, |e| e == id);
            
            // Indent based on depth
            let indent = (depth as f32) * 20.0;
            ui.horizontal(|ui| {
                ui.add_space(indent);
                
                // Expand/collapse arrow
                if has_children {
                    let expand_text = if is_expanded { "▼" } else { "►" };
                    if ui.selectable_label(false, expand_text).clicked() {
                        // Toggle expanded state after ui update
                        let new_state = !is_expanded;
                        self.hierarchy_expanded.insert(id, new_state);
                    }
                } else {
                    ui.add_space(20.0); // Space for alignment when no arrow
                }
                
                // Entity name
                let text_color = match id {
                    1 => Color32::from_rgb(180, 180, 60),    // Camera
                    2 => Color32::from_rgb(255, 200, 80),    // Light
                    _ => Color32::WHITE,
                };
                
                let entity_text = RichText::new(name).color(text_color);
                
                let response = ui.selectable_label(is_selected, entity_text);
                
                if response.clicked() {
                    self.selected_entity = Some(id);
                    self.log_info(&format!("Selected entity: {}", name));
                }
                
                if response.double_clicked() {
                    // Focus on entity in scene view
                    self.log_info(&format!("Focusing on entity: {}", name));
                }
                
                // Dragging
                if response.dragged() {
                    // Mock drag operation
                    self.drag_entity_id = Some(id);
                }
                
                if response.drag_released() {
                    // Mock drop operation
                    if let Some(drag_id) = self.drag_entity_id {
                        self.log_info(&format!("Dropped entity: {}", self.entity_names.get(&drag_id).unwrap()));
                        self.drag_entity_id = None;
                    }
                }
                
                // Context menu
                response.context_menu(|ui| {
                    if ui.button("Rename").clicked() {
                        // In a full implementation, this would show a rename dialog
                        self.log_info(&format!("Renaming entity: {}", name));
                        ui.close_menu();
                    }
                    
                    if ui.button("Duplicate").clicked() {
                        // Duplicate entity
                        let new_id = self.entity_names.len() as u32 + 1;
                        let new_name = format!("{} (Copy)", name);
                        self.entity_names.insert(new_id, new_name.clone());
                        
                        // Copy transform if exists
                        if let Some(transform) = self.entity_transforms.get(&id) {
                            self.entity_transforms.insert(new_id, transform.clone());
                        }
                        
                        // Set parent if this entity has one
                        if let Some(parent_id) = self.entity_parent_map.get(&id) {
                            self.entity_parent_map.insert(new_id, *parent_id);
                        }
                        
                        self.selected_entity = Some(new_id);
                        self.log_info(&format!("Duplicated entity: {}", name));
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    
                    ui.menu_button("Create Child", |ui| {
                        if ui.button("Empty").clicked() {
                            let new_id = self.entity_names.len() as u32 + 1;
                            let new_name = format!("Child of {}", name);
                            self.entity_names.insert(new_id, new_name);
                            self.entity_parent_map.insert(new_id, id);
                            self.selected_entity = Some(new_id);
                            self.log_info(&format!("Created child entity for: {}", name));
                            ui.close_menu();
                        }
                        
                        if ui.button("Sprite").clicked() {
                            let new_id = self.entity_names.len() as u32 + 1;
                            let new_name = format!("Sprite Child of {}", name);
                            self.entity_names.insert(new_id, new_name);
                            self.entity_parent_map.insert(new_id, id);
                            self.selected_entity = Some(new_id);
                            self.log_info(&format!("Created sprite child entity for: {}", name));
                            ui.close_menu();
                        }
                    });
                    
                    ui.separator();
                    
                    if ui.button("Delete").clicked() {
                        self.entity_names.remove(&id);
                        self.entity_transforms.remove(&id);
                        self.entity_parent_map.remove(&id);
                        
                        // Remove any children
                        let children_to_remove: Vec<u32> = self.entity_parent_map
                            .iter()
                            .filter(|(_, &parent)| parent == id)
                            .map(|(&child, _)| child)
                            .collect();
                        
                        for child in children_to_remove {
                            self.entity_names.remove(&child);
                            self.entity_transforms.remove(&child);
                            self.entity_parent_map.remove(&child);
                        }
                        
                        if self.selected_entity == Some(id) {
                            self.selected_entity = None;
                        }
                        
                        self.log_info(&format!("Deleted entity: {}", name));
                        ui.close_menu();
                    }
                });
            });
            
            // Render children if expanded
            if is_expanded && has_children {
                self.render_entity_hierarchy(ui, &item.children, depth + 1);
            }
        }
    }
    
    /// Render the create entity menu
    fn render_create_entity_menu(&mut self, ctx: &Context) {
        let mut open = true;
        Window::new("Create")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .fixed_size(Vec2::new(200.0, 300.0))
            .show(ctx, |ui| {
                // 2D Objects section
                ui.collapsing("2D Objects", |ui| {
                    if ui.selectable_label(false, "Sprite").clicked() {
                        self.create_sprite_entity();
                        self.show_create_entity_menu = false;
                    }
                    
                    if ui.selectable_label(false, "Text").clicked() {
                        self.create_empty_entity(); // Use empty entity for now
                        self.show_create_entity_menu = false;
                    }
                });
                
                // 3D Objects section (for future expansion)
                ui.collapsing("3D Objects", |ui| {
                    if ui.selectable_label(false, "Cube").clicked() {
                        self.create_empty_entity(); // Use empty entity for now
                        self.show_create_entity_menu = false;
                    }
                    
                    if ui.selectable_label(false, "Sphere").clicked() {
                        self.create_empty_entity(); // Use empty entity for now
                        self.show_create_entity_menu = false;
                    }
                });
                
                // Effects section
                ui.collapsing("Effects", |ui| {
                    if ui.selectable_label(false, "Particle System").clicked() {
                        self.create_empty_entity(); // Use empty entity for now
                        self.show_create_entity_menu = false;
                    }
                });
                
                // Light section
                ui.collapsing("Light", |ui| {
                    if ui.selectable_label(false, "Directional Light").clicked() {
                        self.create_empty_entity(); // Use empty entity for now
                        self.show_create_entity_menu = false;
                    }
                    
                    if ui.selectable_label(false, "Point Light").clicked() {
                        self.create_empty_entity(); // Use empty entity for now
                        self.show_create_entity_menu = false;
                    }
                });
                
                // Camera section
                ui.collapsing("Cameras", |ui| {
                    if ui.selectable_label(false, "Camera").clicked() {
                        self.create_empty_entity(); // Use empty entity for now
                        self.show_create_entity_menu = false;
                    }
                });
                
                // UI section
                ui.collapsing("UI", |ui| {
                    if ui.selectable_label(false, "Button").clicked() {
                        self.create_empty_entity(); // Use empty entity for now
                        self.show_create_entity_menu = false;
                    }
                    
                    if ui.selectable_label(false, "Text").clicked() {
                        self.create_empty_entity(); // Use empty entity for now
                        self.show_create_entity_menu = false;
                    }
                    
                    if ui.selectable_label(false, "Image").clicked() {
                        self.create_empty_entity(); // Use empty entity for now
                        self.show_create_entity_menu = false;
                    }
                });
                
                ui.separator();
                
                // Empty option
                if ui.selectable_label(false, "Empty Entity").clicked() {
                    self.create_empty_entity();
                    self.show_create_entity_menu = false;
                }
            });
            
        if !open {
            self.show_create_entity_menu = false;
        }
    }
    
    /// Render the inspector panel
    fn render_inspector_panel(&mut self, ctx: &Context) {
        SidePanel::right("inspector_panel")
            .resizable(true)
            .default_width(300.0)
            .min_width(200.0)
            .max_width(400.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Inspector");
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("⚙").on_hover_text("Inspector Settings").clicked() {
                            // TODO: Show inspector settings
                        }
                    });
                });
                ui.separator();
                
                if let Some(entity_id) = self.selected_entity {
                    // Entity header
                    let mut entity_enabled = true; // Mock value
                    ui.horizontal(|ui| {
                        if ui.checkbox(&mut entity_enabled, "").changed() {
                            self.log_info("Entity enabled state changed");
                        }
                        
                        // Get entity name, use a default if not found
                        let mut name_value = match self.entity_names.get(&entity_id) {
                            Some(name) => name.clone(),
                            None => {
                                let default_name = format!("Entity {}", entity_id);
                                self.entity_names.insert(entity_id, default_name.clone());
                                default_name
                            }
                        };
                        
                        // Name editor with Unity-like styling
                        ui.add_sized([ui.available_width(), 20.0], egui::TextEdit::singleline(&mut name_value)
                            .hint_text("Entity Name"));
                        
                        if name_value != self.entity_names.get(&entity_id).unwrap().clone() {
                            // Update name
                            self.entity_names.insert(entity_id, name_value.clone());
                            self.log_info(&format!("Renamed entity to: {}", name_value));
                        }
                    });
                    
                    ui.add_space(8.0);
                    
                    // Static Header
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Static").strong());
                        let mut is_static = false;
                        if ui.checkbox(&mut is_static, "").changed() {
                            self.log_info("Static flag changed");
                        }
                    });
                    
                    ui.add_space(4.0);
                    
                    // Tag and Layer dropdowns
                    ui.horizontal(|ui| {
                        ui.label("Tag");
                        let mut tag = "Untagged";
                        egui::ComboBox::from_id_source("entity_tag")
                            .selected_text(tag)
                            .width(ui.available_width())
                            .show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                ui.set_min_width(120.0);
                                ui.selectable_value(&mut tag, "Untagged", "Untagged");
                                ui.selectable_value(&mut tag, "Player", "Player");
                                ui.selectable_value(&mut tag, "Enemy", "Enemy");
                                ui.selectable_value(&mut tag, "MainCamera", "MainCamera");
                            });
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Layer");
                        let mut layer = "Default";
                        egui::ComboBox::from_id_source("entity_layer")
                            .selected_text(layer)
                            .width(ui.available_width())
                            .show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                ui.set_min_width(120.0);
                                ui.selectable_value(&mut layer, "Default", "Default");
                                ui.selectable_value(&mut layer, "UI", "UI");
                                ui.selectable_value(&mut layer, "Player", "Player");
                                ui.selectable_value(&mut layer, "Enemy", "Enemy");
                            });
                    });
                    
                    ui.add_space(10.0);
                    
                    // Get component data
                    let entity_components = self.get_entity_components(entity_id);
                    
                    // Render each component
                    for component in entity_components {
                        self.render_component(ui, &component, entity_id);
                    }
                    
                    ui.add_space(10.0);
                    
                    // Add Component button
                    if ui.button(RichText::new("Add Component").strong()).clicked() {
                        self.show_add_component_menu = true;
                    }
                    
                    // Add Component menu
                    if self.show_add_component_menu {
                        self.render_add_component_menu(ctx, entity_id);
                    }
                } else {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.label("No entity selected");
                        ui.label("Select an entity from the Hierarchy panel");
                    });
                }
            });
    }
    
    /// Get the components for an entity
    fn get_entity_components(&self, entity_id: u32) -> Vec<EntityComponent> {
        let mut components = Vec::new();
        
        // Always add Transform component
        components.push(EntityComponent {
            name: "Transform".to_string(),
            component_type: ComponentType::Transform,
            removable: false,
        });
        
        // Add entity-specific components
        match entity_id {
            1 => { // Camera
                components.push(EntityComponent {
                    name: "Camera".to_string(),
                    component_type: ComponentType::Camera,
                    removable: true,
                });
            },
            2 => { // Light
                components.push(EntityComponent {
                    name: "Light".to_string(),
                    component_type: ComponentType::Light,
                    removable: true,
                });
            },
            3 => { // Player
                components.push(EntityComponent {
                    name: "Sprite Renderer".to_string(),
                    component_type: ComponentType::SpriteRenderer,
                    removable: true,
                });
                components.push(EntityComponent {
                    name: "Rigidbody 2D".to_string(),
                    component_type: ComponentType::Rigidbody2D,
                    removable: true,
                });
                components.push(EntityComponent {
                    name: "Box Collider 2D".to_string(),
                    component_type: ComponentType::BoxCollider2D,
                    removable: true,
                });
                components.push(EntityComponent {
                    name: "Lua Script".to_string(),
                    component_type: ComponentType::LuaScript,
                    removable: true,
                });
            },
            4 => { // Background
                components.push(EntityComponent {
                    name: "Sprite Renderer".to_string(),
                    component_type: ComponentType::SpriteRenderer,
                    removable: true,
                });
            },
            _ => {}
        }
        
        components
    }
    
    /// Render a component in the inspector
    fn render_component(&mut self, ui: &mut Ui, component: &EntityComponent, entity_id: u32) {
        // Get component info
        let component_id = format!("{}_{}", entity_id, component.name);
        let is_expanded = *self.component_expanded.get(&component_id).unwrap_or(&true);
        let name = &component.name;
        let removable = component.removable;
        let component_type = component.component_type;
        
        ui.group(|ui| {
            // Component header
            ui.horizontal(|ui| {
                let expand_text = if is_expanded { "▼" } else { "►" };
                if ui.selectable_label(false, expand_text).clicked() {
                    // Toggle expanded state after ui update
                    let new_state = !is_expanded;
                    self.component_expanded.insert(component_id.clone(), new_state);
                }
                
                ui.label(RichText::new(name).strong());
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if removable {
                        if ui.button("⊖").on_hover_text("Remove Component").clicked() {
                            // Mock removal
                            self.log_info(&format!("Removed component: {}", name));
                            // In a real implementation, the component would be removed from the entity
                        }
                    }
                    
                    if ui.button("⋮").on_hover_text("Component Options").clicked() {
                        // Mock component options
                    }
                });
            });
            
            // Component content if expanded
            if is_expanded {
                match component_type {
                    ComponentType::Transform => self.render_transform_component(ui, entity_id),
                    ComponentType::Camera => self.render_camera_component(ui),
                    ComponentType::Light => self.render_light_component(ui),
                    ComponentType::SpriteRenderer => self.render_sprite_renderer_component(ui),
                    ComponentType::Rigidbody2D => self.render_rigidbody2d_component(ui),
                    ComponentType::BoxCollider2D => self.render_box_collider2d_component(ui),
                    ComponentType::LuaScript => self.render_lua_script_component(ui),
                }
            }
        });
    }
    
    /// Render the Transform component
    fn render_transform_component(&mut self, ui: &mut Ui, entity_id: u32) {
        // Get the entity transform data
        let transform = self.entity_transforms.entry(entity_id).or_insert_with(|| EntityTransform {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        });
        
        ui.add_space(4.0);
        
        // Position
        ui.horizontal(|ui| {
            ui.label(RichText::new("Position").strong());
            ui.add(egui::DragValue::new(&mut transform.position[0]).speed(0.1).prefix("X: "));
            ui.add(egui::DragValue::new(&mut transform.position[1]).speed(0.1).prefix("Y: "));
            ui.add(egui::DragValue::new(&mut transform.position[2]).speed(0.1).prefix("Z: "));
        });
        
        // Rotation
        ui.horizontal(|ui| {
            ui.label(RichText::new("Rotation").strong());
            ui.add(egui::DragValue::new(&mut transform.rotation[0]).speed(0.1).prefix("X: "));
            ui.add(egui::DragValue::new(&mut transform.rotation[1]).speed(0.1).prefix("Y: "));
            ui.add(egui::DragValue::new(&mut transform.rotation[2]).speed(0.1).prefix("Z: "));
        });
        
        // Scale
        ui.horizontal(|ui| {
            ui.label(RichText::new("Scale").strong());
            ui.add(egui::DragValue::new(&mut transform.scale[0]).speed(0.1).prefix("X: "));
            ui.add(egui::DragValue::new(&mut transform.scale[1]).speed(0.1).prefix("Y: "));
            ui.add(egui::DragValue::new(&mut transform.scale[2]).speed(0.1).prefix("Z: "));
        });
    }
    
    /// Render the Camera component
    fn render_camera_component(&self, ui: &mut Ui) {
        ui.add_space(4.0);
        
        // Camera Type
        ui.horizontal(|ui| {
            ui.label("Projection:");
            let mut projection_type = 0;
            ui.radio_value(&mut projection_type, 0, "Perspective");
            ui.radio_value(&mut projection_type, 1, "Orthographic");
        });
        
        // Field of View
        let mut fov = 60.0;
        ui.horizontal(|ui| {
            ui.label("Field of View:");
            ui.add(egui::Slider::new(&mut fov, 1.0..=179.0));
        });
        
        // Clipping Planes
        let mut near_clip = 0.1;
        let mut far_clip = 1000.0;
        ui.horizontal(|ui| {
            ui.label("Near Clip:");
            ui.add(egui::DragValue::new(&mut near_clip).speed(0.01));
        });
        ui.horizontal(|ui| {
            ui.label("Far Clip:");
            ui.add(egui::DragValue::new(&mut far_clip).speed(1.0));
        });
        
        // Viewport Rect
        ui.label("Viewport Rect:");
        ui.horizontal(|ui| {
            let mut viewport_x = 0.0;
            let mut viewport_y = 0.0;
            let mut viewport_w = 1.0;
            let mut viewport_h = 1.0;
            ui.add(egui::DragValue::new(&mut viewport_x).speed(0.01).prefix("X: "));
            ui.add(egui::DragValue::new(&mut viewport_y).speed(0.01).prefix("Y: "));
            ui.add(egui::DragValue::new(&mut viewport_w).speed(0.01).prefix("W: "));
            ui.add(egui::DragValue::new(&mut viewport_h).speed(0.01).prefix("H: "));
        });
    }
    
    /// Render the Light component
    fn render_light_component(&self, ui: &mut Ui) {
        ui.add_space(4.0);
        
        // Light Type
        ui.horizontal(|ui| {
            ui.label("Type:");
            let mut light_type = 0;
            ui.radio_value(&mut light_type, 0, "Directional");
            ui.radio_value(&mut light_type, 1, "Point");
            ui.radio_value(&mut light_type, 2, "Spot");
        });
        
        // Light Color
        ui.horizontal(|ui| {
            ui.label("Color:");
            let mut color = [1.0, 1.0, 1.0, 1.0];
            ui.color_edit_button_rgba_premultiplied(&mut color);
        });
        
        // Intensity
        let mut intensity = 1.0;
        ui.horizontal(|ui| {
            ui.label("Intensity:");
            ui.add(egui::Slider::new(&mut intensity, 0.0..=10.0));
        });
        
        // Range (for point/spot lights)
        let mut range = 10.0;
        ui.horizontal(|ui| {
            ui.label("Range:");
            ui.add(egui::Slider::new(&mut range, 0.1..=100.0));
        });
        
        // Spot Angle (for spot lights)
        let mut spot_angle = 30.0;
        ui.horizontal(|ui| {
            ui.label("Spot Angle:");
            ui.add(egui::Slider::new(&mut spot_angle, 1.0..=179.0));
        });
    }
    
    /// Render the Sprite Renderer component
    fn render_sprite_renderer_component(&self, ui: &mut Ui) {
        ui.add_space(4.0);
        
        // Sprite
        ui.horizontal(|ui| {
            ui.label("Sprite:");
            
            // Mock sprite selector
            let mut sprite = "None";
            egui::ComboBox::from_id_source("sprite_selector")
                .selected_text(sprite)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(120.0);
                    ui.selectable_value(&mut sprite, "None", "None");
                    ui.selectable_value(&mut sprite, "Player.png", "Player.png");
                    ui.selectable_value(&mut sprite, "Background.png", "Background.png");
                });
        });
        
        // Color
        ui.horizontal(|ui| {
            ui.label("Color:");
            let mut color = [1.0, 1.0, 1.0, 1.0];
            ui.color_edit_button_rgba_premultiplied(&mut color);
        });
        
        // Flip options
        ui.horizontal(|ui| {
            let mut flip_x = false;
            let mut flip_y = false;
            ui.checkbox(&mut flip_x, "Flip X");
            ui.checkbox(&mut flip_y, "Flip Y");
        });
        
        // Additional options
        ui.horizontal(|ui| {
            ui.label("Order in Layer:");
            let mut order = 0;
            ui.add(egui::DragValue::new(&mut order));
        });
        
        ui.horizontal(|ui| {
            ui.label("Sorting Layer:");
            let mut layer = "Default";
            egui::ComboBox::from_id_source("sorting_layer")
                .selected_text(layer)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut layer, "Default", "Default");
                    ui.selectable_value(&mut layer, "Background", "Background");
                    ui.selectable_value(&mut layer, "Foreground", "Foreground");
                });
        });
    }
    
    /// Render the Rigidbody2D component
    fn render_rigidbody2d_component(&self, ui: &mut Ui) {
        ui.add_space(4.0);
        
        // Body Type
        ui.horizontal(|ui| {
            ui.label("Body Type:");
            let mut body_type = 0;
            ui.radio_value(&mut body_type, 0, "Dynamic");
            ui.radio_value(&mut body_type, 1, "Kinematic");
            ui.radio_value(&mut body_type, 2, "Static");
        });
        
        // Physical properties
        let mut mass = 1.0;
        ui.horizontal(|ui| {
            ui.label("Mass:");
            ui.add(egui::DragValue::new(&mut mass).speed(0.1));
        });
        
        let mut linear_drag = 0.0;
        ui.horizontal(|ui| {
            ui.label("Linear Drag:");
            ui.add(egui::DragValue::new(&mut linear_drag).speed(0.01));
        });
        
        let mut angular_drag = 0.05;
        ui.horizontal(|ui| {
            ui.label("Angular Drag:");
            ui.add(egui::DragValue::new(&mut angular_drag).speed(0.01));
        });
        
        let mut gravity_scale = 1.0;
        ui.horizontal(|ui| {
            ui.label("Gravity Scale:");
            ui.add(egui::DragValue::new(&mut gravity_scale).speed(0.1));
        });
        
        // Constraints
        ui.label("Constraints:");
        ui.horizontal(|ui| {
            let mut freeze_pos_x = false;
            let mut freeze_pos_y = false;
            let mut freeze_rot = false;
            ui.checkbox(&mut freeze_pos_x, "Freeze Position X");
            ui.checkbox(&mut freeze_pos_y, "Freeze Position Y");
            ui.checkbox(&mut freeze_rot, "Freeze Rotation");
        });
    }
    
    /// Render the BoxCollider2D component
    fn render_box_collider2d_component(&self, ui: &mut Ui) {
        ui.add_space(4.0);
        
        // Material
        ui.horizontal(|ui| {
            ui.label("Material:");
            let mut material = "None";
            egui::ComboBox::from_id_source("physics_material")
                .selected_text(material)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut material, "None", "None");
                    ui.selectable_value(&mut material, "Bouncy", "Bouncy");
                    ui.selectable_value(&mut material, "Ice", "Ice");
                });
        });
        
        // Offset and Size
        ui.horizontal(|ui| {
            ui.label("Offset:");
            let mut offset_x = 0.0;
            let mut offset_y = 0.0;
            ui.add(egui::DragValue::new(&mut offset_x).speed(0.1).prefix("X: "));
            ui.add(egui::DragValue::new(&mut offset_y).speed(0.1).prefix("Y: "));
        });
        
        ui.horizontal(|ui| {
            ui.label("Size:");
            let mut size_x = 1.0;
            let mut size_y = 1.0;
            ui.add(egui::DragValue::new(&mut size_x).speed(0.1).prefix("X: "));
            ui.add(egui::DragValue::new(&mut size_y).speed(0.1).prefix("Y: "));
        });
        
        // Collision properties
        ui.horizontal(|ui| {
            let mut is_trigger = false;
            ui.checkbox(&mut is_trigger, "Is Trigger");
        });
        
        ui.horizontal(|ui| {
            ui.label("Density:");
            let mut density = 1.0;
            ui.add(egui::DragValue::new(&mut density).speed(0.1));
        });
    }
    
    /// Render the Lua Script component
    fn render_lua_script_component(&mut self, ui: &mut Ui) {
        ui.add_space(4.0);
        
        ui.horizontal(|ui| {
            ui.label("Script:");
            let mut script = "PlayerController.lua";
            egui::ComboBox::from_id_source("lua_script")
                .selected_text(script)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut script, "PlayerController.lua", "PlayerController.lua");
                    ui.selectable_value(&mut script, "EnemyBehavior.lua", "EnemyBehavior.lua");
                    ui.selectable_value(&mut script, "Movement.lua", "Movement.lua");
                });
        });
        
        if ui.button("Edit Script").clicked() {
            // Call log_info from self when button is clicked
            let log_msg = "Opening script editor";
            self.log_info(log_msg);
        }
        
        ui.collapsing("Variables", |ui| {
            // Mock variables
            ui.horizontal(|ui| {
                ui.label("Speed:");
                let mut speed = 5.0;
                ui.add(egui::DragValue::new(&mut speed).speed(0.1));
            });
            
            ui.horizontal(|ui| {
                ui.label("Health:");
                let mut health = 100;
                ui.add(egui::DragValue::new(&mut health));
            });
            
            ui.horizontal(|ui| {
                ui.label("Is Enemy:");
                let mut is_enemy = false;
                ui.checkbox(&mut is_enemy, "");
            });
        });
    }
    
    /// Render the Add Component menu
    fn render_add_component_menu(&mut self, ctx: &Context, _entity_id: u32) {
        let mut open = true;
        Window::new("Add Component")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_size([300.0, 400.0])
            .show(ctx, |ui| {
                // Search box
                let mut search_text = self.add_component_search.clone();
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    if ui.text_edit_singleline(&mut search_text).changed() {
                        self.add_component_search = search_text.clone();
                    }
                    
                    if !search_text.is_empty() {
                        if ui.button("✕").clicked() {
                            self.add_component_search.clear();
                        }
                    }
                });
                
                ui.separator();
                
                // Component categories
                ScrollArea::vertical().show(ui, |ui| {
                    let search_lower = self.add_component_search.to_lowercase();
                    
                    // Physics category
                    if search_lower.is_empty() || "physics".contains(&search_lower) {
                        ui.collapsing("Physics", |ui| {
                            if ui.selectable_label(false, "Rigidbody 2D").clicked() {
                                self.log_info("Added Rigidbody 2D component");
                                self.show_add_component_menu = false;
                            }
                            if ui.selectable_label(false, "Box Collider 2D").clicked() {
                                self.log_info("Added Box Collider 2D component");
                                self.show_add_component_menu = false;
                            }
                            if ui.selectable_label(false, "Circle Collider 2D").clicked() {
                                self.log_info("Added Circle Collider 2D component");
                                self.show_add_component_menu = false;
                            }
                            if ui.selectable_label(false, "Polygon Collider 2D").clicked() {
                                self.log_info("Added Polygon Collider 2D component");
                                self.show_add_component_menu = false;
                            }
                        });
                    }
                    
                    // Rendering category
                    if search_lower.is_empty() || "rendering".contains(&search_lower) {
                        ui.collapsing("Rendering", |ui| {
                            if ui.selectable_label(false, "Sprite Renderer").clicked() {
                                self.log_info("Added Sprite Renderer component");
                                self.show_add_component_menu = false;
                            }
                            if ui.selectable_label(false, "Camera").clicked() {
                                self.log_info("Added Camera component");
                                self.show_add_component_menu = false;
                            }
                            if ui.selectable_label(false, "Light").clicked() {
                                self.log_info("Added Light component");
                                self.show_add_component_menu = false;
                            }
                        });
                    }
                    
                    // Scripting category
                    if search_lower.is_empty() || "scripting".contains(&search_lower) {
                        ui.collapsing("Scripting", |ui| {
                            if ui.selectable_label(false, "Lua Script").clicked() {
                                self.log_info("Added Lua Script component");
                                self.show_add_component_menu = false;
                            }
                        });
                    }
                    
                    // Audio category
                    if search_lower.is_empty() || "audio".contains(&search_lower) {
                        ui.collapsing("Audio", |ui| {
                            if ui.selectable_label(false, "Audio Source").clicked() {
                                self.log_info("Added Audio Source component");
                                self.show_add_component_menu = false;
                            }
                            if ui.selectable_label(false, "Audio Listener").clicked() {
                                self.log_info("Added Audio Listener component");
                                self.show_add_component_menu = false;
                            }
                        });
                    }
                    
                    // UI category
                    if search_lower.is_empty() || "ui".contains(&search_lower) {
                        ui.collapsing("UI", |ui| {
                            if ui.selectable_label(false, "Text").clicked() {
                                self.log_info("Added Text component");
                                self.show_add_component_menu = false;
                            }
                            if ui.selectable_label(false, "Image").clicked() {
                                self.log_info("Added Image component");
                                self.show_add_component_menu = false;
                            }
                            if ui.selectable_label(false, "Button").clicked() {
                                self.log_info("Added Button component");
                                self.show_add_component_menu = false;
                            }
                        });
                    }
                });
            });
            
        if !open {
            self.show_add_component_menu = false;
        }
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
                // Handle click in scene view (entity selection)
                let pos = response.interact_pointer_pos();
                if let Some(pos) = pos {
                    // Check if clicked on any entity
                    if self.is_point_in_entity(pos, rect, 3, 0.0) {
                        self.selected_entity = Some(3);
                        self.log_info("Selected player entity");
                    } else if self.is_point_in_entity(pos, rect, 4, 0.0) {
                        self.selected_entity = Some(4);
                        self.log_info("Selected background entity");
                    } else if self.is_point_near_camera(pos, rect) {
                        self.selected_entity = Some(1);
                        self.log_info("Selected camera entity");
                    } else if self.is_point_near_light(pos, rect) {
                        self.selected_entity = Some(2);
                        self.log_info("Selected light entity");
                    } else {
                        self.selected_entity = None;
                    }
                }
            }
            
            if response.dragged() {
                // Handle drag in scene view based on selected tool
                match self.scene_view_tool {
                    SceneViewTool::Select => {
                        // Drag to select multiple entities (not implemented yet)
                    },
                    SceneViewTool::Move => {
                        // Move selected entity
                        if self.selected_entity.is_some() {
                            self.log_info("Moving entity");
                        }
                    },
                    SceneViewTool::Rotate => {
                        // Rotate selected entity
                        if self.selected_entity.is_some() {
                            self.log_info("Rotating entity");
                        }
                    },
                    SceneViewTool::Scale => {
                        // Scale selected entity
                        if self.selected_entity.is_some() {
                            self.log_info("Scaling entity");
                        }
                    },
                }
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
    
    /// Check if a point is inside an entity
    fn is_point_in_entity(&self, point: egui::Pos2, rect: egui::Rect, entity_id: u32, _offset_y: f32) -> bool {
        let center_x = rect.min.x + rect.width() / 2.0;
        let center_y = rect.min.y + rect.height() / 2.0;
        
        match entity_id {
            3 => { // Player
                let player_size = 40.0;
                let player_rect = egui::Rect::from_center_size(
                    egui::pos2(center_x, center_y),
                    egui::vec2(player_size, player_size),
                );
                player_rect.contains(point)
            },
            4 => { // Background
                let bg_rect = egui::Rect::from_center_size(
                    egui::pos2(center_x, center_y + 100.0),
                    egui::vec2(300.0, 50.0),
                );
                bg_rect.contains(point)
            },
            _ => false,
        }
    }
    
    /// Check if a point is near the camera icon
    fn is_point_near_camera(&self, point: egui::Pos2, rect: egui::Rect) -> bool {
        let center_x = rect.min.x + rect.width() / 2.0;
        let center_y = rect.min.y + rect.height() / 2.0;
        let camera_pos = egui::pos2(center_x - 150.0, center_y - 100.0);
        
        (point.x - camera_pos.x).powi(2) + (point.y - camera_pos.y).powi(2) < 30.0_f32.powi(2)
    }
    
    /// Check if a point is near the light icon
    fn is_point_near_light(&self, point: egui::Pos2, rect: egui::Rect) -> bool {
        let center_x = rect.min.x + rect.width() / 2.0;
        let center_y = rect.min.y + rect.height() / 2.0;
        let light_pos = egui::pos2(center_x + 150.0, center_y - 100.0);
        
        (point.x - light_pos.x).powi(2) + (point.y - light_pos.y).powi(2) < 30.0_f32.powi(2)
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
        
        // Draw axes with controlled lengths
        let center_x = rect.min.x + rect.width() / 2.0;
        let center_y = rect.min.y + rect.height() / 2.0;
        
        // Limit axes to the view boundaries
        let max_x_axis = rect.width() / 2.0;
        let max_y_axis = rect.height() / 2.0;
        
        // X axis (red) - constrained to view width
        ui.painter().line_segment(
            [egui::pos2(center_x - max_x_axis, center_y), egui::pos2(center_x + max_x_axis, center_y)],
            egui::Stroke::new(2.0, Color32::from_rgb(200, 50, 50)),
        );
        
        // Y axis (green) - constrained to view height
        ui.painter().line_segment(
            [egui::pos2(center_x, center_y - max_y_axis), egui::pos2(center_x, center_y + max_y_axis)],
            egui::Stroke::new(2.0, Color32::from_rgb(50, 200, 50)),
        );
        
        // Draw axis labels
        ui.painter().text(
            egui::pos2(center_x + max_x_axis - 20.0, center_y + 15.0),
            egui::Align2::CENTER_CENTER,
            "X",
            egui::FontId::proportional(14.0),
            Color32::from_rgb(200, 50, 50),
        );
        
        ui.painter().text(
            egui::pos2(center_x - 15.0, center_y - max_y_axis + 20.0),
            egui::Align2::CENTER_CENTER,
            "Y",
            egui::FontId::proportional(14.0),
            Color32::from_rgb(50, 200, 50),
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
        
        // Outline if selected
        if self.selected_entity == Some(3) {
            ui.painter().rect_stroke(
                player_rect.expand(2.0),
                5.0,
                egui::Stroke::new(2.0, Color32::YELLOW),
            );
        }
        
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
        
        // Outline if selected
        if self.selected_entity == Some(4) {
            ui.painter().rect_stroke(
                bg_rect.expand(2.0),
                3.0,
                egui::Stroke::new(2.0, Color32::YELLOW),
            );
        }
        
        // Draw a mock camera icon
        if self.selected_entity != Some(1) {
            self.draw_camera_icon(ui, center_x - 150.0, center_y - 100.0, 30.0, Color32::from_rgb(200, 200, 50));
        } else {
            self.draw_camera_icon(ui, center_x - 150.0, center_y - 100.0, 30.0, Color32::from_rgb(255, 255, 100));
            
            // Draw selection outline
            ui.painter().circle_stroke(
                egui::pos2(center_x - 150.0, center_y - 100.0),
                20.0,
                egui::Stroke::new(2.0, Color32::YELLOW),
            );
        }
        
        // Draw a mock light icon
        if self.selected_entity != Some(2) {
            self.draw_light_icon(ui, center_x + 150.0, center_y - 100.0, 30.0, Color32::from_rgb(255, 200, 50));
        } else {
            self.draw_light_icon(ui, center_x + 150.0, center_y - 100.0, 30.0, Color32::from_rgb(255, 255, 100));
            
            // Draw selection outline
            ui.painter().circle_stroke(
                egui::pos2(center_x + 150.0, center_y - 100.0),
                20.0,
                egui::Stroke::new(2.0, Color32::YELLOW),
            );
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
            .min_height(100.0)
            .max_height(400.0)
            .show(ctx, |ui| {
                // Tabs for project files and console
                ui.horizontal(|ui| {
                    if ui.selectable_label(self.project_active_tab == ProjectTab::Files, "Project").clicked() {
                        self.project_active_tab = ProjectTab::Files;
                    }
                    if ui.selectable_label(self.project_active_tab == ProjectTab::Console, "Console").clicked() {
                        self.project_active_tab = ProjectTab::Console;
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.project_active_tab == ProjectTab::Console {
                            if ui.button("Clear").clicked() {
                                self.console_logs.clear();
                            }
                        } else {
                            if ui.button("New Folder").clicked() {
                                // TODO: Create new folder
                            }
                            if ui.button("Import").clicked() {
                                // TODO: Import file
                            }
                        }
                    });
                });
                
                ui.separator();
                
                match self.project_active_tab {
                    ProjectTab::Files => self.render_project_files(ui),
                    ProjectTab::Console => self.render_project_console(ui),
                }
            });
    }
    
    /// Render project files
    fn render_project_files(&mut self, ui: &mut Ui) {
        // Project browser toolbar
        ui.horizontal(|ui| {
            if ui.button("New Folder").clicked() {
                // Create new folder at current path
                let current_path = self.current_project_path.clone().unwrap_or_else(|| "/".to_string());
                self.create_project_folder(current_path, "New Folder".to_string());
                self.log_info("Created new folder");
            }
            
            if ui.button("New Script").clicked() {
                // Create new Lua script at current path
                let current_path = self.current_project_path.clone().unwrap_or_else(|| "/".to_string());
                self.create_project_file(current_path, "NewScript.lua".to_string(), ProjectFileType::Script);
                self.log_info("Created new Lua script");
            }
            
            if ui.button("Import Asset").clicked() {
                // Mock importing asset
                let current_path = self.current_project_path.clone().unwrap_or_else(|| "/".to_string());
                self.create_project_file(current_path, "ImportedAsset.png".to_string(), ProjectFileType::Texture);
                self.log_info("Imported asset");
            }
            
            // Show current path
            let display_path = self.current_project_path.clone().unwrap_or_else(|| "/".to_string());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("Path: {}", display_path));
                
                if display_path != "/" {
                    if ui.button("⬆ Up").clicked() {
                        // Go up one directory
                        if let Some(path) = &self.current_project_path {
                            if let Some(last_slash) = path.rfind('/') {
                                if last_slash == 0 {
                                    self.current_project_path = None;
                                } else {
                                    self.current_project_path = Some(path[..last_slash].to_string());
                                }
                                self.log_info(&format!("Navigated to {}", self.current_project_path.clone().unwrap_or_else(|| "/".to_string())));
                            }
                        }
                    }
                }
            });
        });
        
        ui.separator();
        
        // Project file view
        ScrollArea::both().show(ui, |ui| {
            let current_path = self.current_project_path.clone().unwrap_or_else(|| "/".to_string());
            
            if self.project_files.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.heading("Project is empty");
                    ui.label("Use the buttons above to add content");
                    ui.add_space(20.0);
                });
            } else {
                if current_path == "/" {
                    // Create a copy of the files to avoid borrow issues
                    let root_files: Vec<ProjectFile> = self.project_files.clone();
                    // Display root files
                    ui.horizontal_wrapped(|ui| {
                        for file in &root_files {
                            self.render_project_file_icon(ui, file);
                        }
                    });
                } else {
                    // Display files in current directory
                    // Create a copy to avoid borrow issues
                    let files_in_current_dir = self.get_files_in_directory(&current_path);
                    
                    if files_in_current_dir.is_empty() {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.heading("Folder is empty");
                            ui.label("Use the buttons above to add content");
                            ui.add_space(20.0);
                        });
                    } else {
                        // Create clones of files to avoid borrow issues
                        let files_to_display: Vec<ProjectFile> = files_in_current_dir.iter().map(|&f| f.clone()).collect();
                        ui.horizontal_wrapped(|ui| {
                            for file in &files_to_display {
                                self.render_project_file_icon(ui, file);
                            }
                        });
                    }
                }
            }
        });
    }
    
    /// Render a project file as an icon
    fn render_project_file_icon(&mut self, ui: &mut Ui, file: &ProjectFile) {
        let min_size = Vec2::new(100.0, 80.0);
        ui.allocate_ui(min_size, |ui| {
            ui.vertical_centered(|ui| {
                let icon = match file.file_type {
                    ProjectFileType::Folder => "📁",
                    ProjectFileType::Scene => "🎮",
                    ProjectFileType::Script => "📜",
                    ProjectFileType::Texture => "🖼️",
                    ProjectFileType::Audio => "🔊",
                    ProjectFileType::Other => "📄",
                };
                
                let response = ui.add(egui::Label::new(RichText::new(icon).size(32.0)).sense(Sense::click()));
                ui.label(RichText::new(&file.name).size(14.0));
                
                if response.double_clicked() {
                    match file.file_type {
                        ProjectFileType::Folder => {
                            // Navigate into folder
                            self.current_project_path = Some(file.path.clone());
                            self.log_info(&format!("Opened folder: {}", file.name));
                        },
                        ProjectFileType::Scene => {
                            // Open scene (mock)
                            self.log_info(&format!("Opened scene: {}", file.name));
                        },
                        ProjectFileType::Script => {
                            // Open script in editor (mock)
                            self.log_info(&format!("Opened script: {}", file.name));
                        },
                        _ => {
                            // Open asset preview (mock)
                            self.log_info(&format!("Opened asset: {}", file.name));
                        }
                    }
                }
                
                let file_clone = file.clone();
                
                response.context_menu(|ui| {
                    if ui.button("Rename").clicked() {
                        self.show_rename_dialog = true;
                        self.rename_file_path = file_clone.path.clone();
                        self.rename_file_new_name = file_clone.name.clone();
                        ui.close_menu();
                    }
                    
                    if ui.button("Delete").clicked() {
                        self.delete_project_file(&file_clone.path);
                        self.log_info(&format!("Deleted: {}", file_clone.name));
                        ui.close_menu();
                    }
                    
                    if matches!(file_clone.file_type, ProjectFileType::Folder) {
                        ui.separator();
                        
                        if ui.button("New Folder").clicked() {
                            self.create_project_folder(file_clone.path.clone(), "New Folder".to_string());
                            self.log_info("Created new folder");
                            ui.close_menu();
                        }
                        
                        if ui.button("New Script").clicked() {
                            self.create_project_file(file_clone.path.clone(), "NewScript.lua".to_string(), ProjectFileType::Script);
                            self.log_info("Created new Lua script");
                            ui.close_menu();
                        }
                        
                        if ui.button("Import Asset").clicked() {
                            self.create_project_file(file_clone.path.clone(), "ImportedAsset.png".to_string(), ProjectFileType::Texture);
                            self.log_info("Imported asset");
                            ui.close_menu();
                        }
                    }
                });
            });
        });
    }
    
    /// Get files in a specific directory
    fn get_files_in_directory(&self, directory_path: &str) -> Vec<&ProjectFile> {
        let mut result = Vec::new();
        
        // Find files in the current directory
        for root_file in &self.project_files {
            self.collect_files_in_directory(root_file, directory_path, &mut result);
        }
        
        result
    }
    
    /// Recursively collect files in a directory
    fn collect_files_in_directory<'a>(&'a self, file: &'a ProjectFile, directory_path: &str, result: &mut Vec<&'a ProjectFile>) {
        if file.parent_path.is_some() && file.parent_path.as_ref().unwrap() == directory_path {
            // This file is directly in the target directory
            result.push(file);
        } else if matches!(file.file_type, ProjectFileType::Folder) {
            // Search in children
            for child in &file.children {
                self.collect_files_in_directory(child, directory_path, result);
            }
        }
    }
    
    /// Create a new folder in the project
    fn create_project_folder(&mut self, parent_path: String, folder_name: String) {
        let parent_path_clone = parent_path.clone();
        let folder_name_clone = folder_name.clone();
        
        let mut name = folder_name;
        let mut index = 1;
        
        // Ensure folder name is unique
        while self.path_exists(&format!("{}/{}", parent_path_clone, name)) {
            name = format!("{} ({})", folder_name_clone, index);
            index += 1;
        }
        
        let path = if parent_path_clone == "/" {
            format!("/{}", name)
        } else {
            format!("{}/{}", parent_path_clone, name)
        };
        
        let new_folder = ProjectFile {
            name,
            file_type: ProjectFileType::Folder,
            path: path.clone(),
            children: Vec::new(),
            expanded: false,
            parent_path: Some(parent_path_clone),
        };
        
        if parent_path == "/" {
            // Add to root
            self.project_files.push(new_folder);
        } else {
            // Add to parent folder
            self.add_file_to_parent(&parent_path, new_folder);
        }
    }
    
    /// Create a new file in the project
    fn create_project_file(&mut self, parent_path: String, file_name: String, file_type: ProjectFileType) {
        let parent_path_clone = parent_path.clone();
        let file_name_clone = file_name.clone();
        
        let mut name = file_name;
        let mut index = 1;
        
        // Ensure file name is unique
        while self.path_exists(&format!("{}/{}", parent_path_clone, name)) {
            let dot_pos = file_name_clone.rfind('.');
            if let Some(pos) = dot_pos {
                let base_name = &file_name_clone[..pos];
                let extension = &file_name_clone[pos..];
                name = format!("{} ({}){}", base_name, index, extension);
            } else {
                name = format!("{} ({})", file_name_clone, index);
            }
            index += 1;
        }
        
        let path = if parent_path_clone == "/" {
            format!("/{}", name)
        } else {
            format!("{}/{}", parent_path_clone, name)
        };
        
        let new_file = ProjectFile {
            name,
            file_type,
            path: path.clone(),
            children: Vec::new(),
            expanded: false,
            parent_path: Some(parent_path_clone.clone()),
        };
        
        if parent_path == "/" {
            // Add to root
            self.project_files.push(new_file);
        } else {
            // Add to parent folder
            self.add_file_to_parent(&parent_path, new_file);
        }
    }
    
    /// Add a file to its parent folder
    fn add_file_to_parent(&mut self, parent_path: &str, file: ProjectFile) {
        let path = parent_path.to_string();
        
        let mut found = false;
        let mut folder_idx = 0;
        
        // First find the parent folder
        for (i, project_file) in self.project_files.iter().enumerate() {
            if project_file.path == path {
                folder_idx = i;
                found = true;
                break;
            }
            
            if let Some(folder_idx_in_children) = self.find_folder_in_children(project_file, &path) {
                folder_idx = folder_idx_in_children;
                found = true;
                break;
            }
        }
        
        // Then add the file to the parent folder
        if found {
            // Add file to folder's children
            self.project_files[folder_idx].children.push(file);
        }
    }
    
    /// Find a folder by path in children
    fn find_folder_in_children(&self, folder: &ProjectFile, path: &str) -> Option<usize> {
        // Check current folder
        if folder.path == path {
            return Some(0);
        }
        
        // Check children
        if matches!(folder.file_type, ProjectFileType::Folder) {
            for (i, child) in folder.children.iter().enumerate() {
                if child.path == path {
                    return Some(i);
                }
                
                if matches!(child.file_type, ProjectFileType::Folder) {
                    if let Some(_) = self.find_folder_in_children(child, path) {
                        return Some(i);
                    }
                }
            }
        }
        
        None
    }
    
    /// Delete a file or folder from the project
    fn delete_project_file(&mut self, path: &str) {
        let path = path.to_string();
        
        // Check if it's a root file
        self.project_files.retain(|file| file.path != path);
        
        // Clone project files to avoid borrow issues
        let project_files_clone = self.project_files.clone();
        
        // Check if it's a child file and update project files
        for (i, file) in project_files_clone.iter().enumerate() {
            if i < self.project_files.len() {
                // Update in-place with modified children
                self.project_files[i].children = remove_from_children(file.children.clone(), &path);
            }
        }
    }
    
    /// Rename a project file
    fn rename_project_file(&mut self) {
        if self.rename_file_new_name.is_empty() {
            return;
        }
        
        // Get the file to rename
        let file_path = self.rename_file_path.clone();
        let new_name = self.rename_file_new_name.clone();
        
        // Flag to track if we've found and renamed the file
        let mut renamed = false;
        
        // Check root files first
        for file in &mut self.project_files {
            if file.path == file_path {
                // Update the file name
                file.name = new_name.clone();
                
                // Update the path
                if let Some(parent_path) = &file.parent_path {
                    if parent_path == "/" {
                        file.path = format!("/{}", new_name);
                    } else {
                        file.path = format!("{}/{}", parent_path, new_name);
                    }
                }
                
                self.log_info(&format!("Renamed file to: {}", new_name));
                renamed = true;
                break;
            }
        }
        
        // If not renamed yet, check children
        if !renamed {
            // Clone project files to avoid borrow issues
            let project_files_clone = self.project_files.clone();
            
            for (i, file) in project_files_clone.iter().enumerate() {
                if i < self.project_files.len() {
                    // Create new children with renamed file
                    let (new_children, was_renamed) = rename_in_children(
                        file.children.clone(), 
                        &file_path, 
                        &new_name
                    );
                    
                    if was_renamed {
                        // Update the children
                        self.project_files[i].children = new_children;
                        self.log_info(&format!("Renamed file to: {}", new_name));
                        renamed = true;
                        break;
                    }
                }
            }
        }
    }
    
    /// Render project console
    fn render_project_console(&mut self, ui: &mut Ui) {
        // Console log
        ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
            if self.console_logs.is_empty() {
                ui.label("No logs to display");
            } else {
                for log in &self.console_logs {
                    let color = match log.level {
                        LogLevel::Info => Color32::WHITE,
                        LogLevel::Warning => Color32::YELLOW,
                        LogLevel::Error => Color32::RED,
                    };
                    
                    let level_text = match log.level {
                        LogLevel::Info => "INFO",
                        LogLevel::Warning => "WARNING",
                        LogLevel::Error => "ERROR",
                    };
                    
                    ui.label(RichText::new(format!("{} [{}] {}", log.timestamp, level_text, log.message)).color(color));
                }
            }
        });
    }
    
    /// Render the console as a separate window
    fn render_console_window(&mut self, ctx: &Context) {
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
                            self.console_logs.clear();
                        }
                    });
                });
                
                ui.separator();
                
                // Console log
                ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                    if self.console_logs.is_empty() {
                        ui.label("No logs to display");
                    } else {
                        for log in &self.console_logs {
                            let color = match log.level {
                                LogLevel::Info => Color32::WHITE,
                                LogLevel::Warning => Color32::YELLOW,
                                LogLevel::Error => Color32::RED,
                            };
                            
                            let level_text = match log.level {
                                LogLevel::Info => "INFO",
                                LogLevel::Warning => "WARNING",
                                LogLevel::Error => "ERROR",
                            };
                            
                            ui.label(RichText::new(format!("{} [{}] {}", log.timestamp, level_text, log.message)).color(color));
                        }
                    }
                });
            });
    }
    
    /// Render the rename dialog
    fn render_rename_dialog(&mut self, ctx: &Context) {
        let mut open = true;
        Window::new("Rename")
            .collapsible(false)
            .resizable(false)
            .fixed_size(Vec2::new(300.0, 100.0))
            .open(&mut open)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.text_edit_singleline(&mut self.rename_file_new_name);
                });
                
                ui.separator();
                
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.show_rename_dialog = false;
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Rename").clicked() {
                            self.rename_project_file();
                            self.show_rename_dialog = false;
                        }
                    });
                });
            });
            
        if !open {
            self.show_rename_dialog = false;
        }
    }
    
    /// Add info log message
    fn log_info(&mut self, message: &str) {
        self.add_log(LogLevel::Info, message);
    }
    
    /// Add warning log message
    fn log_warning(&mut self, message: &str) {
        self.add_log(LogLevel::Warning, message);
    }
    
    /// Add error log message
    fn log_error(&mut self, message: &str) {
        self.add_log(LogLevel::Error, message);
    }
    
    /// Add log message
    fn add_log(&mut self, level: LogLevel, message: &str) {
        use chrono::Local;
        let now = Local::now();
        let timestamp = now.format("%H:%M:%S").to_string();
        
        self.console_logs.push(ConsoleLog {
            timestamp,
            level,
            message: message.to_string(),
        });
    }
    
    /// Create a new scene
    fn new_scene(&mut self) {
        self.selected_entity = None;
        self.hierarchy_expanded.clear();
        self.entity_names.clear();
        self.log_info("Created new scene");
    }
    
    /// Create an empty entity
    fn create_empty_entity(&mut self) {
        let entity_id = if self.entity_names.is_empty() { 1 } else { self.entity_names.keys().max().unwrap() + 1 };
        self.entity_names.insert(entity_id, format!("New Entity {}", entity_id));
        self.entity_transforms.insert(entity_id, EntityTransform {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        });
        self.selected_entity = Some(entity_id);
        self.log_info(&format!("Created empty entity: New Entity {}", entity_id));
    }
    
    /// Create a sprite entity
    fn create_sprite_entity(&mut self) {
        let entity_id = if self.entity_names.is_empty() { 1 } else { self.entity_names.keys().max().unwrap() + 1 };
        self.entity_names.insert(entity_id, format!("New Sprite {}", entity_id));
        self.entity_transforms.insert(entity_id, EntityTransform {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        });
        self.selected_entity = Some(entity_id);
        self.log_info(&format!("Created sprite entity: New Sprite {}", entity_id));
    }
    
    /// Check if a path exists in the project
    fn path_exists(&self, path: &str) -> bool {
        // Check root files
        for file in &self.project_files {
            if file.path == path {
                return true;
            }
            
            // Check in children
            if self.path_exists_in_children(file, path) {
                return true;
            }
        }
        
        false
    }
    
    /// Check if a path exists in children
    fn path_exists_in_children(&self, folder: &ProjectFile, path: &str) -> bool {
        if matches!(folder.file_type, ProjectFileType::Folder) {
            // Check direct children
            for child in &folder.children {
                if child.path == path {
                    return true;
                }
                
                // Check deeper
                if matches!(child.file_type, ProjectFileType::Folder) {
                    if self.path_exists_in_children(child, path) {
                        return true;
                    }
                }
            }
        }
        
        false
    }
}

/// Helper function to remove a file from children (avoids borrow issues)
fn remove_from_children(children: Vec<ProjectFile>, path: &str) -> Vec<ProjectFile> {
    let mut result = Vec::new();
    
    for mut file in children {
        // Skip this file if it matches the path
        if file.path == path {
            continue;
        }
        
        // Process children if it's a folder
        if matches!(file.file_type, ProjectFileType::Folder) {
            file.children = remove_from_children(file.children, path);
        }
        
        result.push(file);
    }
    
    result
}

/// Helper function to rename a file in children (avoids borrow issues)
/// Returns (new children, whether rename happened)
fn rename_in_children(children: Vec<ProjectFile>, path: &str, new_name: &str) -> (Vec<ProjectFile>, bool) {
    let mut result = Vec::new();
    let mut renamed = false;
    
    for mut file in children {
        // Check if this is the file to rename
        if file.path == path {
            // Update the file name
            file.name = new_name.to_string();
            
            // Update the path
            if let Some(parent_path) = &file.parent_path {
                if parent_path == "/" {
                    file.path = format!("/{}", new_name);
                } else {
                    file.path = format!("{}/{}", parent_path, new_name);
                }
                
                // If it's a folder, update children's parent_path
                if matches!(file.file_type, ProjectFileType::Folder) {
                    let old_path = path.to_string();
                    for child in &mut file.children {
                        if let Some(ref child_parent) = child.parent_path {
                            if *child_parent == old_path {
                                child.parent_path = Some(file.path.clone());
                            }
                        }
                    }
                }
            }
            
            result.push(file);
            renamed = true;
        } else {
            // Process children if it's a folder
            if matches!(file.file_type, ProjectFileType::Folder) {
                let (new_children, was_renamed) = rename_in_children(file.children, path, new_name);
                file.children = new_children;
                
                if was_renamed {
                    renamed = true;
                }
            }
            
            result.push(file);
        }
    }
    
    (result, renamed)
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