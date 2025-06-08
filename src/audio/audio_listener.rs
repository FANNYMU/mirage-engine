use glam::Vec3;

/// Struktur yang merepresentasikan pendengar audio untuk audio 3D
#[derive(Debug, Clone, Copy)]
pub struct AudioListener {
    /// Posisi pendengar dalam ruang 3D
    pub position: Vec3,
    
    /// Arah hadap pendengar
    pub forward: Vec3,
}

impl AudioListener {
    /// Membuat instance baru AudioListener
    pub fn new(position: Vec3, forward: Vec3) -> Self {
        Self {
            position,
            forward: forward.normalize(),
        }
    }
    
    /// Mengatur posisi pendengar
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
    
    /// Mengatur arah hadap pendengar
    pub fn set_forward(&mut self, forward: Vec3) {
        self.forward = forward.normalize();
    }
    
    /// Menghitung jarak ke posisi target
    pub fn distance_to(&self, position: Vec3) -> f32 {
        self.position.distance(position)
    }
} 