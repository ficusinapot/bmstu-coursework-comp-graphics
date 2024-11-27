use egui::Color32;
use log::debug;

use crate::facade::Command;
use crate::managers::ManagerSolution;
use crate::object::Component;
use crate::object::objects::texture3d::WorleyBuilder;

pub enum SceneCommand {
    AddObject(&'static str, Component),
    GetObject(Component),
    RemoveObject(Component),

    SetNumSteps(&'static str, usize),
    SetNumStepsLight(&'static str, usize),
    SetCloudScale(&'static str, f32),
    SetDensityMultiplier(&'static str, f32),
    SetDensityThreshold(&'static str, f32),
    SetDensityOffset(&'static str, f32),
    SetOffset(&'static str, glam::Vec3),
    SetAlphaThreshold(&'static str, u8),

    SetNoise(&'static str, WorleyBuilder),
    SetDetailNoise(&'static str, WorleyBuilder),

    SetDetailNoiseScale(&'static str, f32),
    SetDetailNoiseWeight(&'static str, f32),
    SetDetailWeights(&'static str, glam::Vec4),
    SetShapeNoiseWeights(&'static str, glam::Vec4),
    SetPhaseParams(&'static str, glam::Vec4),
    SetShapeOffset(&'static str, glam::Vec3),
    SetDetailOffset(&'static str, glam::Vec3),

    SetLightAbsorptionTowardSun(&'static str, f32),
    SetLightAbsorptionThroughCloud(&'static str, f32),
    SetDarknessThreshold(&'static str, f32),
    SetRayOffsetStrength(&'static str, f32),
    SetLightColor(&'static str, Color32),
    SetColA(&'static str, Color32),
    SetColB(&'static str, Color32),

    SetHeightMapFactor(&'static str, f32),
    SetVolumeOffset(&'static str, f32),
    SetEdgeDistance(&'static str, f32),

    SetSunDistance(&'static str, f32),
    SetSunAngle(&'static str, f32),
}

impl Command for SceneCommand {
    type ReturnType = ();
    fn exec(self, manager: &mut ManagerSolution) {
        match self {
            SceneCommand::AddObject(name, component) => {
                let sm = manager.get_mut_scene_manager();
                sm.add_object(name, component);
            }
            SceneCommand::GetObject(_component) => {
                debug!("get object");
            }
            SceneCommand::RemoveObject(_component) => {
                debug!("remove object");
            }
            SceneCommand::SetNumSteps(id, num_steps) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            cloud.num_steps = num_steps;
                        }
                    }
                }
            }
            SceneCommand::SetNumStepsLight(id, num_steps) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.num_steps_light = num_steps;
                            }
                        }
                    }
                }
            }
            SceneCommand::SetCloudScale(id, cloud_scale) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.cloud_scale = cloud_scale
                            }
                        }
                    }
                }
            }
            SceneCommand::SetRayOffsetStrength(id, ray_offset_strength) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.ray_offset_strength = ray_offset_strength
                            }
                        }
                    }
                }
            }
            SceneCommand::SetDensityMultiplier(id, density_multiplier) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.density_multiplier = density_multiplier
                            }
                        }
                    }
                }
            }
            SceneCommand::SetDensityThreshold(id, density_threshold) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.density_threshold = density_threshold
                            }
                        }
                    }
                }
            }
            SceneCommand::SetDensityOffset(id, d) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.density_offset = d
                            }
                        }
                    }
                }
            }
            SceneCommand::SetOffset(id, offset) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.offset = offset
                            }
                        }
                    }
                }
            }
            SceneCommand::SetAlphaThreshold(id, threshold) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.alpha_threshold = threshold
                            }
                        }
                    }
                }
            }
            SceneCommand::SetNoise(id, noise) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.regenerate_noise(noise)
                            }
                        }
                    }
                }
            }
            SceneCommand::SetDetailNoise(id, noise) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.regenerate_detail_noise(noise)
                            }
                        }
                    }
                }
            }
            SceneCommand::SetDetailNoiseScale(id, detail_noise_scale) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.detail_noise_scale = detail_noise_scale
                            }
                        }
                    }
                }
            }
            SceneCommand::SetDetailNoiseWeight(id, detail_noise_weight) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.detail_noise_weight = detail_noise_weight
                            }
                        }
                    }
                }
            }
            SceneCommand::SetDetailWeights(id, detail_weights) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.detail_weights = detail_weights
                            }
                        }
                    }
                }
            }
            SceneCommand::SetShapeNoiseWeights(id, shape_noise_weights) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.shape_noise_weights = shape_noise_weights
                            }
                        }
                    }
                }
            }
            SceneCommand::SetPhaseParams(id, phase_params) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.phase_params = phase_params
                            }
                        }
                    }
                }
            }
            SceneCommand::SetShapeOffset(id, shape_offset) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.shape_offset = shape_offset
                            }
                        }
                    }
                }
            }
            SceneCommand::SetDetailOffset(id, detail_offset) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.detail_offset = detail_offset
                            }
                        }
                    }
                }
            }
            SceneCommand::SetLightAbsorptionTowardSun(id, light_absorption_toward_sun) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.light_absorption_toward_sun = light_absorption_toward_sun
                            }
                        }
                    }
                }
            }
            SceneCommand::SetLightAbsorptionThroughCloud(id, light_absorption_through_cloud) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.light_absorption_through_cloud =
                                    light_absorption_through_cloud
                            }
                        }
                    }
                }
            }
            SceneCommand::SetDarknessThreshold(id, darkness_threshold) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.darkness_threshold = darkness_threshold
                            }
                        }
                    }
                }
            }
            SceneCommand::SetLightColor(id, light_color) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.light_color = light_color
                            }
                        }
                    }
                }
            }
            SceneCommand::SetColA(id, col_a) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.col_a = col_a
                            }
                        }
                    }
                }
            }
            SceneCommand::SetColB(id, col_b) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.col_b = col_b
                            }
                        }
                    }
                }
            }
            SceneCommand::SetEdgeDistance(id, ed) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.edge_distance = ed
                            }
                        }
                    }
                }
            }
            SceneCommand::SetVolumeOffset(id, vo) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.volume_offset = vo
                            }
                        }
                    }
                }
            }
            SceneCommand::SetHeightMapFactor(id, hmf) => {
                if let Some(comp) = manager.get_mut_scene_manager().get_mut_object(id) {
                    for i in comp {
                        if let Component::Cloud(cloud) = i {
                            {
                                cloud.height_map_factor = hmf
                            }
                        }
                    }
                }
            }
            SceneCommand::SetSunDistance(id, d) => {
                if let Some(Component::Sun(sun)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    sun.set_d(d);
                    debug!("sun pos {:?}", sun.get_pos());
                }
            }
            SceneCommand::SetSunAngle(id, a) => {
                if let Some(Component::Sun(sun)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    sun.prepend_angle(a);
                    debug!("sun pos {:?}", sun.get_pos());
                }
            }
        }
    }
}
