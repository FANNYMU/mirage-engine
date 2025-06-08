use egui::{Context, Ui, ScrollArea, RichText, Color32};
use crate::ui::editor::ui_components::{ProjectFile, ProjectFileType, ProjectTab};

/// Project panel for managing project files
pub struct ProjectPanel {
    /// Active tab for project panel
    pub project_active_tab: ProjectTab,
    /// Project files
    pub project_files: Vec<ProjectFile>,
    /// Current project path
    pub current_project_path: Option<String>,
    /// Show rename dialog
    pub show_rename_dialog: bool,
    /// Rename file path
    pub rename_file_path: String,
    /// Rename file new name
    pub rename_file_new_name: String,
}

impl ProjectPanel {
    /// Create a new project panel
    pub fn new() -> Self {
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
                        path: "/Scripts/Player.lua".to_string(),
                        children: vec![],
                        expanded: false,
                        parent_path: Some("/Scripts".to_string()),
                    },
                    ProjectFile {
                        name: "Enemy.rs".to_string(),
                        file_type: ProjectFileType::Script,
                        path: "/Scripts/Enemy.lua".to_string(),
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
                        name: "player.png".to_string(),
                        file_type: ProjectFileType::Texture,
                        path: "/Textures/player.png".to_string(),
                        children: vec![],
                        expanded: false,
                        parent_path: Some("/Textures".to_string()),
                    },
                    ProjectFile {
                        name: "enemy.png".to_string(),
                        file_type: ProjectFileType::Texture,
                        path: "/Textures/enemy.png".to_string(),
                        children: vec![],
                        expanded: false,
                        parent_path: Some("/Textures".to_string()),
                    },
                    ProjectFile {
                        name: "background.png".to_string(),
                        file_type: ProjectFileType::Texture,
                        path: "/Textures/background.png".to_string(),
                        children: vec![],
                        expanded: false,
                        parent_path: Some("/Textures".to_string()),
                    },
                ],
                expanded: false,
                parent_path: Some("/".to_string()),
            },
            ProjectFile {
                name: "Audio".to_string(),
                file_type: ProjectFileType::Folder,
                path: "/Audio".to_string(),
                children: vec![
                    ProjectFile {
                        name: "music.mp3".to_string(),
                        file_type: ProjectFileType::Audio,
                        path: "/Audio/music.mp3".to_string(),
                        children: vec![],
                        expanded: false,
                        parent_path: Some("/Audio".to_string()),
                    },
                    ProjectFile {
                        name: "explosion.wav".to_string(),
                        file_type: ProjectFileType::Audio,
                        path: "/Audio/explosion.wav".to_string(),
                        children: vec![],
                        expanded: false,
                        parent_path: Some("/Audio".to_string()),
                    },
                    ProjectFile {
                        name: "wind.ogg".to_string(),
                        file_type: ProjectFileType::Audio,
                        path: "/Audio/wind.ogg".to_string(),
                        children: vec![],
                        expanded: false,
                        parent_path: Some("/Audio".to_string()),
                    },
                ],
                expanded: false,
                parent_path: Some("/".to_string()),
            },
        ];
        
        Self {
            project_active_tab: ProjectTab::Files,
            project_files,
            current_project_path: Some("/".to_string()),
            show_rename_dialog: false,
            rename_file_path: String::new(),
            rename_file_new_name: String::new(),
        }
    }
    
    /// Render the project panel
    pub fn render(&mut self, ctx: &Context, log_info: &mut dyn FnMut(&str)) {
        egui::SidePanel::right("project_panel")
            .resizable(true)
            .min_width(250.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.selectable_label(self.project_active_tab == ProjectTab::Files, "Files").clicked() {
                        self.project_active_tab = ProjectTab::Files;
                    }
                    if ui.selectable_label(self.project_active_tab == ProjectTab::Console, "Console").clicked() {
                        self.project_active_tab = ProjectTab::Console;
                    }
                    if ui.selectable_label(self.project_active_tab == ProjectTab::Audio, "Audio").clicked() {
                        self.project_active_tab = ProjectTab::Audio;
                    }
                });
                
                ui.separator();
                
                // This will be handled by the main EditorUI to route to the appropriate panel
            });
            
        // Render rename dialog if needed
        self.render_rename_dialog(ctx, log_info);
    }
    
    /// Render project files tab
    pub fn render_project_files(&mut self, ui: &mut Ui, log_info: &mut dyn FnMut(&str)) {
        ui.horizontal(|ui| {
            ui.heading("Files");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("New Folder").clicked() {
                    if let Some(path) = &self.current_project_path {
                        self.create_project_folder(path.clone(), "New Folder".to_string());
                        log_info("Created new folder");
                    }
                }
                if ui.button("New Script").clicked() {
                    if let Some(path) = &self.current_project_path {
                        self.create_project_file(
                            path.clone(),
                            "NewScript.rs".to_string(),
                            ProjectFileType::Script,
                        );
                        log_info("Created new script");
                    }
                }
            });
        });
        
        ui.separator();
        
        ScrollArea::vertical().show(ui, |ui| {
            // Clone the project files to avoid borrow issues
            let project_files_clone = self.project_files.clone();
            for (i, file) in project_files_clone.iter().enumerate() {
                self.render_file(ui, i, file, 0, log_info);
            }
        });
    }
    
    /// Render a file in the project panel
    fn render_file(&mut self, ui: &mut Ui, _file_index: usize, file: &ProjectFile, depth: usize, log_info: &mut dyn FnMut(&str)) {
        // Convert depth to float for UI spacing
        let indent = (depth as f32) * 20.0;
        
        ui.horizontal(|ui| {
            ui.add_space(indent);
            
            if file.file_type == ProjectFileType::Folder {
                let expanded = file.expanded;
                let text = if expanded { "▼" } else { "►" };
                if ui.button(text).clicked() {
                    // Toggle expanded state
                    if let Some(file) = find_file_mut(&mut self.project_files, &file.path) {
                        file.expanded = !file.expanded;
                    }
                }
            } else {
                ui.add_space(15.0);
            }
            
            self.render_project_file_icon(ui, file);
            
            if ui.label(RichText::new(&file.name).strong()).clicked() {
                // Set current path when clicking on a folder
                if file.file_type == ProjectFileType::Folder {
                    self.current_project_path = Some(file.path.clone());
                    log_info(&format!("Selected folder: {}", file.path));
                } else {
                    log_info(&format!("Selected file: {}", file.path));
                }
            }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Context menu - implemented as buttons for simplicity
                if ui.button("✖").clicked() {
                    self.delete_project_file(&file.path);
                    log_info(&format!("Deleted: {}", file.path));
                }
                if ui.button("✏️").clicked() {
                    self.show_rename_dialog = true;
                    self.rename_file_path = file.path.clone();
                    self.rename_file_new_name = file.name.clone();
                }
            });
        });
        
        if file.file_type == ProjectFileType::Folder && file.expanded {
            // Clone children to avoid borrow issues
            let children_clone: Vec<ProjectFile> = if let Some(file) = find_file(&self.project_files, &file.path) {
                file.children.clone()
            } else {
                vec![]
            };
            
            for (i, child) in children_clone.iter().enumerate() {
                self.render_file(ui, i, child, depth + 1, log_info);
            }
        }
    }
    
    /// Render a project file icon
    fn render_project_file_icon(&mut self, ui: &mut Ui, file: &ProjectFile) {
        let icon = match file.file_type {
            ProjectFileType::Folder => "📁",
            ProjectFileType::Scene => "🎬",
            ProjectFileType::Script => "📝",
            ProjectFileType::Texture => "🖼️",
            ProjectFileType::Audio => "🔊",
            ProjectFileType::Other => "📄",
        };
        
        let color = match file.file_type {
            ProjectFileType::Folder => Color32::from_rgb(255, 223, 0),
            ProjectFileType::Scene => Color32::from_rgb(0, 191, 255),
            ProjectFileType::Script => Color32::from_rgb(0, 255, 127),
            ProjectFileType::Texture => Color32::from_rgb(255, 105, 180),
            ProjectFileType::Audio => Color32::from_rgb(138, 43, 226),
            ProjectFileType::Other => Color32::WHITE,
        };
        
        ui.label(RichText::new(icon).color(color));
    }
    
    /// Render rename dialog
    pub fn render_rename_dialog(&mut self, ctx: &Context, log_info: &mut dyn FnMut(&str)) {
        if self.show_rename_dialog {
            egui::Window::new("Rename File")
                .fixed_size([300.0, 100.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("New name:");
                        ui.text_edit_singleline(&mut self.rename_file_new_name);
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_rename_dialog = false;
                        }
                        
                        if ui.button("Rename").clicked() {
                            self.rename_project_file();
                            self.show_rename_dialog = false;
                            log_info(&format!("Renamed file to: {}", self.rename_file_new_name));
                        }
                    });
                });
        }
    }
    
    /// Get files in a directory
    pub fn get_files_in_directory(&self, directory_path: &str) -> Vec<&ProjectFile> {
        let mut result = Vec::new();
        
        for file in &self.project_files {
            self.collect_files_in_directory(file, directory_path, &mut result);
        }
        
        result
    }
    
    /// Collect files in a directory
    fn collect_files_in_directory<'a>(
        &'a self,
        file: &'a ProjectFile,
        directory_path: &str,
        result: &mut Vec<&'a ProjectFile>,
    ) {
        if file.path == directory_path && file.file_type == ProjectFileType::Folder {
            for child in &file.children {
                result.push(child);
            }
        } else if file.file_type == ProjectFileType::Folder {
            for child in &file.children {
                self.collect_files_in_directory(child, directory_path, result);
            }
        }
    }
    
    /// Create a project folder
    pub fn create_project_folder(&mut self, parent_path: String, folder_name: String) {
        // Ensure folder name is unique
        let folder_name_clone = folder_name.clone();
        let mut unique_name = folder_name_clone;
        let mut counter = 1;
        
        // Keep incrementing counter until we find a unique name
        while self.path_exists(&format!("{}/{}", parent_path, unique_name)) {
            unique_name = format!("{} ({})", folder_name, counter);
            counter += 1;
        }
        
        // Create the folder path
        let path = if parent_path == "/" {
            format!("/{}", unique_name)
        } else {
            format!("{}/{}", parent_path, unique_name)
        };
        
        // Create the folder
        let folder = ProjectFile {
            name: unique_name,
            file_type: ProjectFileType::Folder,
            path: path.clone(),
            children: Vec::new(),
            expanded: false,
            parent_path: Some(parent_path.clone()),
        };
        
        // Add to parent
        if parent_path == "/" {
            self.project_files.push(folder);
        } else {
            self.add_file_to_parent(&parent_path, folder);
        }
    }
    
    /// Create a project file
    pub fn create_project_file(
        &mut self,
        parent_path: String,
        file_name: String,
        file_type: ProjectFileType,
    ) {
        // Ensure file name is unique
        let file_name_clone = file_name.clone();
        let mut unique_name = file_name_clone;
        let mut counter = 1;
        
        while self.path_exists(&format!("{}/{}", parent_path, unique_name)) {
            // For files with extensions, insert counter before extension
            if unique_name.contains('.') {
                let file_name_clone2 = file_name.clone();
                let parts: Vec<&str> = file_name_clone2.split('.').collect();
                let name = parts[0];
                let ext = if parts.len() > 1 {
                    parts[1..].join(".")
                } else {
                    String::new()
                };
                
                if !ext.is_empty() {
                    unique_name = format!("{} ({})", name, counter);
                    if !ext.is_empty() {
                        unique_name = format!("{}.{}", unique_name, ext);
                    }
                } else {
                    unique_name = format!("{} ({})", unique_name, counter);
                }
            } else {
                unique_name = format!("{} ({})", unique_name, counter);
            }
            counter += 1;
        }
        
        // Create the file
        let path = if parent_path == "/" {
            format!("/{}", unique_name)
        } else {
            format!("{}/{}", parent_path, unique_name)
        };
        
        let file = ProjectFile {
            name: unique_name,
            file_type,
            path: path.clone(),
            children: Vec::new(),
            expanded: false,
            parent_path: Some(parent_path.clone()),
        };
        
        // Add to parent
        if parent_path == "/" {
            self.project_files.push(file);
        } else {
            self.add_file_to_parent(&parent_path, file);
        }
    }
    
    /// Add a file to its parent
    fn add_file_to_parent(&mut self, parent_path: &str, file: ProjectFile) {
        // Cek dulu apakah parent path ada di root
        for project_file in &mut self.project_files {
            if project_file.path == parent_path {
                project_file.children.push(file);
                return;
            }
        }
        
        // Jika tidak ada di root, cari di children
        // Pertama, identifikasi file mana yang berisi parent_path
        let mut parent_index = None;
        let mut child_index = None;
        
        // Cari indeks file yang mengandung parent_path
        for (i, project_file) in self.project_files.iter().enumerate() {
            if let Some((p_idx, c_idx)) = find_folder_path_indices(&project_file.children, parent_path) {
                parent_index = Some(i);
                child_index = Some((p_idx, c_idx));
                break;
            }
        }
        
        // Jika ditemukan, tambahkan file ke children
        if let Some(p_idx) = parent_index {
            if let Some((parent_idx, child_idx)) = child_index {
                if parent_idx == 0 {
                    // Parent langsung ada di children
                    self.project_files[p_idx].children[child_idx].children.push(file);
                } else {
                    // Parent ada di nested children, gunakan indeks yang ditemukan
                    self.project_files[p_idx].children[parent_idx].children[child_idx].children.push(file);
                }
            }
        }
    }
    
    /// Delete a project file
    pub fn delete_project_file(&mut self, path: &str) {
        // Remove from root if it's a root file
        self.project_files = self.project_files
            .iter()
            .filter(|file| file.path != path)
            .cloned()
            .collect();
        
        // Remove from children
        for file in &mut self.project_files {
            file.children = remove_from_children(file.children.clone(), path);
        }
    }
    
    /// Rename a project file
    pub fn rename_project_file(&mut self) {
        let path_to_rename = self.rename_file_path.clone();
        let new_name = self.rename_file_new_name.clone();
        let mut renamed = false;
        
        // Try to rename at root level
        for i in 0..self.project_files.len() {
            if self.project_files[i].path == path_to_rename {
                let old_path = self.project_files[i].path.clone();
                let parent_path = self.project_files[i].parent_path.clone().unwrap_or_else(|| "/".to_string());
                
                // Update the path
                if parent_path == "/" {
                    self.project_files[i].path = format!("/{}", new_name);
                } else {
                    self.project_files[i].path = format!("{}/{}", parent_path, new_name);
                }
                
                // Update name
                self.project_files[i].name = new_name.clone();
                
                // Update children paths
                let new_path = self.project_files[i].path.clone();
                
                // Clone the file to avoid borrow issues
                let mut file_clone = self.project_files[i].clone();
                update_children_paths_helper(&mut file_clone, &old_path, &new_path);
                self.project_files[i] = file_clone;
                
                renamed = true;
                break;
            }
        }
        
        // If not renamed yet, try in children
        if !renamed {
            for i in 0..self.project_files.len() {
                let (new_children, success) = rename_in_children(
                    self.project_files[i].children.clone(),
                    &path_to_rename,
                    &new_name,
                );
                
                if success {
                    self.project_files[i].children = new_children;
                    renamed = true;
                    break;
                }
            }
        }
    }
    
    /// Update children paths after a parent is renamed
    fn update_children_paths(&mut self, file: &mut ProjectFile, old_parent: &str, new_parent: &str) {
        // Create a new vector for updated children
        let mut updated_children = Vec::new();
        
        // Process each child
        for mut child in std::mem::take(&mut file.children) {
            // Update the path and parent_path
            child.path = child.path.replace(old_parent, new_parent);
            child.parent_path = Some(new_parent.to_string());
            
            // Recursively update paths for this child's children
            update_children_paths_helper(&mut child, old_parent, new_parent);
            
            // Add the updated child to our new vector
            updated_children.push(child);
        }
        
        // Replace the original children with our updated ones
        file.children = updated_children;
    }
    
    /// Check if a path exists
    pub fn path_exists(&self, path: &str) -> bool {
        // Check in root files
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
        for child in &folder.children {
            if child.path == path {
                return true;
            }
            
            if child.file_type == ProjectFileType::Folder {
                if self.path_exists_in_children(child, path) {
                    return true;
                }
            }
        }
        
        false
    }
}

/// Helper function to remove a file from children
pub fn remove_from_children(children: Vec<ProjectFile>, path: &str) -> Vec<ProjectFile> {
    let mut result = Vec::new();
    
    for child in children {
        if child.path != path {
            let mut new_child = child.clone();
            if !new_child.children.is_empty() {
                new_child.children = remove_from_children(new_child.children, path);
            }
            result.push(new_child);
        }
    }
    
    result
}

/// Helper function to rename a file in children
pub fn rename_in_children(
    children: Vec<ProjectFile>,
    path: &str,
    new_name: &str,
) -> (Vec<ProjectFile>, bool) {
    let mut result = Vec::new();
    let mut renamed = false;
    
    for child in children {
        if child.path == path {
            let mut new_child = child.clone();
            let old_path = new_child.path.clone();
            let parent_path = new_child.parent_path.clone().unwrap_or_else(|| "/".to_string());
            
            // Update the path
            if parent_path == "/" {
                new_child.path = format!("/{}", new_name);
            } else {
                new_child.path = format!("{}/{}", parent_path, new_name);
            }
            
            // Update name
            new_child.name = new_name.to_string();
            
            // Update children paths (manually for now)
            for grandchild in &mut new_child.children {
                grandchild.path = grandchild.path.replace(&old_path, &new_child.path);
                grandchild.parent_path = Some(new_child.path.clone());
            }
            
            result.push(new_child);
            renamed = true;
        } else {
            let mut new_child = child.clone();
            if !new_child.children.is_empty() {
                let (new_children, success) = rename_in_children(new_child.children, path, new_name);
                new_child.children = new_children;
                if success {
                    renamed = true;
                }
            }
            result.push(new_child);
        }
    }
    
    (result, renamed)
}

/// Helper function to find a file in a project files vector by path
fn find_file<'a>(files: &'a [ProjectFile], path: &str) -> Option<&'a ProjectFile> {
    for file in files {
        if file.path == path {
            return Some(file);
        }
        
        if let Some(found) = find_file(&file.children, path) {
            return Some(found);
        }
    }
    
    None
}

/// Helper function to find a mutable file in a project files vector by path
fn find_file_mut<'a>(files: &'a mut [ProjectFile], path: &str) -> Option<&'a mut ProjectFile> {
    for file in files {
        if file.path == path {
            return Some(file);
        }
        
        if let Some(found) = find_file_mut(&mut file.children, path) {
            return Some(found);
        }
    }
    
    None
}

/// Helper function to update children paths without self reference
pub fn update_children_paths_helper(file: &mut ProjectFile, old_parent: &str, new_parent: &str) {
    // Create a new vector for updated children
    let mut updated_children = Vec::new();
    
    // Process each child
    for mut child in std::mem::take(&mut file.children) {
        // Update the path and parent_path
        child.path = child.path.replace(old_parent, new_parent);
        child.parent_path = Some(new_parent.to_string());
        
        // Recursively update paths for this child's children
        update_children_paths_helper(&mut child, old_parent, new_parent);
        
        // Add the updated child to our new vector
        updated_children.push(child);
    }
    
    // Replace the original children with our updated ones
    file.children = updated_children;
}

/// Helper function to find folder path indices
pub fn find_folder_path_indices(children: &[ProjectFile], path: &str) -> Option<(usize, usize)> {
    // Cek langsung di children
    for (i, child) in children.iter().enumerate() {
        if child.path == path && child.file_type == ProjectFileType::Folder {
            return Some((0, i)); // 0 berarti parent langsung
        }
        
        // Cek di children dari children
        if child.file_type == ProjectFileType::Folder {
            for (j, grandchild) in child.children.iter().enumerate() {
                if grandchild.path == path && grandchild.file_type == ProjectFileType::Folder {
                    return Some((i, j)); // i adalah indeks parent, j adalah indeks child
                }
            }
        }
    }
    
    None
} 