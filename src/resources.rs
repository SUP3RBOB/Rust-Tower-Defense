use bevy::prelude::*;

#[derive(Resource)]
pub struct Images {
    pub tower1: Handle<Image>,
    pub tower2: Handle<Image>,   
    pub bullet: Handle<Image>,   
    pub path: Handle<Image>,
    pub range_view: Handle<Image>,
    pub square: Handle<Image>,
}

pub struct ImagesPlugin;
impl Plugin for ImagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets);
    }
}

fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(Images {
        tower1: assets.load("sprites/tower1.png"),
        tower2: assets.load("sprites/tower2.png"),
        bullet: assets.load("sprites/bullet.png"),
        path: assets.load("sprites/path.png"),
        range_view: assets.load("sprites/range_view.png"),
        square: assets.load("sprites/square.png"),
    });
}
