use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
//use heron::CollisionEvent;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::geometry::Group;
use std::f32::consts::PI;
use bevy_mod_picking::*;

//cargo build --release --target wasm32-unknown-unknown
//wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/sweet_land.wasm

#[derive(Component)]
struct AnimateTile;

#[derive(Component)]
struct AnimatePusher {
    rotate: f32,
}

#[derive(Component)]
struct AnimateReceiver;

#[derive(Component)]
struct AnimateArm1 {
    angle: f32,
    flag: u32,
}

#[derive(Component)]
struct AnimateArm2 {
    angle: f32,
    flag: u32,
}

#[derive(Component)]
struct AnimateArm3 {
    angle: f32,
    flag: u32,
}

#[derive(Component)]
struct AnimateItem;



#[derive(Component)]
struct CameraText;

#[derive(Component)]
struct PointText {
    point: u32,
}

#[derive(Component)]
struct SelectButton1 {
    flag: bool,
    act: u32,
}

#[derive(Component)]
struct SelectButton2 {
    flag: bool,
    act: u32
}

#[derive(Component)]
struct AnimateTimer(Timer);

const ROUND_TABLE_RADIUS: f32 = 20.0;
const ROUND_TABLE_HEIGHT: f32 = 2.0;
const ARM_SPEED: f32 = 30.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        //.add_system(print_ball_altitude)
        .add_system(move_camera)
        .add_system(animate_tile)
        .add_system(animate_pusher)
        .add_system(update_camera_text)
        .add_system(select_button1)
        .add_system(select_button2)
        .add_system(move_button1)
        .add_system(move_button2)
        .add_system(action_button1)
        .add_system(action_button2)
        .add_system(rotate_arm1)
        .add_system(rotate_arm2)
        .add_system(rotate_arm3)
        .add_system(reciever_event)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_graphics(
    mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //カメラ
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(12.0, 15.0, 45.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(PickingCameraBundle::default());

    //環境光
    commands.insert_resource(AmbientLight {
        color: Color::LIME_GREEN,
        brightness: 0.5,
    });

    //サンライト
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 5.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    
    //床
    commands
        .spawn(Collider::cuboid(20.0, 2.0, 20.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.5, 0.0)));
    

    //中央のシリンダー
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 10.0,
                height: 20.0,
                resolution: 32,
                segments: 32,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::GREEN,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 11.0, 0.0),
            ..default()
        },
    ))
    .insert(Collider::cylinder(5.0, 10.0));


    commands.spawn(SceneBundle {
        scene: asset_server.load("models/floor.gltf#Scene0"),
        transform: Transform::from_xyz(21.0, -15.0, 20.0)
                * Transform::default().with_scale(Vec3::new(4.1, 4.0, 4.1))
                //* Transform::from_rotation(Quat::from_rotation_x(PI / 2.0))
                //* Transform::from_rotation(Quat::from_rotation_z(PI / -2.0))
                ,
        ..default()
    });

    //プッシャー1
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(8.0, 1.0, 10.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..default()
            }),
            transform: Transform::from_xyz(-1.0, 4.0, 15.0),
            ..default()
        },
    ))
    .insert(Collider::cuboid(4.0, 0.5, 5.0));

    //プッシャー2
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(8.0, 1.0, 10.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                ..default()
            }),
            transform: Transform::from_xyz(-1.0, 5.0, 10.0),
            ..default()
        },
        AnimatePusher { rotate: 0.0 }
    ))
    .insert(Collider::cuboid(4.0, 0.5, 5.0));

    //受け取り口
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(8.0, 1.0, 6.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::PURPLE,
                ..default()
            }),
            transform: Transform::from_xyz(-1.0, 1.0, 24.0),
            ..default()
        },
        AnimateReceiver,
    ))
    .insert(Collider::cuboid(4.0, 0.5, 2.5))
    .insert(ActiveEvents::COLLISION_EVENTS)
    .with_children(|parent| {
        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(9.0, 5.0, 1.0))),
                transform: Transform::from_xyz(0.0, 0.0, -3.0),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            },
        ))
        .insert(Collider::cuboid(4.5, 2.5, 0.5));

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 5.0, 5.0))),
                transform: Transform::from_xyz(-4.0, 0.0, 0.0),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            },
        ))
        .insert(Collider::cuboid(0.5, 2.5, 2.5));

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(9.0, 5.0, 1.0))),
                transform: Transform::from_xyz(0.0, 0.0, 3.0),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            },
        ))
        .insert(Collider::cuboid(4.5, 2.5, 0.5));

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 5.0, 5.0))),
                transform: Transform::from_xyz(4.0, 0.0, 0.0),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            },
        ))
        .insert(Collider::cuboid(0.5, 2.5, 2.5));

    });

    //回転タイル
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: ROUND_TABLE_RADIUS,
                    height: ROUND_TABLE_HEIGHT,
                    resolution: 10,
                    segments: 10,
                })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        AnimateTile,
    ))
    .insert(RigidBody::Dynamic)
    .insert(Collider::cylinder(ROUND_TABLE_HEIGHT / 2.0 , ROUND_TABLE_RADIUS))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
    .insert(Friction {  //摩擦
        coefficient: 0.7,   //摩擦係数
        combine_rule: CoefficientCombineRule::Max,
    })
    .insert(Dominance::group(10))
    .with_children(|parent| {
        //床移動補助用回転バー1
        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 0.5, ROUND_TABLE_RADIUS*2.0))),
                transform: Transform::from_xyz(0.0, 1.5, 0.0),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            },
        ))
        //.insert(RigidBody::Dynamic)
        //.insert(Dominance::group(10))
        .insert(Collider::cuboid(0.25, 0.25, ROUND_TABLE_RADIUS))
        .insert(CollisionGroups::new(Group::from_bits_truncate(0b0010), Group::from_bits_truncate(0b0001)));

        //床移動補助用回転バー2
        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(ROUND_TABLE_RADIUS*2.0, 0.5, 0.5))),
                transform: Transform::from_xyz(0.0, 1.5, 0.0),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            },
        ))
        //.insert(RigidBody::Dynamic)
        //.insert(Dominance::group(10))
        .insert(Collider::cuboid(ROUND_TABLE_RADIUS, 0.25, 0.25))
        .insert(CollisionGroups::new(Group::from_bits_truncate(0b0010), Group::from_bits_truncate(0b0001)));

    })
    .insert(CollisionGroups::new(Group::from_bits_truncate(0b0010), Group::from_bits_truncate(0b0001)));




/*
    //グループ検証用
    let collision_groups1 = CollisionGroups::new(Group::from_bits_truncate(0b0110001), Group::from_bits_truncate(0b0010000));
    let collision_groups2 = CollisionGroups::new(Group::from_bits_truncate(0b0110001), Group::from_bits_truncate(0b0100000));
    let collision_groups3 = CollisionGroups::new(Group::from_bits_truncate(0b1000001), Group::from_bits_truncate(0b0010000));

    //1
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(2.0, 2.0, 2.0))),
            transform: Transform::from_xyz(0.0, 4.0, 15.0),
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        },
    ))
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(1.0, 1.0, 1.0))
    .insert(collision_groups1);

    //2
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(2.0, 2.0, 2.0))),
            transform: Transform::from_xyz(0.0, 6.0, 15.0),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        },
    ))
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(1.0, 1.0, 1.0))
    .insert(collision_groups2);


    //3
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(2.0, 2.0, 2.0))),
            transform: Transform::from_xyz(0.0, 8.0, 15.0),
            material: materials.add(StandardMaterial {
                base_color: Color::YELLOW,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        },
    ))
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(1.0, 1.0, 1.0))
    .insert(collision_groups3);
*/

    //移動アーム1
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 20.0, 0.5))),
            transform: Transform::from_xyz(0.0, 7.0, 10.0),
            material: materials.add(StandardMaterial {
                base_color: Color::ORANGE_RED,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        },
        AnimateArm1 {angle: -0.1, flag: 0},
    ))
    .with_children(|parent| {

        //移動アーム2
        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 5.0, 1.0))),
                transform: Transform::from_xyz(0.0, 10.0, 0.0),
                material: materials.add(StandardMaterial {
                    base_color: Color::ORANGE_RED,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            },
            AnimateArm2 {angle: -0.1, flag: 0},
        )).with_children(|parent| {

            //移動アーム3
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(8.0, 1.0, 4.0))),
                    transform: Transform::from_xyz(0.0, -2.0, 2.0),
                    material: materials.add(StandardMaterial {
                        base_color: Color::ORANGE_RED,
                        perceptual_roughness: 1.0,
                        ..default()
                    }),
                    ..default()
                },
                AnimateArm3 {angle: -0.1, flag: 0},
            ))
            //.insert(RigidBody::Dynamic)
            //.insert(Dominance::group(10))
            .insert(Collider::cuboid(4.0, 0.5, 2.0))
            .with_children(|parent| {
                parent.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box::new(0.1, 4.0, 4.0))),
                        transform: Transform::from_xyz(4.0, -2.0, 0.0),
                        material: materials.add(StandardMaterial {
                            base_color: Color::ORANGE_RED,
                            perceptual_roughness: 1.0,
                            ..default()
                        }),
                        ..default()
                    },
                ))
                //.insert(RigidBody::Dynamic)
                //.insert(Dominance::group(10))
                .insert(Collider::cuboid(0.05, 2.0, 2.0))
                .insert(Friction {  //摩擦
                    coefficient: 1.0,   //摩擦係数
                    combine_rule: CoefficientCombineRule::Max,
                })
                .insert(Restitution {   //反発
                    coefficient: 0.0,   //反発係数
                    combine_rule: CoefficientCombineRule::Max,
                });

                parent.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box::new(0.2, 0.1, 4.0))),
                        transform: Transform::from_xyz(3.9, -4.0, 0.0),
                        material: materials.add(StandardMaterial {
                            base_color: Color::ORANGE_RED,
                            perceptual_roughness: 1.0,
                            ..default()
                        }),
                        ..default()
                    },
                ))
                //.insert(RigidBody::Dynamic)
                //.insert(Dominance::group(10))
                .insert(Collider::cuboid(0.1, 0.05, 2.0))
                .insert(Friction {  //摩擦
                    coefficient: 1.0,   //摩擦係数
                    combine_rule: CoefficientCombineRule::Max,
                })
                .insert(Restitution {   //反発
                    coefficient: 0.0,   //反発係数
                    combine_rule: CoefficientCombineRule::Max,
                });
            });
        });

    })
    //.insert(RigidBody::Dynamic)
    //.insert(Dominance::group(10))
    .insert(Collider::cuboid(0.25, 5.0, 0.25))
    .insert(CollisionGroups::new(Group::from_bits_truncate(0b0100), Group::from_bits_truncate(0b0001)))
    .insert(AnimateTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));

    //壁
    for i in 0..20 {
        let i = i as f32;
        if i == 5.0 {
            commands
            .spawn(Collider::cuboid(4.0, 5.0, 0.25))
            .insert(TransformBundle::from(
                Transform::from_xyz(20.0*(i*PI/10.0).cos(), -0.5, 20.0*(i*PI/10.0).sin())
                .with_rotation(Quat::from_rotation_y(-1.0*(PI/2.0 + i*PI/10.0)))))
                .insert(Ccd::enabled());
        } else {

            commands
            .spawn(Collider::cuboid(4.0, 50.0, 0.25))
            .insert(TransformBundle::from(
                Transform::from_xyz(20.0*(i*PI/10.0).cos(), -0.5, 20.0*(i*PI/10.0).sin())
                .with_rotation(Quat::from_rotation_y(-1.0*(PI/2.0 + i*PI/10.0)))))
                .insert(Ccd::enabled());
        }
        //println!("{}, {}, {}, {} ", i, 20.0*(i*PI/10.0).cos(), 20.0*(i*PI/10.0).sin(), -1.0*(PI/2.0 + i*PI/10.0));
    }

    
    //動いてるアイテム
    for i in 0..200 {
        let i = i as f32;
        commands
            .spawn((
                PbrBundle {
                    //mesh: meshes.add(Mesh::from(shape::Cube::new(2.0))),
                    mesh: meshes.add(Mesh::from(shape::UVSphere {
                        radius: 0.4,
                        ..default()
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::PURPLE,
                        perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
                },
                AnimateItem
            ))
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(0.4))
            .insert(Restitution::coefficient(0.7))
            .insert(TransformBundle::from(Transform::from_xyz(15.0*(i*PI/10.0).cos(), 2.0 + 0.4*i*0.1, 15.0*(i*PI/10.0).sin())))
            .insert(Friction {  //摩擦
                coefficient: 1.0,   //摩擦係数
                combine_rule: CoefficientCombineRule::Max,
            })
            .insert(Restitution {   //反発
                coefficient: 0.0,   //反発係数
                combine_rule: CoefficientCombineRule::Max,
            })
            .insert(Dominance::group(-10));
    }
    

    //動いてるアイテム
    for i in 0..300 {
        let i = i as f32;
        commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
                    material: materials.add(StandardMaterial {
                        base_color: Color::PURPLE,
                        perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
                },
                AnimateItem
            ))
            .insert(RigidBody::Dynamic)
            .insert(Ccd::enabled())
            .insert(Collider::cuboid(0.5, 0.5, 0.5))
            .insert(Restitution::coefficient(0.7))
            .insert(TransformBundle::from(Transform::from_xyz(17.0*(i*PI/10.0).cos(), 2.0 + 0.4*i*0.1, 17.0*(i*PI/10.0).sin())))
            .insert(Friction {  //摩擦
                coefficient: 1.0,   //摩擦係数
                combine_rule: CoefficientCombineRule::Max,
            })
            .insert(Restitution {   //反発
                coefficient: 0.0,   //反発係数
                combine_rule: CoefficientCombineRule::Max,
            })
            .insert(Dominance::group(-10));
    }

    for i in 0..20 {
        let i = i as f32;
        
        commands.spawn(SceneBundle {
            scene: asset_server.load("models/cafe.gltf#Scene0"),
            transform: Transform::from_xyz(15.0, 6.0 + i*2.0, -5.0 + i)
                    * Transform::default().with_scale(Vec3::new(0.1, 0.1, 0.1))
                    //* Transform::from_rotation(Quat::from_rotation_x(PI / 2.0))
                    //* Transform::from_rotation(Quat::from_rotation_z(PI / -2.0))
                    ,
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Ccd::enabled())
        .insert(Collider::cuboid(1.0, 1.0, 1.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Friction {  //摩擦
            coefficient: 1.0,   //摩擦係数
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(Restitution {   //反発
            coefficient: 0.0,   //反発係数
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(Dominance::group(-10));
    }

    for i in 0..20 {
        let i = i as f32;
        
        commands.spawn(SceneBundle {
            scene: asset_server.load("models/pc.gltf#Scene0"),
            transform: Transform::from_xyz(-15.0, 6.0 + i*2.0, -5.0 + i)
                    * Transform::default().with_scale(Vec3::new(0.1, 0.1, 0.1))
                    //* Transform::from_rotation(Quat::from_rotation_x(PI / 2.0))
                    //* Transform::from_rotation(Quat::from_rotation_z(PI / -2.0))
                    ,
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Ccd::enabled())
        .insert(Collider::cuboid(1.0, 1.0, 1.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Friction {  //摩擦
            coefficient: 1.0,   //摩擦係数
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(Restitution {   //反発
            coefficient: 0.0,   //反発係数
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(Dominance::group(-10));
    }

    //ボタン台
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(12.0, 2.0, 5.0))),
            transform: Transform::from_xyz(-1.0, 3.5, 31.0),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        },
    ))
    .with_children(|parent| {

        //アーム移動用ボタン(1)
        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: 1.5,
                    height: 2.0,
                    resolution: 32,
                    segments: 32,
                })),
                transform: Transform::from_xyz(-2.0, 2.0, -2.0),
                material: materials.add(StandardMaterial {
                    base_color: Color::GOLD,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            },
        )).with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cylinder {
                        radius: 1.3,
                        height: 1.5,
                        resolution: 32,
                        segments: 32,
                    })),
                    transform: Transform::from_xyz(0.0, 1.5, 0.0),
                    material: materials.add(StandardMaterial {
                        base_color: Color::BLUE,
                        perceptual_roughness: 1.0,
                        ..default()
                    }),
                    ..default()
                },
                SelectButton1 {flag: true, act: 0},
            ))
            .insert(PickableBundle::default());
        });
    
        //アーム移動用ボタン(2)
        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: 1.5,
                    height: 2.0,
                    resolution: 32,
                    segments: 32,
                })),
                transform: Transform::from_xyz(2.0, 2.0, -2.0),
                material: materials.add(StandardMaterial {
                    base_color: Color::GOLD,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            },
        )).with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cylinder {
                        radius: 1.3,
                        height: 1.5,
                        resolution: 32,
                        segments: 32,
                    })),
                    transform: Transform::from_xyz(0.0, 1.5, 0.0),
                    material: materials.add(StandardMaterial {
                        base_color: Color::BLUE,
                        perceptual_roughness: 1.0,
                        ..default()
                    }),
                    ..default()
                },
                SelectButton2 {flag: false, act: 0},
            ))
            .insert(PickableBundle::default());
        });

        //説明文1
        parent.spawn(SceneBundle {
            scene: asset_server.load("models/1_movearm.gltf#Scene0"),
            transform: Transform::default().with_scale(Vec3::new(0.2, 0.2, 0.2))
                        * Transform::from_xyz(0.0, 6.0, 3.0),
            ..default()
        });

        //説明文2
        parent.spawn(SceneBundle {
            scene: asset_server.load("models/2_releasecargo.gltf#Scene0"),
            transform: Transform::default().with_scale(Vec3::new(0.2, 0.2, 0.2))
                        * Transform::from_xyz(0.0, 6.0, 9.0),
            ..default()
        });

        //1ボタンアイコン
        parent.spawn(SceneBundle {
            scene: asset_server.load("models/button1.gltf#Scene0"),
            transform: Transform::default().with_scale(Vec3::new(0.3, 0.3, 0.3))
                        * Transform::from_xyz(4.0, 7.0, -1.5) 
                        * Transform::from_rotation(Quat::from_rotation_x(PI / 2.0))
                        //* Transform::from_rotation(Quat::from_rotation_z(PI / -2.0))
                        ,
            ..default()
        });

        //2ボタンアイコン
        parent.spawn(SceneBundle {
            scene: asset_server.load("models/button2.gltf#Scene0"),
            transform: Transform::default().with_scale(Vec3::new(0.3, 0.3, 0.3))
                        * Transform::from_xyz(17.0, 7.0, -1.5) 
                        * Transform::from_rotation(Quat::from_rotation_x(PI / 2.0))
                        //* Transform::from_rotation(Quat::from_rotation_z(PI / -2.0))
                        ,
            ..default()
        });
    });



    //獲得品箱
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 1.0, 10.0))),
            transform: Transform::from_xyz(-30.0, 1.0, 5.0) * Transform::from_rotation(Quat::from_rotation_y(PI / -4.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        },
    ))
    .insert(Collider::cuboid(5.0, 0.5, 5.0))
    .with_children(|parent| {

        parent.spawn(SceneBundle {
            scene: asset_server.load("models/get_items.gltf#Scene0"),
            transform: Transform::default().with_scale(Vec3::new(0.3, 0.3, 0.3))
                        * Transform::from_xyz(20.0, 20.0, 3.0) 
                        * Transform::from_rotation(Quat::from_rotation_x(PI / 2.0))
                        * Transform::from_rotation(Quat::from_rotation_z(PI / -2.0)),
            ..default()
        });

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 10.0, 1.0))),
                transform: Transform::from_xyz(0.0, 5.0, -5.0),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            },
        ))
        .insert(Collider::cuboid(5.0, 5.0, 0.5));

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 10.0, 10.0))),
                transform: Transform::from_xyz(-5.0, 5.0, 0.0),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            },
        ))
        .insert(Collider::cuboid(0.5, 5.0, 5.0));

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 10.0, 1.0))),
                transform: Transform::from_xyz(0.0, 5.0, 5.0),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            },
        ))
        .insert(Collider::cuboid(5.0, 5.0, 0.5));

        parent.spawn(Collider::cuboid(0.5, 5.0, 5.0))
        .insert(TransformBundle::from(Transform::from_xyz(5.0, 5.0, 0.0)));
    });




    //カメラ座標の表示
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Camera Altitude: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ]),
        CameraText,
    ));


    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "  Get : ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ]),
        PointText { point: 0 },
    ));



    
}


//カメラ移動
fn move_camera(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    //entity(componentの集合体)を一つだけ取得
    let mut camera = camera.single_mut();

    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::Up) {
        direction.z -= 1.0;
    }
    if input.pressed(KeyCode::Down) {
        direction.z += 1.0;
    }
    if input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::O) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::P) {
        direction.y -= 1.0;
    }

    camera.translation += time.delta_seconds() * 10.0 * direction;

}

//床の回転
fn animate_tile(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<AnimateTile>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * -0.25);
    }
}

//プッシャーの移動
fn animate_pusher(
    mut query: Query<(&mut Transform, &mut AnimatePusher), With<AnimatePusher>>,
) {
    for (mut transform, mut animate_pusher) in &mut query {
        transform.translation.z = 10.0  + 3.0 * (animate_pusher.rotate*PI).sin();
        animate_pusher.rotate += 0.01;
    }
}

//カメラ情報表示
fn update_camera_text(
    mut text: Query<&mut Text, With<CameraText>>,
    mut camera: Query<&Transform, With<Camera>>,
) {
    for mut text in &mut text {
        let camera = camera.single_mut();
        text.sections[1].value = format!("x:{}, y:{}, z:{}", camera.translation.x,camera.translation.y,camera.translation.z);
    }
}


fn select_button1(
    mut events: EventReader<PickingEvent>,
    mut query: Query<&mut SelectButton1, With<SelectButton1>>,
) {
    for ev in events.iter() {
        //println!("{:?}", ev);
        if let PickingEvent::Clicked(info) = ev {
            //println!("{:?}", query);
            if let Ok(_transform) = query.get(*info) {
                //println!("{:?}", transform);
                //println!("{:?}", transform.translation.y);

                for mut selectbutton1 in &mut query {
                    selectbutton1.act = 1;
                }
            }
        }
    }
}

fn select_button2(
    mut events: EventReader<PickingEvent>,
    mut query: Query<&mut SelectButton2, With<SelectButton2>>,
) {
    for ev in events.iter() {
        //println!("{:?}", ev);
        if let PickingEvent::Clicked(info) = ev {
            //println!("{:?}", query);
            if let Ok(_transform) = query.get(*info) {
                //println!("{:?}", transform);
                //println!("{:?}", transform.translation.y);

                for mut selectbutton2 in &mut query {
                    selectbutton2.act = 1;
                }
            }
        }
    }
}

fn move_button1(
    mut query: Query<(&mut SelectButton1, &mut Transform), With<SelectButton1>>,
) {

    for (mut selectbutton1, mut transform) in &mut query {
        match selectbutton1.act {
            1 => {
                transform.translation.y -= 0.05;
                if transform.translation.y < 1.1 {
                    selectbutton1.act = 2;
                }
            },
            2 => {
                transform.translation.y += 0.05;
                if transform.translation.y == 1.5 {
                    selectbutton1.act = 0;
                }
            },
            _ => { }
        }
    }         
}

fn move_button2(
    mut query: Query<(&mut SelectButton2, &mut Transform), With<SelectButton2>>,
) {

    for (mut selectbutton2, mut transform) in &mut query {
        match selectbutton2.act {
            1 => {
                transform.translation.y -= 0.05;
                if transform.translation.y < 1.1 {
                    selectbutton2.act = 2;
                }
            },
            2 => {
                transform.translation.y += 0.05;
                if transform.translation.y == 1.5 {
                    selectbutton2.act = 0;
                }
            },
            _ => { }
        }
    }         
}

fn action_button1(
    mut query: Query<(&mut SelectButton1, &mut Transform), (With<SelectButton1>, Changed<Transform>)>,
    mut armquery1: Query<&mut AnimateArm1, (With<AnimateArm1>, Without<SelectButton1>)>,
    mut armquery2: Query<&mut AnimateArm2, (With<AnimateArm2>, Without<SelectButton1>)>,
) {
    for (mut selectbutton1, _) in query.iter_mut() {
        if selectbutton1.flag {
            for mut animate_arm1 in &mut armquery1 {
                if animate_arm1.angle >= 0.0 {
                    animate_arm1.angle = 360.0;
                    selectbutton1.flag = false;
                    animate_arm1.flag = 1;
                } else {
                    animate_arm1.angle = 0.0;
                }
            }
            
            for mut animate_arm2 in &mut armquery2 {
                if animate_arm2.angle >= 0.0 {
                    animate_arm2.angle = 0.0;
                    animate_arm2.flag = 1;
                } else {
                    animate_arm2.angle = 0.0;
                }
            }
        }
    }
}

fn action_button2(
    mut query: Query<(&mut SelectButton2, &mut Transform), (With<SelectButton2>, Changed<Transform>)>,
    mut armquery3: Query<&mut AnimateArm3, (With<AnimateArm3>, Without<SelectButton2>)>,
) {
    for (mut selectbutton2, _) in query.iter_mut() {
        if selectbutton2.flag {
            for mut animate_arm3 in &mut armquery3 {
                if animate_arm3.angle >= 0.0 {
                    animate_arm3.angle = 270.0;
                    selectbutton2.flag = false;
                    animate_arm3.flag = 2;
                } else {
                    animate_arm3.angle = 0.0;
                }
            }
        }
    }
}


fn rotate_arm1(
    mut armquery1: Query<(&mut AnimateArm1, &mut Transform), With<AnimateArm1>>,
    mut armquery3: Query<&mut AnimateArm3, With<AnimateArm3>>,
    mut button2: Query<&mut SelectButton2, (With<SelectButton2>, Without<AnimateArm1>)>,
    time: Res<Time>,
) {
    for (mut animate_arm1, mut transform) in &mut armquery1 {
        for mut animate_arm3 in &mut armquery3 {
            match animate_arm1.flag {
                1 => {
                    animate_arm1.angle -= ARM_SPEED * time.delta_seconds();
                    if animate_arm1.angle <= 270.0 {
                        animate_arm1.angle = 270.0;
                        animate_arm1.flag = 0;

                        animate_arm3.angle = 360.0;
                        animate_arm3.flag = 1;
                    }
                },
                2 => {
                    animate_arm1.angle += ARM_SPEED * time.delta_seconds();
                    if animate_arm1.angle >= 360.0 {
                        animate_arm1.angle = 360.0;
                        animate_arm1.flag = 0;

                        for mut selectbutton2 in &mut button2 {
                            selectbutton2.flag = true;
                        }
                    }
                },
                _ => {}
            }
            transform.rotation = Quat::from_rotation_z(animate_arm1.angle.to_radians());
        }
    }
}

fn rotate_arm2(
    mut armquery: Query<(&mut AnimateArm2, &mut Transform), With<AnimateArm2>>,
    time: Res<Time>,
) {
    for (mut animate_arm2, mut transform) in &mut armquery {
        match animate_arm2.flag {
            1 => {
                animate_arm2.angle += ARM_SPEED * time.delta_seconds();
                if animate_arm2.angle >= 90.0 {
                    animate_arm2.angle = 90.0;
                    animate_arm2.flag = 0;
                }
            },
            2 => {
                animate_arm2.angle -= ARM_SPEED * time.delta_seconds();
                if animate_arm2.angle <= 0.0 {
                    animate_arm2.angle = 0.0;
                    animate_arm2.flag = 0;
                }
            },
            _ => {}
        }
        transform.rotation = Quat::from_rotation_z(animate_arm2.angle.to_radians());
    }
}

fn rotate_arm3(
    mut armquery3: Query<(&mut AnimateArm3, &mut Transform), With<AnimateArm3>>,
    mut armquery1: Query<&mut AnimateArm1, With<AnimateArm1>>,
    mut armquery2: Query<&mut AnimateArm2, With<AnimateArm2>>,
    mut button1: Query<&mut SelectButton1, (With<SelectButton1>, Without<AnimateArm3>)>,
    time: Res<Time>,
) {
    for (mut animate_arm3, mut transform) in &mut armquery3 {
        for mut animate_arm1 in &mut armquery1 {
            for mut animate_arm2 in &mut armquery2 {
                match animate_arm3.flag {
                    1 => {
                        animate_arm3.angle -= ARM_SPEED * time.delta_seconds();
                        if animate_arm3.angle <= 270.0 {
                            animate_arm3.angle = 270.0;
                            animate_arm3.flag = 0;

                            animate_arm1.angle = 270.0;
                            animate_arm1.flag = 2;
                            animate_arm2.angle = 90.0;
                            animate_arm2.flag = 2;
                        }
                    },
                    2 => {
                        animate_arm3.angle += ARM_SPEED * time.delta_seconds();
                        if animate_arm3.angle >= 360.0 {
                            animate_arm3.angle = 360.0;
                            animate_arm3.flag = 0;

                            for mut selectbutton1 in &mut button1 {
                                selectbutton1.flag = true;
                            }
                        }
                    },
                    _ => {}
                }
                transform.rotation = Quat::from_rotation_z(animate_arm3.angle.to_radians());
            }
        }
    }
}

fn reciever_event(
    mut commands: Commands,
    mut text: Query<(&mut PointText, &mut Text), With<PointText>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(_data1, data2, _) => {
                commands.entity(*data2)
                .insert(TransformBundle::from(
                    Transform::from_xyz(-30.0, 10.0, 5.0)));
                
                for (mut pointtext, mut text) in &mut text {
                    pointtext.point += 1;
                    text.sections[1].value = format!("{} items", pointtext.point);
                }

            }
            CollisionEvent::Stopped(_, _, _) => { }
        }
    }

}
