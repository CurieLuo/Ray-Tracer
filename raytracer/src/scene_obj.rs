use crate::{
    aarect::*, bvh::*, hittable::*, hittable_list::*, material::*, medium::*, obj_file::*,
    rect_box::*, scene::*, sphere::*, texture::*, transform::*, utility::*,
};

pub fn aircraft() -> Arc<dyn Hittable> {
    let material = Lambertian::new_texture(ImageTexture::new("image/E-45-Aircraft/E-45_col.jpg"));
    Arc::new(Translate::new(
        RotateY::new(
            BvhNode::new(
                &load_obj_and_normal(
                    "object/E-45-Aircraft.obj",
                    "image/E-45-Aircraft/E-45-normal.jpg",
                    material,
                    50.0,
                ),
                0.,
                1.,
            ),
            70.,
        ),
        Vec3::new(-250., 100., 200.),
    ))
}

pub fn iron_man() -> Arc<dyn Hittable> {
    let material =
        Lambertian::new_texture(ImageTexture::new("image/Iron_Man/Iron_Man_Diffuse.png"));
    Arc::new(Translate::new(
        RotateX::new(
            RotateY::new(
                BvhNode::new(
                    &load_obj_and_normal(
                        "object/IronMan.obj",
                        "image/Iron_Man/Iron_Man_Normal.png",
                        material,
                        80.0,
                    ),
                    0.,
                    1.,
                ),
                -30.,
            ),
            -5.,
        ),
        Vec3::new(350., -190., 100.),
    ))
}

pub fn book() -> Arc<dyn Hittable> {
    let material = Lambertian::new_texture(ImageTexture::new("image/book/baseColor.png"));
    Arc::new(Translate::new(
        RotateY::new(
            RotateZ::new(
                RotateX::new(
                    BvhNode::new(
                        &load_obj_and_normal(
                            "object/book.obj",
                            "image/book/normal.png",
                            material,
                            30.0,
                        ),
                        0.,
                        1.,
                    ),
                    90.,
                ),
                20.,
            ),
            45.,
        ),
        Vec3::new(70., 180., 100.),
        // Vec3::new(70. / 7., 180. / 9., 100.),
    ))
}

pub fn sword() -> Arc<dyn Hittable> {
    let material =
        Lambertian::new_texture(ImageTexture::new("image/Sting-Sword/Sting_Base_Color.png"));
    Arc::new(Translate::new(
        RotateZ::new(
            RotateY::new(
                RotateX::new(
                    BvhNode::new(
                        &load_obj_and_normal(
                            "object/Sting-Sword.obj",
                            "image/Sting-Sword/Sting_Normal.png",
                            material,
                            20.0,
                        ),
                        0.,
                        1.,
                    ),
                    90.,
                ),
                40.,
            ),
            90.,
        ),
        Vec3::new(-250., -100., -600.),
    ))
}

pub fn _stone_man() -> Arc<dyn Hittable> {
    let material = Lambertian::new_texture(ImageTexture::new("image/StoneMan/diffuse.tif"));
    Arc::new(Translate::new(
        RotateX::new(
            RotateY::new(
                BvhNode::new(
                    &load_obj_and_normal(
                        "object/StoneMan.obj",
                        "image/StoneMan/normal.png",
                        material,
                        50.0,
                    ),
                    0.,
                    1.,
                ),
                -30.,
            ),
            -5.,
        ),
        Vec3::new(350., -230., 100.),
    ))
}

pub fn test1() -> HittableList {
    let mut world = HittableList::new();

    world.add(aircraft());
    world.add(iron_man());
    world.add(book());
    world.add(sword());

    world
}
