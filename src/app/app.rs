use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, TextureCreator},
    video::WindowContext,
};

use crate::{
    colorbuffer::colorbuffer::ColorBuffer,
    game::game::Game,
    player::player::Player,
    ray::ray::Ray,
    timekeeper::timekeeper::TimeKeeper,
    utils::{numbers::wrapping_sub_float, points::distance_between_points},
    window::window::{
        DISTANCE_PROJ_PLANE, MINIMAP_SCALING, NUM_COLS, NUM_RAYS, NUM_ROWS, TILE_SIZE,
        WINDOW_HEIGHT, WINDOW_WIDTH,
    },
};

pub struct App<'a> {
    pub game: Game,
    pub player: Player,
    pub sdl_context: &'a sdl2::Sdl,
    pub video_subsystem: &'a sdl2::VideoSubsystem,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub color_buffer: ColorBuffer<'a>,
    pub canvas: &'a mut Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    pub timekeeper: TimeKeeper,
    pub is_running: bool,
}

impl<'a> App<'a> {
    pub fn new(
        sdl_context: &'a sdl2::Sdl,
        video_subsystem: &'a sdl2::VideoSubsystem,
        canvas: &'a mut Canvas<sdl2::video::Window>,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, String> {
        let event_pump = sdl_context.event_pump()?;
        let color_buffer = ColorBuffer::new(&texture_creator)?;

        Ok(App {
            game: Game::default(),
            player: Player::default(),
            sdl_context,
            video_subsystem,
            canvas,
            texture_creator,
            color_buffer,
            event_pump,
            timekeeper: TimeKeeper::default(),
            is_running: true,
        })
    }

    pub fn generate3d_projection(&mut self) -> Result<(), String> {
        for x in 0..NUM_RAYS {
            let perp_dist_wrapped = self.calculate_perpendicular_distance(x)?;

            let proj_wall_height = self.calculate_projected_wall_height(perp_dist_wrapped);

            let (top_wall_pixel, bottom_wall_pixel) = self.calculate_wall_pixels(proj_wall_height);

            self.render_ceiling(x, top_wall_pixel);
            self.render_wall(x, top_wall_pixel, bottom_wall_pixel);
            self.render_floor(x, bottom_wall_pixel);
        }

        self.update_texture()
    }

    pub fn render_color_buffer(&mut self) -> Result<(), String> {
        let color_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                self.color_buffer.color.as_ptr() as *const u8,
                self.color_buffer.color.len() * 4,
            )
        };

        self.color_buffer
            .texture
            .update(None, color_bytes, WINDOW_WIDTH as usize * 4)
            .map_err(|err| format!("Error updating texture: {:?}", err))?;

        self.canvas
            .copy(&self.color_buffer.texture, None, None)
            .map_err(|err| format!("Error copying texture to canvas: {:?}", err))?;

        Ok(())
    }

    pub fn render_player(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));

        let player_rect = Rect::new(
            (self.player.x * MINIMAP_SCALING) as i32,
            (self.player.y * MINIMAP_SCALING) as i32,
            (self.player.width * MINIMAP_SCALING) as u32,
            (self.player.height * MINIMAP_SCALING) as u32,
        );
        self.canvas.fill_rect(player_rect)?;

        let length = 30.0;
        let line_end_x = (MINIMAP_SCALING * self.player.x) as i32
            + (length * self.player.rotation_angle.cos()) as i32;
        let line_end_y = (MINIMAP_SCALING * self.player.y) as i32
            + (length * self.player.rotation_angle.sin()) as i32;

        let start_point = Point::new(
            (MINIMAP_SCALING * self.player.x) as i32,
            (MINIMAP_SCALING * self.player.y) as i32,
        );
        let end_point = Point::new(line_end_x, line_end_y);
        self.canvas.draw_line(start_point, end_point)?;

        Ok(())
    }

    pub fn render_map(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        self.canvas.fill_rect(Rect::new(
            0,
            0,
            (MINIMAP_SCALING * NUM_COLS as f64 * TILE_SIZE as f64) as u32,
            (MINIMAP_SCALING * NUM_ROWS as f64 * TILE_SIZE as f64) as u32,
        ))?;

        for i in 0..NUM_ROWS {
            for j in 0..NUM_COLS {
                let x_tile = j * TILE_SIZE;
                let y_tile = i * TILE_SIZE;

                let tile_color = if self.game.game_map[i as usize][j as usize] != 0 {
                    255
                } else {
                    0
                };

                self.canvas
                    .set_draw_color(Color::RGBA(tile_color, tile_color, tile_color, 255));
                let map_tile = Rect::new(
                    (x_tile as f64 * MINIMAP_SCALING) as i32,
                    (y_tile as f64 * MINIMAP_SCALING) as i32,
                    (TILE_SIZE as f64 * MINIMAP_SCALING) as u32,
                    (TILE_SIZE as f64 * MINIMAP_SCALING) as u32,
                );
                self.canvas
                    .fill_rect(map_tile)
                    .map_err(|err| format!("Error filling rect {:?}", err))?;
            }
        }
        Ok(())
    }

    pub fn render_rays(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));

        for i in 0..NUM_RAYS {
            let ray_start = Point::new(
                (MINIMAP_SCALING * self.player.x) as i32,
                (MINIMAP_SCALING * self.player.y) as i32,
            );
            let ray_end = Point::new(
                (MINIMAP_SCALING * self.game.rays[i as usize].x_collision) as i32,
                (MINIMAP_SCALING * self.game.rays[i as usize].y_collision) as i32,
            );
            self.canvas.draw_line(ray_start, ray_end)?;
        }

        Ok(())
    }

    fn cast_ray(&mut self, angle: f64, ray_id: usize) {
        self.game.rays[ray_id] = Ray::new(angle);

        let ray = &self.game.rays[ray_id];

        let h = self.calculate_horizontal_intersection(ray);
        let v = self.calculate_vertical_intersection(ray);

        let ray = &mut self.game.rays[ray_id];

        let horz_collision_dist = if h.found_horz_collision {
            distance_between_points(
                self.player.x,
                self.player.y,
                h.horz_x_wall_collision,
                h.horz_y_wall_collision,
            )
        } else {
            f64::MAX
        };

        let vert_collision_dist = if v.found_vert_collision {
            distance_between_points(
                self.player.x,
                self.player.y,
                v.vert_x_wall_collision,
                v.vert_y_wall_collision,
            )
        } else {
            f64::MAX
        };

        if vert_collision_dist < horz_collision_dist {
            ray.distance = vert_collision_dist;
            ray.x_collision = v.vert_x_wall_collision;
            ray.y_collision = v.vert_y_wall_collision;
            ray.content = v.vert_wall_content;
            ray.is_vertical_collision = true;
        } else {
            ray.distance = horz_collision_dist;
            ray.x_collision = h.horz_x_wall_collision;
            ray.y_collision = h.horz_y_wall_collision;
            ray.content = h.horz_wall_content;
            ray.is_vertical_collision = false;
        }
    }

    pub fn cast_rays(&mut self) {
        for col in 0..NUM_RAYS as usize {
            let angle = self.player.rotation_angle
                + (col as f64 - (NUM_RAYS as f64) / 2.0).atan2(*DISTANCE_PROJ_PLANE as f64);
            self.cast_ray(angle, col);
        }
    }

    fn calculate_perpendicular_distance(&self, x: u32) -> Result<f64, String> {
        let angle_cos = self.game.rays[x as usize].angle.cos();
        let perp_dist_wrapped = wrapping_sub_float(
            self.game.rays[x as usize].distance * angle_cos,
            self.player.rotation_angle,
        );
        Ok(perp_dist_wrapped)
    }

    fn calculate_projected_wall_height(&self, perp_dist_wrapped: f64) -> i32 {
        ((TILE_SIZE as f64 / perp_dist_wrapped) * *DISTANCE_PROJ_PLANE) as i32
    }

    fn calculate_wall_pixels(&self, proj_wall_height: i32) -> (u32, u32) {
        let half_window_height = WINDOW_HEIGHT / 2;

        let top_wall_pixel = half_window_height
            .checked_sub((proj_wall_height / 2) as u32)
            .unwrap_or(0);
        let bottom_wall_pixel = half_window_height
            .checked_add((proj_wall_height / 2) as u32)
            .unwrap_or(WINDOW_HEIGHT);

        (top_wall_pixel, bottom_wall_pixel)
    }

    fn render_ceiling(&mut self, x: u32, top_wall_pixel: u32) {
        for y in 0..top_wall_pixel {
            self.color_buffer.color[((WINDOW_WIDTH * y) + x) as usize] = 0xFF444444;
        }
    }

    fn render_wall(&mut self, x: u32, top_wall_pixel: u32, bottom_wall_pixel: u32) {
        for y in top_wall_pixel..bottom_wall_pixel {
            let index = (WINDOW_WIDTH as usize)
                .checked_mul(y as usize)
                .and_then(|mul_result| mul_result.checked_add(x as usize));

            if let Some(index) = index {
                let index_clamped = index.min(self.color_buffer.color.len() - 1);

                let color = if self.game.rays[x as usize].is_vertical_collision {
                    0xFFFFFFFF
                } else {
                    0xFFCCCCCC
                };

                self.color_buffer.color[index_clamped] = color;
            } else {
                println!("Overflow in index calculation: x={}, y={}", x, y);
            }
        }
    }

    // fn render_wall(&mut self, x: u32, top_wall_pixel: u32, bottom_wall_pixel: u32) {
    //     for y in top_wall_pixel..bottom_wall_pixel {
    //         let color = if self.game.rays[x as usize].is_vertical_collision {
    //             0xFFFFFFFF
    //         } else {
    //             0xFFCCCCCC
    //         };
    //         self.color_buffer.color[((WINDOW_WIDTH * y) + x) as usize] = color;
    //     }
    // }

    fn render_floor(&mut self, x: u32, bottom_wall_pixel: u32) {
        for y in bottom_wall_pixel..WINDOW_HEIGHT {
            self.color_buffer.color[((WINDOW_WIDTH * y) + x) as usize] = 0xFF777777;
        }
    }

    fn update_texture(&mut self) -> Result<(), String> {
        let color_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                self.color_buffer.color.as_ptr() as *const u8,
                self.color_buffer.color.len() * 4,
            )
        };

        match self
            .color_buffer
            .texture
            .update(None, color_bytes, WINDOW_WIDTH as usize * 4)
        {
            Ok(()) => Ok(()),
            Err(err) => Err(format!("Error updating textures: {:?}", err)),
        }
    }
}
